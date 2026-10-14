use crate::AppState;
use crate::dto::{GuiError, VisibilityDto};

#[tauri::command]
pub fn qsd_set_visibility(
    visibility: VisibilityDto,
    state: tauri::State<'_, AppState>,
) -> Result<(), GuiError> {
    info!("qsd_set_visibility: {visibility:?}");
    state
        .rqs
        .lock()
        .map_err(|e| GuiError::core(format!("state lock poisoned: {e}")))?
        .change_visibility(visibility.into());
    Ok(())
}
