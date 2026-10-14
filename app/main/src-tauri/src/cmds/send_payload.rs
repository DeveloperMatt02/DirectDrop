use std::path::Path;

use crate::AppState;
use crate::dto::{GuiError, SendPayloadDto};

#[tauri::command]
pub async fn qsd_send_lan(
    m: SendPayloadDto,
    state: tauri::State<'_, AppState>,
) -> Result<(), GuiError> {
    info!("qsd_send_lan: id={} addr={} files={}", m.id, m.addr, m.files.len());
    if m.id.is_empty() {
        return Err(GuiError::validation("id is empty"));
    }
    if m.addr.is_empty() {
        return Err(GuiError::validation("addr is empty (ip:port from EndpointDto)"));
    }
    if m.files.is_empty() {
        return Err(GuiError::validation("no files given"));
    }
    // Parita' CLI: solo file regolari, cartelle rifiutate con VALIDATION.
    for f in &m.files {
        let p = Path::new(f);
        if !p.is_file() {
            return Err(GuiError::validation(format!(
                "{f}: not a regular file (folders aren't supported)"
            )));
        }
    }
    state
        .sender_file
        .send(m.into_send_info())
        .await
        .map_err(|e| GuiError::core(format!("couldn't send payload: {e}")))?;
    Ok(())
}
