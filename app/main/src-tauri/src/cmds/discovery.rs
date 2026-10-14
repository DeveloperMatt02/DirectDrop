use std::sync::atomic::Ordering;

use crate::AppState;
use crate::dto::GuiError;

#[tauri::command]
pub async fn qsd_start_discovery(
    state: tauri::State<'_, AppState>,
) -> Result<(), GuiError> {
    info!("qsd_start_discovery");
    // Idempotente: se gia' running, `RQS::discovery` ri-spawna il task;
    // il flag resta `true`.
    state
        .rqs
        .lock()
        .map_err(|e| GuiError::core(format!("state lock poisoned: {e}")))?
        .discovery(state.dch_sender.clone())
        .map_err(|e| {
            if e.to_string().contains("wasn't first started") {
                GuiError::not_running(e.to_string())
            } else {
                GuiError::core(e.to_string())
            }
        })?;
    state.discovery_running.store(true, Ordering::SeqCst);
    Ok(())
}

#[tauri::command]
pub fn qsd_stop_discovery(state: tauri::State<'_, AppState>) -> Result<(), GuiError> {
    info!("qsd_stop_discovery");
    state
        .rqs
        .lock()
        .map_err(|e| GuiError::core(format!("state lock poisoned: {e}")))?
        .stop_discovery();
    state.discovery_running.store(false, Ordering::SeqCst);
    Ok(())
}
