use std::io::{ErrorKind, Write};
use std::process::{Command, Stdio};

pub(crate) fn copy_to_clipboard(text: &str) -> Result<(), String> {
    let mut child = Command::new("wl-copy")
        .arg("--type")
        .arg("text/plain;charset=utf-8")
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|e| {
            if e.kind() == ErrorKind::NotFound {
                "`wl-copy` was not found; install `wl-clipboard`".to_string()
            } else {
                format!("failed to start `wl-copy`: {e}")
            }
        })?;

    let mut stdin = child
        .stdin
        .take()
        .ok_or_else(|| "failed to open `wl-copy` stdin".to_string())?;
    stdin
        .write_all(text.as_bytes())
        .map_err(|e| format!("failed to write to `wl-copy`: {e}"))?;
    drop(stdin);

    Ok(())
}
