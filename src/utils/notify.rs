use std::process::{Command, Stdio};
use std::io::{self, Write};

pub fn notify(args: &[&str]) -> io::Result<String> {
    let output = Command::new("notify-send")
        .arg("-a")
        .arg("ferret-cli")
        .args(args)
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("notify-send failed: {}", stderr),
        ));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(stdout.trim().to_string())
}

pub fn close_notification(identifier: &str) -> io::Result<()> {
    let status = Command::new("gdbus")
        .arg("call")
        .arg("--session")
        .arg("--dest=org.freedesktop.Notifications")
        .arg("--object-path=/org/freedesktop/Notifications")
        .arg("--method=org.freedesktop.Notifications.CloseNotification")
        .arg(identifier)
        .stdout(Stdio::null()) 
        .stderr(Stdio::null())
        .status()?;

    if !status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Failed to close notification via gdbus",
        ));
    }

    Ok(())
}
