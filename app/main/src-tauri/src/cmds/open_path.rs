use std::path::{Component, Path, PathBuf};
use tauri::AppHandle;

use crate::dto::GuiError;
use crate::store;

fn extended_path() -> String {
    let mut path = std::env::var("PATH").unwrap_or_default();
    if let Ok(user) = std::env::var("USER") {
        path.push_str(&format!(":/etc/profiles/per-user/{user}/bin"));
    }
    path.push_str(":/run/current-system/sw/bin:/usr/local/bin:/usr/bin:/bin");
    path
}

fn file_uri(path: &Path) -> String {
    let mut s = String::from("file://");
    for b in path.to_string_lossy().bytes() {
        if b.is_ascii_alphanumeric() || b"-._~/".contains(&b) {
            s.push(b as char);
        } else {
            s.push_str(&format!("%{b:02X}"));
        }
    }
    s
}

fn open_detached(target: &Path) -> Result<(), GuiError> {
    let path = extended_path();
    let target_str = target.to_string_lossy();
    let scoped = std::process::Command::new("systemd-run")
        .args(["--user", "--scope", "--quiet", "--collect", "xdg-open"])
        .arg(target_str.as_ref())
        .env("PATH", &path)
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn();

    if scoped.is_err() {
        std::process::Command::new("xdg-open")
            .arg(target_str.as_ref())
            .env("PATH", &path)
            .stdin(std::process::Stdio::null())
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn()
            .map_err(|e| GuiError::io(format!("failed to run xdg-open: {e}")))?;
    }
    Ok(())
}

fn reveal_in_file_manager(dir: &Path, file_path: &Path) -> Result<(), GuiError> {
    let uri = file_uri(file_path);
    let path = extended_path();
    let mut cmd = std::process::Command::new("busctl");
    cmd.args([
        "--user",
        "call",
        "org.freedesktop.FileManager1",
        "/org/freedesktop/FileManager1",
        "org.freedesktop.FileManager1",
        "ShowItems",
        "ass",
        "1",
    ])
    .arg(&uri)
    .arg("")
    .env("PATH", &path)
    .stdin(std::process::Stdio::null())
    .stdout(std::process::Stdio::null())
    .stderr(std::process::Stdio::null());

    let ok = cmd.status().map(|s| s.success()).unwrap_or(false);
    if !ok {
        open_detached(dir)?;
    }
    Ok(())
}

pub fn resolve_target_path(download_dir: &Path, name: Option<&str>) -> Result<PathBuf, GuiError> {
    let canonical_dir = download_dir
        .canonicalize()
        .map_err(|e| GuiError::io(format!("download directory not found: {e}")))?;

    if !canonical_dir.is_dir() {
        return Err(GuiError::validation("download path is not a directory"));
    }

    match name {
        None => Ok(canonical_dir),
        Some(n) => {
            if n.is_empty() {
                return Err(GuiError::validation("file name cannot be empty"));
            }
            if n.contains('/') || n.contains('\\') || n.contains("..") {
                return Err(GuiError::validation(
                    "file name contains invalid characters or path traversal",
                ));
            }
            let p = Path::new(n);
            if p.is_absolute() {
                return Err(GuiError::validation("file name cannot be absolute"));
            }
            let mut components = p.components();
            match components.next() {
                Some(Component::Normal(_)) if components.next().is_none() => {}
                _ => return Err(GuiError::validation("invalid file name")),
            }

            let candidate = canonical_dir.join(n);
            let canonical_target = candidate.canonicalize().map_err(|e| {
                GuiError::io(format!(
                    "file '{n}' not found in download directory: {e}"
                ))
            })?;

            if !canonical_target.starts_with(&canonical_dir) {
                return Err(GuiError::validation("path escapes download directory"));
            }

            Ok(canonical_target)
        }
    }
}

#[tauri::command]
pub async fn qsd_open_path(
    app: AppHandle,
    name: Option<String>,
    reveal: bool,
) -> Result<(), GuiError> {
    tokio::task::spawn_blocking(move || {
        let download_dir = store::get_download_path(&app)
            .unwrap_or_else(rqs_lib::utils::get_download_dir);

        let target = resolve_target_path(&download_dir, name.as_deref())?;

        if reveal {
            if name.is_some() && target.is_file() {
                let parent = target.parent().unwrap_or(&download_dir);
                reveal_in_file_manager(parent, &target)?;
            } else {
                open_detached(&target)?;
            }
        } else {
            open_detached(&target)?;
        }

        Ok(())
    })
    .await
    .map_err(|e| GuiError::core(format!("task join error: {e}")))?
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};

    struct TempTestDir(PathBuf);
    impl TempTestDir {
        fn new(prefix: &str) -> Self {
            let unique = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos();
            let dir = std::env::temp_dir().join(format!("{prefix}-{unique}"));
            fs::create_dir_all(&dir).unwrap();
            Self(dir)
        }
        fn path(&self) -> &Path {
            &self.0
        }
    }
    impl Drop for TempTestDir {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    #[test]
    fn test_resolve_download_dir_none() {
        let tmp = TempTestDir::new("qsd-test-none");
        let path = resolve_target_path(tmp.path(), None).unwrap();
        assert_eq!(path, tmp.path().canonicalize().unwrap());
    }

    #[test]
    fn test_resolve_valid_file() {
        let tmp = TempTestDir::new("qsd-test-valid");
        let file_path = tmp.path().join("test_file.txt");
        File::create(&file_path).unwrap();

        let resolved = resolve_target_path(tmp.path(), Some("test_file.txt")).unwrap();
        assert_eq!(resolved, file_path.canonicalize().unwrap());
    }

    #[test]
    fn test_resolve_nonexistent_file() {
        let tmp = TempTestDir::new("qsd-test-missing");
        let err = resolve_target_path(tmp.path(), Some("missing.txt")).unwrap_err();
        assert_eq!(err.code, "IO");
    }

    #[test]
    fn test_resolve_path_traversal() {
        let tmp = TempTestDir::new("qsd-test-traversal");
        let file_path = tmp.path().join("test_file.txt");
        File::create(&file_path).unwrap();

        assert_eq!(
            resolve_target_path(tmp.path(), Some("../outside.txt"))
                .unwrap_err()
                .code,
            "VALIDATION"
        );
        assert_eq!(
            resolve_target_path(tmp.path(), Some(".."))
                .unwrap_err()
                .code,
            "VALIDATION"
        );
        assert_eq!(
            resolve_target_path(tmp.path(), Some("dir/../../outside.txt"))
                .unwrap_err()
                .code,
            "VALIDATION"
        );
        assert_eq!(
            resolve_target_path(tmp.path(), Some("/etc/passwd"))
                .unwrap_err()
                .code,
            "VALIDATION"
        );
        assert_eq!(
            resolve_target_path(tmp.path(), Some(""))
                .unwrap_err()
                .code,
            "VALIDATION"
        );
    }

    #[test]
    fn test_resolve_symlink_escape() {
        let tmp_outside = TempTestDir::new("qsd-test-outside");
        let outside_file = tmp_outside.path().join("secret.txt");
        File::create(&outside_file).unwrap();

        let tmp_download = TempTestDir::new("qsd-test-download");
        let symlink_path = tmp_download.path().join("link_to_outside.txt");

        #[cfg(unix)]
        std::os::unix::fs::symlink(&outside_file, &symlink_path).unwrap();

        #[cfg(unix)]
        {
            let err =
                resolve_target_path(tmp_download.path(), Some("link_to_outside.txt")).unwrap_err();
            assert_eq!(err.code, "VALIDATION");
            assert!(err.message.contains("escapes"));
        }
    }

    #[test]
    fn test_resolve_symlink_inside() {
        let tmp = TempTestDir::new("qsd-test-inside");
        let sub = tmp.path().join("subdir");
        fs::create_dir(&sub).unwrap();
        let real_file = sub.join("inside.txt");
        File::create(&real_file).unwrap();

        let symlink_path = tmp.path().join("link_inside.txt");

        #[cfg(unix)]
        std::os::unix::fs::symlink(&real_file, &symlink_path).unwrap();

        #[cfg(unix)]
        {
            let resolved = resolve_target_path(tmp.path(), Some("link_inside.txt")).unwrap();
            assert_eq!(resolved, real_file.canonicalize().unwrap());
        }
    }
}
