use clap::Args;
use std::error::Error;
use serde_json::to_string_pretty;

use crate::utils::paths::AppPaths;
use crate::utils::wallpaper; // Importamos el módulo de utilidad
use super::Runnable;

#[derive(Args, Debug)]
pub struct WallpaperCmd {
    /// Print colors for a specific wallpaper file
    #[arg(short, long)]
    pub print: Option<String>,

    /// Set a specific file as wallpaper
    #[arg(short, long)]
    pub file: Option<String>,

    /// Set a random wallpaper
    #[arg(short, long)]
    pub random: bool,

    /// Disable smart color generation
    #[arg(long)]
    pub no_smart: bool,
}

impl Runnable<&AppPaths> for WallpaperCmd {
    fn run(&self, paths: &AppPaths) -> Result<(), Box<dyn Error>> {
        match self {
            // Caso 1: Print (--print <FILE>)
            cmd if cmd.print.is_some() => {
                let path = cmd.print.as_ref().unwrap();
                let colors = wallpaper::get_colours_for_wall(path, cmd.no_smart);
                // Imprimimos el JSON formateado
                println!("{}", to_string_pretty(&colors)?);
            }

            // Caso 2: Set File (--file <FILE>)
            cmd if cmd.file.is_some() => {
                let path = cmd.file.as_ref().unwrap();
                // Pasamos 'paths' porque set_wallpaper probablemente necesite guardar estado
                wallpaper::set_wallpaper(path, cmd.no_smart, paths)?;
            }

            // Caso 3: Random (--random)
            cmd if cmd.random => {
                // set_random necesita 'paths' para saber dónde está la carpeta de wallpapers
                wallpaper::set_random(paths)?;
            }

            // Caso 4: Default (Mostrar actual)
            _ => {
                match wallpaper::get_wallpaper(paths) {
                    Some(wall) => println!("{}", wall.trim()),
                    None => println!("No wallpaper set"),
                }
            }
        }

        Ok(())
    }
}
