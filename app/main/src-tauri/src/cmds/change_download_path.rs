use std::path::PathBuf;

use tauri::AppHandle;

use crate::AppState;
use crate::dto::GuiError;
use crate::store;

#[tauri::command]
pub fn qsd_set_download_path(
    app: AppHandle,
    p: Option<String>,
    state: tauri::State<'_, AppState>,
) -> Result<(), GuiError> {
    info!("qsd_set_download_path: {p:?}");
    let path = p.map(PathBuf::from);
    state
        .rqs
        .lock()
        .map_err(|e| GuiError::core(format!("state lock poisoned: {e}")))?
        .set_download_path(path.clone());
    store::set_download_path(&app, path.as_ref());
    Ok(())
}

// Compat: il frontend esistente invoca ancora `change_download_path`.
#[tauri::command]
pub fn change_download_path(
    app: AppHandle,
    message: Option<String>,
    state: tauri::State<'_, AppState>,
) -> Result<(), GuiError> {
    qsd_set_download_path(app, message, state)
}
