use serde::Serialize;
use sha2::{Digest, Sha256};
use std::env;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use tempfile::NamedTempFile;

#[derive(Debug, Clone)]
pub struct Paths {
    pub config_dir: PathBuf,
    pub data_dir: PathBuf,
    pub state_dir: PathBuf,
    pub cache_dir: PathBuf,
    pub pictures_dir: PathBuf,
    pub videos_dir: PathBuf,

    pub f_config_dir: PathBuf,
    pub f_data_dir: PathBuf,
    pub f_state_dir: PathBuf,
    pub f_cache_dir: PathBuf,

    pub user_config_path: PathBuf,
    pub cli_data_dir: PathBuf,
    pub templates_dir: PathBuf,
    pub user_templates_dir: PathBuf,
    pub theme_dir: PathBuf,

    pub scheme_path: PathBuf,
    pub scheme_data_dir: PathBuf,
    pub scheme_cache_dir: PathBuf,

    pub wallpapers_dir: PathBuf,
    pub wallpaper_path_path: PathBuf,
    pub wallpaper_link_path: PathBuf,
    pub wallpaper_thumbnail_path: PathBuf,
    pub wallpapers_cache_dir: PathBuf,

    pub screenshots_dir: PathBuf,
    pub screenshots_cache_dir: PathBuf,

    pub recordings_dir: PathBuf,
    pub recording_path: PathBuf,
    pub recording_notif_path: PathBuf,
}

impl Paths {
    pub fn new() -> Self {
        let home = env::var("HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("/"));

        let get_env_path = |key: &str, default: PathBuf| -> PathBuf {
            env::var(key).map(PathBuf::from).unwrap_or(default)
        };

        let config_dir = get_env_path("XDG_CONFIG_HOME", home.join(".config"));
        let data_dir = get_env_path("XDG_DATA_HOME", home.join(".local/share"));
        let state_dir = get_env_path("XDG_STATE_HOME", home.join(".local/state"));
        let cache_dir = get_env_path("XDG_CACHE_HOME", home.join(".cache"));
        let pictures_dir = get_env_path("XDG_PICTURES_DIR", home.join("Pictures"));
        let videos_dir = get_env_path("XDG_VIDEOS_DIR", home.join("Videos"));

        let f_config_dir = config_dir.join("ferret");
        let f_data_dir = data_dir.join("ferret");
        let f_state_dir = state_dir.join("ferret");
        let f_cache_dir = cache_dir.join("ferret");

        let user_config_path = f_config_dir.join("cli.json");

        let current_exe = env::current_exe().unwrap_or_else(|_| PathBuf::from("."));
        let cli_data_dir = current_exe
            .parent()
            .unwrap_or(Path::new("."))
            .parent()
            .unwrap_or(Path::new("."))
            .join("data");

        let templates_dir = cli_data_dir.join("templates");
        let user_templates_dir = f_config_dir.join("templates");
        let theme_dir = f_state_dir.join("theme");

        let scheme_path = f_state_dir.join("scheme.json");
        let scheme_data_dir = cli_data_dir.join("schemes");
        let scheme_cache_dir = f_cache_dir.join("schemes");

        let wallpapers_dir = get_env_path("FERRET_WALLPAPERS_DIR", pictures_dir.join("Wallpapers"));
        let wallpaper_path_path = f_state_dir.join("wallpaper/path.txt");
        let wallpaper_link_path = f_state_dir.join("wallpaper/current");
        let wallpaper_thumbnail_path = f_state_dir.join("wallpaper/thumbnail.jpg");
        let wallpapers_cache_dir = f_cache_dir.join("wallpapers");

        let screenshots_dir =
            get_env_path("FERRET_SCREENSHOTS_DIR", pictures_dir.join("Screenshots"));
        let screenshots_cache_dir = f_cache_dir.join("screenshots");

        let recordings_dir = get_env_path("FERRET_RECORDINGS_DIR", videos_dir.join("Recordings"));
        let recording_path = f_state_dir.join("record/recording.mp4");
        let recording_notif_path = f_state_dir.join("record/notifid.txt");

        Self {
            config_dir,
            data_dir,
            state_dir,
            cache_dir,
            pictures_dir,
            videos_dir,
            f_config_dir,
            f_data_dir,
            f_state_dir,
            f_cache_dir,
            user_config_path,
            cli_data_dir,
            templates_dir,
            user_templates_dir,
            theme_dir,
            scheme_path,
            scheme_data_dir,
            scheme_cache_dir,
            wallpapers_dir,
            wallpaper_path_path,
            wallpaper_link_path,
            wallpaper_thumbnail_path,
            wallpapers_cache_dir,
            screenshots_dir,
            screenshots_cache_dir,
            recordings_dir,
            recording_path,
            recording_notif_path,
        }
    }
}

pub fn compute_hash<P: AsRef<Path>>(path: P) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut hasher = Sha256::new();

    io::copy(&mut file, &mut hasher)?;

    let hash = hasher.finalize();
    Ok(hex::encode(hash))
}

pub fn atomic_dump<P: AsRef<Path>, T: Serialize>(path: P, content: &T) -> io::Result<()> {
    let path = path.as_ref();

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let dir = path.parent().unwrap_or_else(|| Path::new("."));
    let mut temp_file = NamedTempFile::new_in(dir)?;

    serde_json::to_writer(&mut temp_file, content)?;

    temp_file.flush()?;

    temp_file.persist(path).map_err(|e| e.error)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_compute_hash_basic() {
        let mut temp_file = tempfile::NamedTempFile::new().unwrap();

        write!(temp_file, "hello world").unwrap();

        let hash = compute_hash(temp_file.path()).expect("Fallo al calcular hash");

        assert_eq!(
            hash,
            "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"
        );
    }

    #[test]
    fn test_compute_hash_empty_file() {
        let temp_file = tempfile::NamedTempFile::new().unwrap();
        let hash = compute_hash(temp_file.path()).unwrap();

        assert_eq!(
            hash,
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }

    #[test]
    fn test_atomic_dump_creates_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test_config.json");

        let data = serde_json::json!({
            "username": "ferret_user",
            "level": 9000
        });

        atomic_dump(&file_path, &data).expect("Fallo atomic_dump");

        assert!(file_path.exists());

        let content = std::fs::read_to_string(file_path).unwrap();
        let json_content: serde_json::Value = serde_json::from_str(&content).unwrap();

        assert_eq!(json_content["username"], "ferret_user");
        assert_eq!(json_content["level"], 9000);
    }

    #[test]
    fn test_atomic_dump_creates_nested_directories() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("nested").join("deeply").join("config.json");

        let data = serde_json::json!({"valid": true});

        atomic_dump(&file_path, &data).expect("Debería crear directorios padres");

        assert!(file_path.exists());
    }

    #[test]
    fn test_app_paths_structure() {
        let temp_home = tempdir().unwrap();
        unsafe {
            std::env::set_var("XDG_CONFIG_HOME", temp_home.path().join(".config"));
        }

        let paths = Paths::new();

        assert!(paths.f_config_dir.ends_with("caelestia"));
        assert!(paths.user_config_path.ends_with("cli.json"));

        unsafe {
            std::env::remove_var("XDG_CONFIG_HOME");
        }
    }
}
