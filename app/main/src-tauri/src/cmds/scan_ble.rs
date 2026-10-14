use std::time::Duration;

use crate::AppState;
use crate::dto::{BleReceiverDto, GuiError, ScanBleInput};

/// Hex lowercase dell'endpoint id (4 byte Nearby).
pub fn endpoint_hex(id: &[u8; 4]) -> String {
    id.iter().map(|b| format!("{b:02x}")).collect()
}

/// Token opaco: `endpoint_hex-address` (stabile per la sessione di scan).
pub fn receiver_token(r: &rqs_lib::hdl::BleReceiver) -> String {
    format!("{}-{}", endpoint_hex(&r.endpoint_id), r.address)
}

impl From<&rqs_lib::hdl::BleReceiver> for BleReceiverDto {
    fn from(r: &rqs_lib::hdl::BleReceiver) -> Self {
        Self {
            address: r.address.to_string(),
            name: r.name.clone(),
            endpoint_id: endpoint_hex(&r.endpoint_id),
        }
    }
}

/// One-shot BLE scan (non streaming): apre il proprio handle `bluer::Adapter`
/// (come la CLI `quickshare-direct.rs`), esegue `discover_receivers` con
/// `timeout` (default 30 s, range 3–30) + `settle` fisso 3 s, popola la token
/// map (cap 32) e ritorna DTO display-only. Zero risultati -> `[]`, mai errore.
#[tauri::command]
pub async fn qsd_scan_ble(
    m: ScanBleInput,
    state: tauri::State<'_, AppState>,
) -> Result<Vec<BleReceiverDto>, GuiError> {
    let timeout_s = m.timeout_s.unwrap_or(30);
    if !(3..=30).contains(&timeout_s) {
        return Err(GuiError::validation(format!(
            "timeout_s {timeout_s} out of range 3-30"
        )));
    }
    info!(
        "qsd_scan_ble: name={:?} timeout_s={timeout_s}",
        m.name
    );

    let session = bluer::Session::new()
        .await
        .map_err(|e| GuiError::not_running(format!("bluetooth session: {e}")))?;
    let adapter = session
        .default_adapter()
        .await
        .map_err(|e| GuiError::not_running(format!("bluetooth adapter: {e}")))?;
    if !adapter
        .is_powered()
        .await
        .map_err(|e| GuiError::not_running(format!("bluetooth powered check: {e}")))?
    {
        return Err(GuiError::not_running("Bluetooth is off (turn it on and try again)"));
    }

    let found = rqs_lib::hdl::discover_receivers(
        &adapter,
        m.name.as_deref(),
        Duration::from_secs(timeout_s),
        Duration::from_secs(3),
    )
    .await
    .map_err(|e| GuiError::core(e.to_string()))?;

    let mut map = state
        .ble_receivers
        .lock()
        .map_err(|e| GuiError::core(format!("state lock poisoned: {e}")))?;
    let dtos: Vec<BleReceiverDto> = found
        .iter()
        .map(|r| {
            // Cap 32: evict arbitrario (HashMap senza ordine).
            if map.len() >= 32 && !map.contains_key(&receiver_token(r)) {
                if let Some(k) = map.keys().next().cloned() {
                    map.remove(&k);
                }
            }
            map.insert(receiver_token(r), r.clone());
            BleReceiverDto::from(r)
        })
        .collect();
    info!("qsd_scan_ble: found {}", dtos.len());
    Ok(dtos)
}
