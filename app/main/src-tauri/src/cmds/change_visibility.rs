use crate::AppState;
use crate::dto::{GuiError, VisibilityDto};

#[tauri::command]
pub fn qsd_set_visibility(
    v: VisibilityDto,
    state: tauri::State<'_, AppState>,
) -> Result<(), GuiError> {
    info!("qsd_set_visibility: {v:?}");
    state
        .rqs
        .lock()
        .map_err(|e| GuiError::core(format!("state lock poisoned: {e}")))?
        .change_visibility(v.into());
    Ok(())
}
