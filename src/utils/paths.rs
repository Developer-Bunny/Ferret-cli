use anyhow::{Context, Result};
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::env;
use std::fs::{self, File};
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use tempfile::NamedTemporaryFile;

#[derive(Debug, Clone)]
pub struct FerretPaths {
    pub config_dir: PathBuf,
    pub data_dir: PathBuf,
    pub state_dir: PathBuf,
    pub cache_dir: PathBuf,

    pub c_config_dir: PathBuf,
    pub c_data_dir: PathBuf,
    pub c_state_dir: PathBuf,
    pub c_cache_dir: PathBuf,

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

impl FerretPaths {
    pub fn new() -> Result<Self> {
        let home = dirs::home_dir().context("No se pudo encontrar el directorio home")?;

        let get_path = |env_var: &str, default: PathBuf| -> PathBuf {
            env::var(env_var).map(PathBuf::from).unwrap_or(default)
        };

        let config_dir = get_path("XDG_CONFIG_HOME", home.join(".config"));
        let data_dir = get_path("XDG_DATA_HOME", home.join(".local/share"));
        let state_dir = get_path("XDG_STATE_HOME", home.join(".local/state"));
        let cache_dir = get_path("XDG_CACHE_HOME", home.join(".cache"));
        let pictures_dir = get_path("XDG_PICTURES_DIR", home.join("Pictures"));
        let videos_dir = get_path("XDG_VIDEOS_DIR", home.join("Videos"));

        let c_config_dir = config_dir.join("ferret");
        let c_data_dir = data_dir.join("ferret");
        let c_state_dir = state_dir.join("ferret");
        let c_cache_dir = cache_dir.join("ferret");

        let user_config_path = c_config_dir.join("cli.json");

        let current_exe = env::current_exe()?;
        let cli_data_dir = current_exe.parent().unwrap().parent().unwrap().join("data");

        let templates_dir = cli_data_dir.join("templates");
        let user_templates_dir = c_config_dir.join("templates");
        let theme_dir = c_state_dir.join("theme");

        let scheme_path = c_state_dir.join("scheme.json");
        let scheme_data_dir = cli_data_dir.join("schemes");
        let scheme_cache_dir = c_cache_dir.join("schemes");

        let wallpapers_dir = get_path("FERRET_WALLPAPERS_DIR", pictures_dir.join("Wallpapers"));
        let wallpaper_path_path = c_state_dir.join("wallpaper/path.txt");
        let wallpaper_link_path = c_state_dir.join("wallpaper/current");
        let wallpaper_thumbnail_path = c_state_dir.join("wallpaper/thumbnail.jpg");
        let wallpapers_cache_dir = c_cache_dir.join("wallpapers");

        let screenshots_dir = get_path("FERRET_SCREENSHOTS_DIR", pictures_dir.join("Screenshots"));
        let screenshots_cache_dir = c_cache_dir.join("screenshots");

        let recordings_dir = get_path("FERRET_RECORDINGS_DIR", videos_dir.join("Recordings"));
        let recording_path = c_state_dir.join("record/recording.mp4");
        let recording_notif_path = c_state_dir.join("record/notifid.txt");

        Ok(Self {
            config_dir,
            data_dir,
            state_dir,
            cache_dir,
            c_config_dir,
            c_data_dir,
            c_state_dir,
            c_cache_dir,
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
        })
    }
}

pub fn compute_hash<P: AsRef<Path>>(path: P) -> Result<String> {
    let mut file = File::open(path)?;
    let mut hasher = Sha256::new();
    io::copy(&mut file, &mut hasher)?;
    Ok(hex::encode(hasher.finalize()))
}

pub fn atomic_dump<P: AsRef<Path>, T: Serialize>(path: P, content: &T) -> Result<()> {
    let path = path.as_ref();

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let parent_dir = path.parent().unwrap_or_else(|| Path::new("."));
    let temp_file = NamedTemporaryFile::new_in(parent_dir)?;

    serde_json::to_writer_pretty(&temp_file, content)?;

    temp_file.persist(path).map_err(|e| e.error)?;

    Ok(())
}
