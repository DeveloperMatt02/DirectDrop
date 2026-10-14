use std::{path::PathBuf, sync::Arc, time::Duration};

use rqs_lib::Visibility;
use tauri::AppHandle;
use tauri_plugin_store::{Store, StoreExt};

fn _get_store(app_handle: &AppHandle) -> Arc<Store<tauri::Wry>> {
    app_handle
        .store_builder(".settings.json")
        .auto_save(Duration::from_millis(100))
        .build()
        .unwrap()
}

fn default_device_name() -> String {
    sys_metrics::host::get_hostname().unwrap_or_else(|_| "Unknown device".into())
}

pub fn init_default(app_handle: &AppHandle) {
    let store = _get_store(app_handle);

    if !store.has("autostart") {
        store.set("autostart", true);
    }

    if !store.has("realclose") {
        store.set("realclose", false);
    }

    if !store.has("visibility") {
        store.set("visibility", Visibility::Visible as u8);
    }

    if !store.has("startminimized") {
        store.set("startminimized", false);
    }

    if !store.has("device_name") {
        store.set("device_name", default_device_name());
    }
}

pub fn get_realclose(app_handle: &AppHandle) -> bool {
    let store = _get_store(app_handle);

    store
        .get("realclose")
        .and_then(|json| json.as_bool())
        .unwrap_or_default()
}

pub fn get_port(app_handle: &AppHandle) -> Option<u32> {
    let store = _get_store(app_handle);

    store
        .get("port")
        .and_then(|json| json.as_u64().map(|v| v as u32))
}

pub fn get_visibility(app_handle: &AppHandle) -> Visibility {
    let store = _get_store(app_handle);

    store
        .get("visibility")
        .and_then(|json| json.as_u64().map(Visibility::from_raw_value))
        .unwrap_or(Visibility::Visible)
}

/// Persiste senza emettere: l'emit `qsd://visibility` spetta al bridge
/// (`bridge.rs`), unico produttore dell'evento.
pub fn save_visibility(app_handle: &AppHandle, v: Visibility) -> Result<(), anyhow::Error> {
    let store = _get_store(app_handle);
    store.set("visibility", v as u8);
    Ok(())
}

pub fn get_device_name(app_handle: &AppHandle) -> String {
    let store = _get_store(app_handle);

    store
        .get("device_name")
        .and_then(|json| json.as_str().map(String::from))
        .unwrap_or_else(default_device_name)
}

pub fn set_device_name(app_handle: &AppHandle, name: &str) {
    let store = _get_store(app_handle);
    store.set("device_name", name);
}

pub fn get_download_path(app_handle: &AppHandle) -> Option<PathBuf> {
    let store = _get_store(app_handle);

    store
        .get("download_path")
        .and_then(|json| json.as_str().map(PathBuf::from))
}

pub fn set_download_path(app_handle: &AppHandle, path: Option<&PathBuf>) {
    let store = _get_store(app_handle);
    match path {
        Some(p) => store.set("download_path", p.to_string_lossy().as_ref()),
        None => {
            store.delete("download_path");
        }
    }
}

pub fn get_logging_level(app_handle: &AppHandle) -> Option<String> {
    let store = _get_store(app_handle);

    store
        .get("debug_level")
        .and_then(|json| json.as_str().map(String::from))
}

pub fn get_startminimized(app_handle: &AppHandle) -> bool {
    let store = _get_store(app_handle);

    store
        .get("startminimized")
        .and_then(|json| json.as_bool())
        .unwrap_or_default()
}
