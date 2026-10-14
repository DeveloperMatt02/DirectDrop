//! Bridge core -> GUI: `broadcast`/`watch` -> `emit(qsd://*)` su DTO.
//!
//! - `qsd://transfer`: dedup per `id` + throttle progress (§2.4), PIN incluso
//!   nel metadata, timeout di consenso `QSD_CONSENT_TIMEOUT` (default 60 s)
//!   con auto-decline (parita' CLI `quickshare-direct.rs`).
//! - `qsd://endpoints`: emit immediato (eventi rari resolve/remove mDNS).
//! - `qsd://visibility`: dal `watch` (il timer Temporarily -> Invisible di
//!   60 s vive gia' nel core `mdns.rs`; la GUI non lo reimplementa).
//! - `qsd://ble-presence`: evento throttled a 120 s.
//!
//! Robustezza broadcast: su `Lagged(n)` skip + debug log (come la CLI),
//! mai crash.

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use std::time::{Instant, SystemTime, UNIX_EPOCH};

use rqs_lib::Visibility;
use tauri::{AppHandle, Emitter, Manager};

use crate::AppState;
use crate::dto::{
    self, BLE_PRESENCE_THROTTLE, EVT_BLE_PRESENCE, EVT_ENDPOINTS, EVT_TRANSFER,
    EVT_VISIBILITY, BlePresenceEvent, EndpointDto, TransferDto, VisibilityEvent,
};
use crate::notification::send_request_notification;
use crate::notification::send_temporarily_notification;
use crate::store;

pub fn spawn_receiver_tasks(app_handle: &AppHandle) {
    spawn_transfer_task(app_handle);
    spawn_endpoints_task(app_handle);
    spawn_visibility_task(app_handle);
    spawn_ble_presence_task(app_handle);
}

fn now_millis() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

struct ProgressEntry {
    state: crate::dto::TransferStateDto,
    ack_bytes: u64,
    total_bytes: u64,
    at: Instant,
}

fn spawn_transfer_task(app_handle: &AppHandle) {
    let capp = app_handle.clone();
    tauri::async_runtime::spawn(async move {
        let state: tauri::State<'_, AppState> = capp.state();
        let mut rx = state.message_sender.subscribe();
        let sender = state.message_sender.clone();
        // `asked`: transfer su cui si e' gia' chiesto/notificato (il core ripete
        // gli update di stato). `pending_consent`: in attesa di risposta, per
        // cancellare l'auto-decline quando arriva uno stato non-Waiting.
        let asked: Arc<Mutex<HashSet<String>>> = Arc::default();
        let pending_consent: Arc<Mutex<HashSet<String>>> = Arc::default();
        let mut last: HashMap<String, ProgressEntry> = HashMap::new();
        let consent_timeout = dto::consent_timeout_secs();

        loop {
            let cm = match rx.recv().await {
                Ok(cm) => cm,
                Err(tokio::sync::broadcast::error::RecvError::Lagged(n)) => {
                    debug!("bridge transfer: skipped {n} state updates");
                    continue;
                }
                Err(_) => break,
            };
            let Some(transfer) = TransferDto::from_channel(&cm) else {
                continue;
            };
            let id = transfer.id.clone();
            let tstate = transfer.state;
            let (ack, total) = transfer
                .metadata
                .as_ref()
                .map(|m| (m.ack_bytes, m.total_bytes))
                .unwrap_or((0, 0));

            if tstate == crate::dto::TransferStateDto::WaitingForUserConsent {
                // Dedup: una sola notifica + un solo timer per `id`.
                if !asked.lock().unwrap().insert(id.clone()) {
                    continue;
                }
                pending_consent.lock().unwrap().insert(id.clone());
                last.insert(
                    id.clone(),
                    ProgressEntry { state: tstate, ack_bytes: ack, total_bytes: total, at: Instant::now() },
                );
                // Notifica di sistema (parita' CLI: nome + PIN).
                let name = transfer
                    .metadata
                    .as_ref()
                    .and_then(|m| m.source.as_ref())
                    .map(|s| s.name.clone())
                    .unwrap_or_else(|| "a nearby device".to_string());
                let pin = transfer.metadata.as_ref().and_then(|m| m.pin_code.clone());
                send_request_notification(name, pin, id.clone(), &capp);
                emit_transfer(&capp, &transfer);
                // Timeout consenso con auto-decline (default 60 s).
                let c_sender = sender.clone();
                let c_pending = pending_consent.clone();
                let c_asked = asked.clone();
                let c_id = id.clone();
                tauri::async_runtime::spawn(async move {
                    tokio::time::sleep(std::time::Duration::from_secs(consent_timeout)).await;
                    if c_pending.lock().unwrap().remove(&c_id) {
                        info!("{c_id}: consent timeout ({consent_timeout}s), auto-decline");
                        let _ = c_sender.send(rqs_lib::channel::ChannelMessage {
                            id: c_id.clone(),
                            msg: rqs_lib::channel::Message::Lib {
                                action: rqs_lib::channel::TransferAction::ConsentDecline,
                            },
                        });
                        // `asked` resta fino allo stato terminale per non
                        // rinotificare i duplicati in arrivo.
                        let _ = c_asked;
                    }
                });
                continue;
            }

            // Qualsiasi avanzamento oltre il consenso cancella il timer.
            if !pending_consent.lock().unwrap().remove(&id) {
                // non era in attesa: nessun timer da cancellare
            }

            if tstate.is_terminal() {
                // Ultimo stato terminale sempre emesso.
                emit_transfer(&capp, &transfer);
                asked.lock().unwrap().remove(&id);
                last.remove(&id);
                continue;
            }

            // Dedup + throttle progress (§2.4).
            match last.get(&id) {
                Some(prev)
                    if prev.state == tstate
                        && prev.ack_bytes == ack
                        && prev.total_bytes == total =>
                {
                    continue;
                }
                Some(prev)
                    if prev.state == tstate
                        && ack.saturating_sub(prev.ack_bytes) < dto::PROGRESS_BYTES_STEP
                        && prev.at.elapsed() < dto::PROGRESS_MIN_INTERVAL =>
                {
                    continue;
                }
                _ => {}
            }
            last.insert(
                id,
                ProgressEntry { state: tstate, ack_bytes: ack, total_bytes: total, at: Instant::now() },
            );
            emit_transfer(&capp, &transfer);
        }
    });
}

fn emit_transfer(app: &AppHandle, dto: &TransferDto) {
    if let Err(e) = app.emit(EVT_TRANSFER, dto) {
        debug!("bridge: emit {EVT_TRANSFER} failed: {e}");
    }
}

fn spawn_endpoints_task(app_handle: &AppHandle) {
    let capp = app_handle.clone();
    tauri::async_runtime::spawn(async move {
        let state: tauri::State<'_, AppState> = capp.state();
        let mut rx = state.dch_sender.subscribe();
        loop {
            let info = match rx.recv().await {
                Ok(info) => info,
                Err(tokio::sync::broadcast::error::RecvError::Lagged(n)) => {
                    debug!("bridge endpoints: skipped {n} updates");
                    continue;
                }
                Err(_) => break,
            };
            let dto = EndpointDto::from(&info);
            if let Err(e) = capp.emit(EVT_ENDPOINTS, &dto) {
                debug!("bridge: emit {EVT_ENDPOINTS} failed: {e}");
            }
        }
    });
}

fn spawn_visibility_task(app_handle: &AppHandle) {
    let capp = app_handle.clone();
    tauri::async_runtime::spawn(async move {
        let state: tauri::State<'_, AppState> = capp.state();
        let mut rx = state.visibility_sender.lock().unwrap().subscribe();
        loop {
            if rx.changed().await.is_err() {
                break;
            }
            let v: Visibility = *rx.borrow_and_update();
            if let Err(e) = store::save_visibility(&capp, v) {
                debug!("bridge: save_visibility failed: {e}");
            }
            let dto = VisibilityEvent { visibility: v.into() };
            if let Err(e) = capp.emit(EVT_VISIBILITY, &dto) {
                debug!("bridge: emit {EVT_VISIBILITY} failed: {e}");
            }
        }
    });
}

fn spawn_ble_presence_task(app_handle: &AppHandle) {
    let capp = app_handle.clone();
    tauri::async_runtime::spawn(async move {
        let state: tauri::State<'_, AppState> = capp.state();
        let mut rx = state.ble_receiver.resubscribe();
        let mut last_sent = Instant::now() - BLE_PRESENCE_THROTTLE;
        loop {
            match rx.recv().await {
                Ok(()) => {
                    if last_sent.elapsed() < BLE_PRESENCE_THROTTLE {
                        continue;
                    }
                    last_sent = Instant::now();
                    let evt = BlePresenceEvent { at: now_millis() };
                    if let Err(e) = capp.emit(EVT_BLE_PRESENCE, &evt) {
                        debug!("bridge: emit {EVT_BLE_PRESENCE} failed: {e}");
                    }
                    // Parita' CLI: hint solo se Invisible.
                    if store::get_visibility(&capp) == Visibility::Invisible {
                        send_temporarily_notification(&capp);
                    }
                }
                Err(e) => {
                    debug!("RecvError: ble_receiver: {e}");
                    // `Lagged` su un broadcast di `()` da 5 slot: ricomincia.
                    if matches!(
                        e,
                        tokio::sync::broadcast::error::RecvError::Closed
                    ) {
                        break;
                    }
                }
            }
        }
    });
}
