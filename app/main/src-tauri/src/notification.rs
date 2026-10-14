#[cfg(target_os = "linux")]
use notify_rust::Notification;
use rqs_lib::Visibility;
use tauri::AppHandle;
#[cfg(not(target_os = "linux"))]
use tauri_plugin_notification::NotificationExt;
use tauri::Manager;

use crate::AppState;

/// Notifica di consenso con PIN (parita' CLI): Accept/Decline.
pub fn send_request_notification(
    name: String,
    pin: Option<String>,
    id: String,
    app_handle: &AppHandle,
) {
    let pin = pin.map(|p| format!(" · PIN {p}")).unwrap_or_default();
    let body = format!("{name} wants to share with you{pin}");

    #[cfg(not(target_os = "linux"))]
    let _ = app_handle
        .notification()
        .builder()
        .title("RQuickShare")
        .body(&body)
        .show();

    #[cfg(target_os = "linux")]
    match Notification::new()
        .summary("RQuickShare")
        .body(&body)
        .action("accept", "Accept")
        .action("reject", "Reject")
        .show()
    {
        Ok(n) => {
            let capp_handle = app_handle.clone();
            tokio::task::spawn(async move {
                n.wait_for_action(|action| match action {
                    "accept" => {
                        let state: tauri::State<'_, AppState> = capp_handle.state();
                        let _ = state.message_sender.send(rqs_lib::channel::ChannelMessage {
                            id: id.clone(),
                            msg: rqs_lib::channel::Message::Lib {
                                action: rqs_lib::channel::TransferAction::ConsentAccept,
                            },
                        });
                    }
                    "reject" => {
                        let state: tauri::State<'_, AppState> = capp_handle.state();
                        let _ = state.message_sender.send(rqs_lib::channel::ChannelMessage {
                            id: id.clone(),
                            msg: rqs_lib::channel::Message::Lib {
                                action: rqs_lib::channel::TransferAction::ConsentDecline,
                            },
                        });
                    }
                    _ => (),
                });
            });
        }
        Err(e) => {
            error!("Couldn't show notification: {}", e);
        }
    }
}

pub fn send_temporarily_notification(app_handle: &AppHandle) {
    let body = "RQuickShare is temporarily hidden".to_string();

    #[cfg(not(target_os = "linux"))]
    let _ = app_handle
        .notification()
        .builder()
        .title("RQuickShare")
        .body(&body)
        .show();

    #[cfg(target_os = "linux")]
    match Notification::new()
        .summary("RQuickShare")
        .body(&body)
        .action("visible", "Be visible (1m)")
        .action("ignore", "Ignore")
        .id(1919)
        .show()
    {
        Ok(n) => {
            let capp_handle = app_handle.clone();
            tokio::task::spawn(async move {
                n.wait_for_action(|action| match action {
                    "visible" => {
                        let state: tauri::State<'_, AppState> = capp_handle.state();
                        state
                            .rqs
                            .lock()
                            .unwrap()
                            .change_visibility(Visibility::Temporarily);
                    }
                    "ignore" => {}
                    _ => (),
                });
            });
        }
        Err(e) => {
            error!("Couldn't show notification: {}", e);
        }
    }
}
