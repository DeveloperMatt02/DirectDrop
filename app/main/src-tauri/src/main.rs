#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[macro_use]
extern crate log;

use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicBool;

use rqs_lib::{EndpointInfo, SendInfo, Visibility, RQS};
use store::get_startminimized;
#[cfg(target_os = "macos")]
use tauri::image::Image;
use tauri::{
    menu::{MenuBuilder, MenuItemBuilder},
    tray::TrayIconBuilder,
    AppHandle, Manager, Window, WindowEvent,
};
use tauri_plugin_autostart::MacosLauncher;
use tokio::sync::{broadcast, mpsc, watch};

use crate::bridge::spawn_receiver_tasks;
use crate::logger::set_up_logging;
use crate::store::{
    get_device_name, get_download_path, get_port, get_realclose, get_visibility, init_default,
};

mod bridge;
mod cmds;
mod dto;
mod logger;
mod notification;
mod store;

pub struct AppState {
    pub message_sender: broadcast::Sender<rqs_lib::channel::ChannelMessage>,
    pub dch_sender: broadcast::Sender<EndpointInfo>,
    pub visibility_sender: Arc<Mutex<watch::Sender<Visibility>>>,
    pub sender_file: mpsc::Sender<SendInfo>,
    pub ble_receiver: broadcast::Receiver<()>,
    pub discovery_running: AtomicBool,
    pub rqs: Mutex<RQS>,
    /// Token opachi BLE: id (`endpoint_hex-address`) -> `BleReceiver` (clone).
    /// Condiviso tra `qsd_scan_ble` e `qsd_send_ble`; cap 32, evict arbitrario.
    pub ble_receivers: Mutex<std::collections::HashMap<String, rqs_lib::hdl::BleReceiver>>,
    /// CancellationToken per `send_id` BLE (fase pre-handshake + upgrade wait).
    /// Il core onora anche `TransferCancel` via `message_sender` per l'outbound;
    /// `qsd_cancel_send` fa entrambe le cose.
    pub ble_cancel:
        Mutex<std::collections::HashMap<String, tokio_util::sync::CancellationToken>>,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Define tauri async runtime to be tokio
    tauri::async_runtime::set(tokio::runtime::Handle::current());

    // Build and run Tauri app
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            None,
        ))
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_single_instance::init(|app, _argv, _cwd| {
            trace!("tauri_plugin_single_instance: instance already running");
            open_main_window(app);
        }))
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            cmds::qsd_snapshot,
            cmds::qsd_set_visibility,
            cmds::qsd_set_download_path,
            cmds::qsd_set_device_name,
            cmds::qsd_start_discovery,
            cmds::qsd_stop_discovery,
            cmds::qsd_consent,
            cmds::qsd_send_lan,
            cmds::qsd_scan_ble,
            cmds::qsd_send_ble,
            cmds::qsd_cancel_send,
            cmds::qsd_get_hostname,
            cmds::get_hostname,
            cmds::change_download_path,
            cmds::qsd_open_path,
            cmds::qsd_system_theme,
        ])
        .setup(|app| {
            // Setting up logging inside file for the app
            set_up_logging(app.app_handle())?;

            debug!("Starting setup of DirectDrop app");

            // Initialize default values for the store
            init_default(app.app_handle());

            // Initialize system Tray
            let name = MenuItemBuilder::new("DirectDrop")
                .enabled(false)
                .build(app)?;
            let show = MenuItemBuilder::with_id("show", "Show").build(app)?;
            let quit = MenuItemBuilder::with_id("quit", "Quit").build(app)?;
            let menu = MenuBuilder::new(app)
                .item(&name)
                .separator()
                .items(&[&show, &quit])
                .build()?;

            #[cfg(target_os = "macos")]
            let icon = Image::from_bytes(include_bytes!("../icons/tray.png")).unwrap();
            #[cfg(not(target_os = "macos"))]
            let icon = app.default_window_icon().unwrap().clone();

            let tray = TrayIconBuilder::new()
                .icon(icon)
                .menu(&menu)
                .on_menu_event(move |app, event| match event.id().as_ref() {
                    "show" => {
                        trace!("tray_show");
                        open_main_window(app);
                    }
                    "quit" => {
                        trace!("tray_quit");
                        kill_app(app.app_handle());
                    }
                    _ => (),
                })
                .build(app)?;

            let _ = tray.set_icon_as_template(true);

            // Fetch initial configuration values
            let visibility = get_visibility(app.app_handle());
            let port_number = get_port(app.app_handle());
            let download_path = get_download_path(app.app_handle());
            let device_name = get_device_name(app.app_handle());

            let app_handle = app.app_handle().clone();
            // This is not optimal, but until I find a better way to init log
            // (inside file and stdout) before starting the lib, I'll keep it as
            // is. This allow me to get the whole log :)
            tokio::task::block_in_place(|| {
                tauri::async_runtime::block_on(async move {
                    trace!("Beginning of RQS start");
                    // Start the DirectDrop service (4 args contro il core attuale)
                    let mut rqs =
                        RQS::new(visibility, port_number, download_path, Some(device_name));
                    let (sender_file, ble_receiver) = rqs.run().await.unwrap();

                    // Define state for tauri app
                    app_handle.manage(AppState {
                        message_sender: rqs.message_sender.clone(),
                        dch_sender: broadcast::channel(10).0,
                        visibility_sender: rqs.visibility_sender.clone(),
                        sender_file,
                        ble_receiver,
                        discovery_running: AtomicBool::new(false),
                        rqs: Mutex::new(rqs),
                        ble_receivers: Mutex::default(),
                        ble_cancel: Mutex::default(),
                    });
                });
            });

            spawn_receiver_tasks(app.app_handle());
            cmds::spawn_system_theme_watch(app.app_handle());
            Ok(())
        })
        .on_window_event(handle_window_event)
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|app_handle, event| match event {
            tauri::RunEvent::Ready { .. } => {
                trace!("RunEvent::Ready");
                if get_startminimized(app_handle) {
                    #[cfg(not(target_os = "macos"))]
                    app_handle
                        .get_webview_window("main")
                        .unwrap()
                        .hide()
                        .unwrap();
                    #[cfg(target_os = "macos")]
                    app_handle.hide().unwrap();
                }
            }
            tauri::RunEvent::ExitRequested { code, .. } => {
                trace!("RunEvent::ExitRequested");
                if code != Some(-1) {
                    kill_app(app_handle);
                }
            }
            #[cfg(target_os = "macos")]
            tauri::RunEvent::Reopen { .. } => {
                trace!("RunEvent::Reopen");
                open_main_window(app_handle);
            }
            _ => {}
        });

    info!("Application stopped");
    Ok(())
}

fn handle_window_event(w: &Window, event: &WindowEvent) {
    if let tauri::WindowEvent::CloseRequested { api, .. } = event {
        if get_realclose(w.app_handle()) {
            trace!("handle_window_event: real close");
            return;
        }

        trace!("handle_window_event: prevent close");
        w.hide().unwrap();
        api.prevent_close();
    }
}

fn open_main_window(app_handle: &AppHandle) {
    if let Some(webview_window) = app_handle.get_webview_window("main") {
        let _ = webview_window.show();
        let _ = webview_window.set_focus();
        return;
    }

    warn!("open_main_window: no main window found");
}

fn kill_app(app_handle: &AppHandle) {
    let state: tauri::State<'_, AppState> = app_handle.state();

    tokio::task::block_in_place(|| {
        #[allow(clippy::await_holding_lock)]
        tauri::async_runtime::block_on(async move {
            let _ = state.rqs.lock().unwrap().stop().await;
        });
    });

    app_handle.exit(-1);
}
