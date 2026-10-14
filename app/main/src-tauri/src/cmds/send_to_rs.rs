use crate::AppState;
use crate::dto::{ConsentDto, GuiError};

/// Fire-and-forget come la CLI: dopo l'invio il backend non attende ack.
#[tauri::command]
pub fn qsd_consent(m: ConsentDto, state: tauri::State<'_, AppState>) -> Result<(), GuiError> {
    info!("qsd_consent: {:?} {:?}", m.id, m.action);
    if m.id.is_empty() {
        return Err(GuiError::validation("id is empty"));
    }
    state
        .message_sender
        .send(m.into_channel_message())
        .map_err(|e| GuiError::core(format!("couldn't perform consent: {e}")))?;
    Ok(())
}
