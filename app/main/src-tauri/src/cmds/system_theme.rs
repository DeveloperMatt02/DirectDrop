//! Desktop color scheme from the freedesktop Settings portal (Linux only).
//!
//! tao's `theme()` and WebKitGTK's `prefers-color-scheme` only look at the
//! GTK theme name, so GNOME's "prefer-dark" with Adwaita reaches neither.
//! The GUI reads `org.freedesktop.appearance color-scheme` through
//! `qsd_system_theme` at startup and follows `qsd://system-theme` on changes.
//! Any portal failure only logs a warning: the GUI keeps its fallback chain
//! (Tauri `theme()`, then the media query). Other OSes: always no preference.

use serde::Serialize;
use tauri::AppHandle;

pub const EVT_SYSTEM_THEME: &str = "qsd://system-theme";

/// `qsd://system-theme` payload: "dark", "light" or null (no preference).
#[derive(Clone, Serialize)]
pub struct SystemThemeEvent {
    pub theme: Option<&'static str>,
}

/// Portal `color-scheme`: 0 no preference, 1 prefer dark, 2 prefer light
/// (unknown values count as no preference).
#[cfg(any(target_os = "linux", test))]
fn theme_from_color_scheme(value: u32) -> Option<&'static str> {
    match value {
        1 => Some("dark"),
        2 => Some("light"),
        _ => None,
    }
}

#[cfg(target_os = "linux")]
mod portal {
    use std::future::Future;
    use std::time::Duration;

    use ashpd::desktop::settings::{Settings, APPEARANCE_NAMESPACE, COLOR_SCHEME_KEY};
    use ashpd::zbus::export::futures_util::StreamExt;
    use tauri::{AppHandle, Emitter};

    use super::{theme_from_color_scheme, SystemThemeEvent, EVT_SYSTEM_THEME};

    const PORTAL_TIMEOUT: Duration = Duration::from_secs(2);

    /// Runs a portal call with the timeout; logs and returns None on failure.
    async fn call<T>(
        what: &str,
        fut: impl Future<Output = Result<T, ashpd::Error>>,
    ) -> Option<T> {
        match tokio::time::timeout(PORTAL_TIMEOUT, fut).await {
            Ok(Ok(v)) => Some(v),
            Ok(Err(e)) => {
                warn!("system theme: {what} failed: {e}");
                None
            }
            Err(_) => {
                warn!("system theme: {what} timed out");
                None
            }
        }
    }

    pub async fn read() -> Option<&'static str> {
        let settings = call("settings portal", Settings::new()).await?;
        let value = call(
            "color-scheme read",
            settings.read::<u32>(APPEARANCE_NAMESPACE, COLOR_SCHEME_KEY),
        )
        .await?;
        theme_from_color_scheme(value)
    }

    pub fn spawn_watch(app: AppHandle) {
        tauri::async_runtime::spawn(async move {
            let Some(settings) = call("settings portal", Settings::new()).await else {
                return;
            };
            let Some(stream) = call(
                "color-scheme subscription",
                settings.receive_setting_changed_with_args::<u32>(
                    APPEARANCE_NAMESPACE,
                    COLOR_SCHEME_KEY,
                ),
            )
            .await
            else {
                return;
            };
            let mut stream = std::pin::pin!(stream);
            while let Some(item) = stream.next().await {
                match item {
                    Ok(value) => {
                        let evt = SystemThemeEvent {
                            theme: theme_from_color_scheme(value),
                        };
                        if let Err(e) = app.emit(EVT_SYSTEM_THEME, evt) {
                            warn!("system theme: emit failed: {e}");
                        }
                    }
                    Err(e) => warn!("system theme: bad color-scheme value: {e}"),
                }
            }
            warn!("system theme: portal signal stream ended");
        });
    }
}

/// Initial desktop color scheme: "dark", "light" or null (no preference,
/// portal missing or failing, not Linux).
#[tauri::command]
pub async fn qsd_system_theme() -> Option<String> {
    #[cfg(target_os = "linux")]
    {
        portal::read().await.map(String::from)
    }
    #[cfg(not(target_os = "linux"))]
    {
        None
    }
}

/// Follows portal changes in the background (Linux); never blocks setup.
pub fn spawn_system_theme_watch(app: &AppHandle) {
    #[cfg(target_os = "linux")]
    portal::spawn_watch(app.clone());
    #[cfg(not(target_os = "linux"))]
    let _ = app;
}

#[cfg(test)]
mod tests {
    use super::theme_from_color_scheme;

    #[test]
    fn color_scheme_values() {
        assert_eq!(theme_from_color_scheme(0), None);
        assert_eq!(theme_from_color_scheme(1), Some("dark"));
        assert_eq!(theme_from_color_scheme(2), Some("light"));
        assert_eq!(theme_from_color_scheme(7), None);
    }
}
