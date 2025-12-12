use clap::Args;
use serde::Deserialize;
use serde_json::{Value, json};
use std::collections::HashMap;
use std::error::Error;
use std::fs;

use crate::utils::hypr::{self, HyprResponse};
use crate::utils::paths::Paths;

#[derive(Args, Debug)]
pub struct ToggleCmd {
    pub workspace: String,
}

#[derive(Deserialize, Debug, Clone)]
struct ClientConfig {
    #[serde(default)]
    enable: bool,
    #[serde(rename = "match")]
    matches: Option<Vec<Value>>,
    command: Option<Vec<String>>,
    #[serde(rename = "move")]
    should_move: Option<bool>,
}

type ClientGroup = HashMap<String, ClientConfig>;
type FullConfig = HashMap<String, ClientGroup>;

fn command_exists(cmd: &str) -> bool {
    if cmd.ends_with(".desktop") {
        return true;
    }
    which::which(cmd).is_ok()
}

fn is_subset(superset: &Value, subset: &Value) -> bool {
    match (superset, subset) {
        (Value::Object(super_map), Value::Object(sub_map)) => {
            for (k, v_sub) in sub_map {
                match super_map.get(k) {
                    Some(v_super) => {
                        if !is_subset(v_super, v_sub) {
                            return false;
                        }
                    }
                    None => return false,
                }
            }
            true
        }
        (Value::String(s_super), Value::String(s_sub)) => s_super.contains(s_sub),
        (Value::Array(arr_super), Value::Array(arr_sub)) => {
            arr_sub.iter().all(|sub_item| arr_super.contains(sub_item))
        }
        (v_super, v_sub) => v_super == v_sub,
    }
}

fn get_default_config() -> FullConfig {
    let j = json!({
        "communication": {
            "discord": { "enable": true, "match": [{"class": "discord"}], "command": ["discord"], "move": true },
            "whatsapp": { "enable": true, "match": [{"class": "whatsapp"}], "move": true }
        },
        "music": {
            "spotify": {
                "enable": true,
                "match": [{"class": "Spotify"}, {"initialTitle": "Spotify"}, {"initialTitle": "Spotify Free"}],
                "command": ["spicetify", "watch", "-s"],
                "move": true
            },
            "feishin": { "enable": true, "match": [{"class": "feishin"}], "move": true }
        },
        "sysmon": {
            "btop": {
                "enable": true,
                "match": [{"class": "btop", "title": "btop", "workspace": {"name": "special:sysmon"}}],
                "command": ["foot", "-a", "btop", "-T", "btop", "fish", "-C", "exec btop"]
            }
        },
        "todo": {
            "todoist": { "enable": true, "match": [{"class": "Todoist"}], "command": ["todoist"], "move": true }
        }
    });
    serde_json::from_value(j).unwrap()
}

fn load_config(paths: &Paths) -> FullConfig {
    let mut config = get_default_config();

    if let Ok(content) = fs::read_to_string(&paths.user_config_path)
        && let Ok(json_val) = serde_json::from_str::<Value>(&content)
        && let Some(toggles) = json_val.get("toggles").and_then(|v| v.as_object())
    {
        for (group_name, group_val) in toggles {
            if let Some(group_obj) = group_val.as_object() {
                let target_group = config.entry(group_name.clone()).or_default();

                for (client_name, client_val) in group_obj {
                    if let Ok(client_cfg) =
                        serde_json::from_value::<ClientConfig>(client_val.clone())
                    {
                        target_group.insert(client_name.clone(), client_cfg);
                    }
                }
            }
        }
    }
    config
}

impl ToggleCmd {
    pub fn run(&self, paths: &Paths) -> Result<(), Box<dyn Error>> {
        if self.workspace == "specialws" {
            self.toggle_special_ws()?;
            return Ok(());
        }

        let config = load_config(paths);

        let clients_cache: Option<Vec<Value>> = None;

        let mut spawned = false;

        if let Some(group) = config.get(&self.workspace) {
            for client_cfg in group.values() {
                if client_cfg.enable
                    && self.handle_client_config(
                        client_cfg,
                        &self.workspace,
                        clients_cache.clone(),
                    )?
                {
                    spawned = true;
                }
            }
        }

        if !spawned {
            hypr::dispatch("togglespecialworkspace", &[&self.workspace]);
        }

        Ok(())
    }

    fn get_clients(&self) -> Result<Vec<Value>, Box<dyn Error>> {
        let resp = hypr::message("clients", true)?;
        if let HyprResponse::Json(Value::Array(arr)) = resp {
            Ok(arr)
        } else {
            Ok(vec![])
        }
    }

    fn handle_client_config(
        &self,
        client: &ClientConfig,
        workspace: &str,
        _cache: Option<Vec<Value>>,
    ) -> Result<bool, Box<dyn Error>> {
        let clients = self.get_clients()?;

        let selector = |c: &Value| -> bool {
            if let Some(matches) = &client.matches {
                for rule in matches {
                    if is_subset(c, rule) {
                        return true;
                    }
                }
            }
            false
        };

        let mut spawned = false;

        if let Some(cmd_parts) = &client.command
            && !cmd_parts.is_empty()
        {
            let already_running = clients.iter().any(&selector);

            let cmd_executable = command_exists(&cmd_parts[0]);

            if cmd_executable && !already_running {
                let joined_args = shell_words::join(cmd_parts);
                let exec_arg = format!(
                    "[workspace special:{}] app2unit -- {}",
                    workspace, joined_args
                );

                hypr::dispatch("exec", &[&exec_arg]);
                spawned = true;
            }
        }

        if client.should_move.unwrap_or(false) {
            for c in &clients {
                if selector(c) {
                    let current_ws = c["workspace"]["name"].as_str().unwrap_or("");
                    let target_ws_name = format!("special:{}", workspace);

                    if current_ws != target_ws_name {
                        let address = c["address"].as_str().unwrap_or("");
                        let arg = format!("special:{},address:{}", workspace, address);
                        hypr::dispatch("movetoworkspacesilent", &[&arg]);
                    }
                }
            }
        }

        Ok(spawned)
    }

    fn toggle_special_ws(&self) -> Result<(), Box<dyn Error>> {
        let resp = hypr::message("monitors", true)?;

        let mut special_name = String::from("special");

        if let HyprResponse::Json(Value::Array(monitors)) = resp
            && let Some(focused) = monitors
                .iter()
                .find(|m| m["focused"].as_bool().unwrap_or(false))
            && let Some(sw) = focused
                .get("specialWorkspace")
                .and_then(|sw| sw.get("name"))
            && let Some(name) = sw.as_str()
            && name.len() > 8
        {
            special_name = name[8..].to_string();
        }

        hypr::dispatch("togglespecialworkspace", &[&special_name]);
        Ok(())
    }
}
