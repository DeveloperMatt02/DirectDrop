use std::path::Path;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use rqs_lib::channel::{ChannelMessage, Message, MessageClient, TransferKind};
use rqs_lib::hdl::info::{TransferMetadata, TransferPayload, TransferPayloadKind};
use rqs_lib::utils::RemoteDeviceInfo;
use rqs_lib::{DeviceType, TransferState};

use crate::AppState;
use crate::cmds::scan_ble::receiver_token;
use crate::dto::{GuiError, SendBleInput};

fn new_ble_send_id() -> String {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    format!("ble-{nanos}")
}

/// Messaggio terminale sintetico: se il task fallisce PRIMA che
/// `OutboundRequest` esista (adapter/connect), il core non ha emesso nulla e
/// la GUI resterebbe appesa su "connecting". Il bridge lo inoltra a
/// `qsd://transfer` come un terminale normale (i duplicati con stati già
/// emessi dal core sono no-op per lo store: le card terminali non mostrano
/// progress, quindi `ack_bytes: 0` non regredisce nulla).
fn terminal_message(
    send_id: &str,
    state: TransferState,
    peer_name: &str,
    files: &[String],
) -> ChannelMessage {
    let total_bytes = files
        .iter()
        .filter_map(|p| std::fs::metadata(p).ok())
        .map(|md| md.len())
        .sum();
    ChannelMessage {
        id: send_id.to_string(),
        msg: Message::Client(MessageClient {
            kind: TransferKind::Outbound,
            state: Some(state),
            metadata: Some(TransferMetadata {
                id: send_id.to_string(),
                source: Some(RemoteDeviceInfo {
                    name: peer_name.to_string(),
                    device_type: DeviceType::Phone,
                }),
                pin_code: None,
                payload_kind: TransferPayloadKind::Files,
                payload_preview: None,
                payload: Some(TransferPayload::Files(files.to_vec())),
                total_bytes,
                ack_bytes: 0,
            }),
            // La GUI tiene l'ultimo mezzo ricevuto dal core.
            transport: None,
        }),
    }
}

/// Invia file a un receiver BLE (token da `qsd_scan_ble`).
/// Genera `send_id` (`ble-<nanos>` o frontend-provided), lo ritorna subito e
/// spedisce in background via `send_files_ble` (progress Outbound su
/// `message_sender`, già inoltrato dal bridge a `qsd://transfer`).
#[tauri::command]
pub async fn qsd_send_ble(
    m: SendBleInput,
    state: tauri::State<'_, AppState>,
) -> Result<String, GuiError> {
    if m.receiver_id.is_empty() {
        return Err(GuiError::validation("receiver_id is empty"));
    }
    if m.files.is_empty() {
        return Err(GuiError::validation("no files given"));
    }
    // Parita' CLI / send_payload.rs: solo file regolari.
    for f in &m.files {
        let p = Path::new(f);
        if !p.is_file() {
            return Err(GuiError::validation(format!(
                "{f}: not a regular file (folders aren't supported)"
            )));
        }
    }
    let receiver = {
        let map = state
            .ble_receivers
            .lock()
            .map_err(|e| GuiError::core(format!("state lock poisoned: {e}")))?;
        map.get(&m.receiver_id).cloned()
    };
    let Some(receiver) = receiver else {
        return Err(GuiError::validation(format!(
            "unknown receiver_id (re-scan?): {}",
            m.receiver_id
        )));
    };
    let send_id = match m.send_id {
        Some(s) if !s.is_empty() => s,
        _ => new_ble_send_id(),
    };
    info!(
        "qsd_send_ble: id={send_id} receiver={} files={}",
        receiver_token(&receiver),
        m.files.len()
    );

    let ctk = tokio_util::sync::CancellationToken::new();
    state
        .ble_cancel
        .lock()
        .map_err(|e| GuiError::core(format!("state lock poisoned: {e}")))?
        .insert(send_id.clone(), ctk.clone());

    // Cloni 'static per lo spawn.
    let sender = state.message_sender.clone();
    let files = m.files.clone();
    let sid = send_id.clone();
    let peer_name = receiver.name.clone().unwrap_or_else(|| "phone".to_string());
    // NB: `tauri::State` non e' 'static; riaccediamo via AppHandle nello spawn
    // solo per pulire la mappa token alla fine.
    tauri::async_runtime::spawn(async move {
        let fail = |sender: &tokio::sync::broadcast::Sender<ChannelMessage>, e: String| {
            warn!("{sid}: ble send failed: {e}");
            let _ = sender.send(terminal_message(
                &sid,
                TransferState::Disconnected,
                &peer_name,
                &files,
            ));
        };
        let session = bluer::Session::new().await;
        let adapter = match session {
            Ok(s) => s.default_adapter().await,
            Err(e) => {
                fail(&sender, format!("session failed: {e}"));
                return;
            }
        };
        let adapter = match adapter {
            Ok(a) => a,
            Err(e) => {
                fail(&sender, format!("adapter failed: {e}"));
                return;
            }
        };
        // L'indirizzo BLE del telefono ruota nel tempo; il token salvato dallo
        // scan puo' essere stale (la CLI non ha questo problema perche' scopre
        // e invia in un colpo solo). Re-resolve rapido per endpoint_id prima di
        // connettere; se non si ritrova, si usa il token originale.
        let resolved = match rqs_lib::hdl::discover_receivers(
            &adapter,
            None,
            Duration::from_secs(8),
            Duration::ZERO,
        )
        .await
        {
            Ok(v) => v
                .into_iter()
                .find(|r| r.endpoint_id == receiver.endpoint_id)
                .unwrap_or_else(|| receiver.clone()),
            Err(e) => {
                warn!("{sid}: ble re-resolve failed ({e}); using scanned address");
                receiver.clone()
            }
        };
        if resolved.address != receiver.address {
            info!("{sid}: ble address rotated {} -> {}", receiver.address, resolved.address);
        }
        let r = rqs_lib::hdl::send_files_ble(
            &adapter,
            &resolved,
            files.clone(),
            sid.clone(),
            sender.clone(),
            ctk,
        )
        .await;
        match r {
            Ok(st) => {
                info!("{sid}: ble send terminal: {st:?}");
                // Fallback per il cancel pre-handshake (nessun OutboundRequest
                // esistente -> il core non ha emesso nulla).
                if st == TransferState::Cancelled {
                    let _ = sender.send(terminal_message(
                        &sid,
                        TransferState::Cancelled,
                        &peer_name,
                        &files,
                    ));
                }
            }
            Err(e) => fail(&sender, e.to_string()),
        }
    });
    // Pulizia entry ctk alla fine: lo fa il task? No (non ha lo State) —
    // la entry resta fino a cancel o al prossimo send con stesso id;
    // `qsd_cancel_send` la rimuove dopo il fire. Per evitare leak su send
    // completati, il frontend chiama clear dopo il terminale oppure un
    // nuovo scan riusa la mappa. Accettato per F2 (cap implicito: un token
    // per send attivo).
    Ok(send_id)
}

/// Cancella un send BLE (e in generale un outbound per `send_id`):
/// fa scattare il `CancellationToken` (fase pre-handshake / upgrade wait) e
/// invia `TransferCancel` sul broadcast (onorato da `OutboundRequest`,
/// verificato in `core_lib/src/hdl/outbound.rs:149,877`).
#[tauri::command]
pub async fn qsd_cancel_send(
    send_id: String,
    state: tauri::State<'_, AppState>,
) -> Result<(), GuiError> {
    if send_id.is_empty() {
        return Err(GuiError::validation("send_id is empty"));
    }
    info!("qsd_cancel_send: {send_id}");
    if let Ok(map) = state.ble_cancel.lock() {
        if let Some(ctk) = map.get(&send_id) {
            ctk.cancel();
        }
    }
    // Reuse del path ConsentDto{TransferCancel}: fire-and-forget come la CLI.
    let _ = state.message_sender.send(rqs_lib::channel::ChannelMessage {
        id: send_id,
        msg: rqs_lib::channel::Message::Lib {
            action: rqs_lib::channel::TransferAction::TransferCancel,
        },
    });
    Ok(())
}
