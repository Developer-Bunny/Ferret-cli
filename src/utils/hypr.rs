use serde_json::Value;
use std::env;
use std::io::{self, Read, Write};
use std::os::unix::net::UnixStream;
use std::path::PathBuf;

#[derive(Debug)]
pub enum HyprResponse {
    Raw(String),
    Json(Value),
}

impl HyprResponse {
    pub fn as_raw(&self) -> Option<&str> {
        match self {
            HyprResponse::Raw(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_json(&self) -> Option<&Value> {
        match self {
            HyprResponse::Json(v) => Some(v),
            _ => None,
        }
    }
}

fn get_socket_path() -> io::Result<PathBuf> {
    let runtime_dir = env::var("XDG_RUNTIME_DIR").map_err(|e| {
        io::Error::new(
            io::ErrorKind::NotFound,
            format!("XDG_RUNTIME_DIR not set: {}", e),
        )
    })?;

    let signature = env::var("HYPRLAND_INSTANCE_SIGNATURE").map_err(|e| {
        io::Error::new(
            io::ErrorKind::NotFound,
            format!("HYPRLAND_INSTANCE_SIGNATURE not set: {}", e),
        )
    })?;

    Ok(PathBuf::from(runtime_dir)
        .join("hypr")
        .join(signature)
        .join(".socket.sock"))
}

pub fn message(msg: &str, as_json: bool) -> io::Result<HyprResponse> {
    let socket_path = get_socket_path()?;

    let mut stream = UnixStream::connect(socket_path)?;

    let payload = if as_json {
        format!("j/{}", msg)
    } else {
        msg.to_string()
    };

    stream.write_all(payload.as_bytes())?;

    let mut response = String::new();
    stream.read_to_string(&mut response)?;

    if as_json {
        let v: Value = serde_json::from_str(&response)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        Ok(HyprResponse::Json(v))
    } else {
        Ok(HyprResponse::Raw(response))
    }
}

pub fn dispatch(dispatcher: &str, args: &[&str]) -> bool {
    let args_str = args.join(" ");
    let cmd = format!("dispatch {} {}", dispatcher, args_str);

    let clean_cmd = cmd.trim_end();

    match message(clean_cmd, false) {
        Ok(HyprResponse::Raw(resp)) => resp == "ok",
        _ => false,
    }
}

pub fn batch(msgs: &[&str], json_inner: bool) -> io::Result<HyprResponse> {
    let processed_msgs: Vec<String> = if json_inner {
        msgs.iter().map(|m| format!("j/{}", m.trim())).collect()
    } else {
        msgs.iter().map(|m| m.trim().to_string()).collect()
    };

    let payload = format!("[[BATCH]]{}", processed_msgs.join(";"));

    message(&payload, false)
}
