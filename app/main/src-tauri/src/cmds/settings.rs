//! `qsd_snapshot` + `qsd_set_device_name` (+ persist store).

use std::sync::atomic::Ordering;

use tauri::AppHandle;

use crate::AppState;
use crate::dto::{DTO_VERSION, GuiError, SnapshotDto};
use crate::store;

/// Chiamato al mount: sostituisce i getter sparsi precedenti.
#[tauri::command]
pub async fn qsd_snapshot(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
) -> Result<SnapshotDto, GuiError> {
    let (port, device_name) = {
        let guard = state
            .rqs
            .lock()
            .map_err(|e| GuiError::core(format!("state lock poisoned: {e}")))?;
        (guard.port_number, guard.get_device_name())
    };
    Ok(SnapshotDto {
        dto_version: DTO_VERSION,
        visibility: store::get_visibility(&app).into(),
        device_name,
        download_path: store::get_download_path(&app)
            .map(|p| p.to_string_lossy().into_owned()),
        port,
        discovery_running: state.discovery_running.load(Ordering::SeqCst),
    })
}

/// 1-255 char validati. Effettivo sui nuovi annunci (richiede restart del
/// service, documentato in `lib.rs`); la GUI mostra hint, mai restart
/// automatico durante un transfer.
#[tauri::command]
pub fn qsd_set_device_name(
    app: AppHandle,
    name: String,
    state: tauri::State<'_, AppState>,
) -> Result<(), GuiError> {
    let name = name.trim().to_string();
    if name.is_empty() || name.len() > 255 {
        return Err(GuiError::validation(
            "device name must be 1-255 characters",
        ));
    }
    info!("qsd_set_device_name: {name:?}");
    state
        .rqs
        .lock()
        .map_err(|e| GuiError::core(format!("state lock poisoned: {e}")))?
        .set_device_name(name.clone());
    store::set_device_name(&app, &name);
    Ok(())
}
