#[tauri::command]
pub fn qsd_get_hostname() -> String {
    sys_metrics::host::get_hostname().unwrap_or_else(|_| String::from("Unknown"))
}

#[tauri::command]
pub fn get_hostname() -> String {
    qsd_get_hostname()
}
