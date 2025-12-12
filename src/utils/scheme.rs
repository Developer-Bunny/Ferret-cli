use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::error::Error;
use std::fmt;
use std::io::Write;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};

// --- MOCK IMPORTS (Suponiendo que estas existen en tu proyecto) ---
// use crate::utils::notify::notify;
// use crate::utils::paths::{atomic_dump, scheme_data_dir, scheme_path};
// use crate::utils::material::get_colours_for_image;

// Para que el código compile aquí, definiré stubs de lo que viene de fuera:
mod utils_mock {
    use std::path::PathBuf;
    pub fn scheme_data_dir() -> PathBuf { PathBuf::from("schemes") } // Ejemplo
    pub fn scheme_path() -> PathBuf { PathBuf::from("current_scheme.json") } // Ejemplo
    pub fn atomic_dump(path: &std::path::Path, data: &str) -> std::io::Result<()> {
        std::fs::write(path, data)
    }
    pub fn notify(_u: &str, _urgency: &str, title: &str, body: &str) {
        eprintln!("NOTIFY [{}]: {} - {}", _urgency, title, body);
    }
}
use utils_mock::*;

// --- CONSTANTS ---
pub const SCHEME_VARIANTS: &[&str] = &[
    "tonalspot",
    "vibrant",
    "expressive",
    "fidelity",
    "fruitsalad",
    "monochrome",
    "neutral",
    "rainbow",
    "content",
];

// --- STRUCT DEFINITION ---

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Scheme {
    // Usamos getters/setters manuales, así que los campos son privados
    #[serde(rename = "name")]
    _name: String,
    #[serde(rename = "flavour")]
    _flavour: String,
    #[serde(rename = "mode")]
    _mode: String,
    #[serde(rename = "variant")]
    _variant: String,
    #[serde(rename = "colours")]
    _colours: HashMap<String, String>,
    #[serde(rename = "default")]
    _default: bool,
    
    #[serde(skip)]
    pub notify: bool,
}

impl Scheme {
    /// Constructor por defecto ("catppuccin", "mocha", etc.)
    pub fn default_scheme() -> Result<Self, Box<dyn Error>> {
        let mut scheme = Scheme {
            _name: "catppuccin".to_string(),
            _flavour: "mocha".to_string(),
            _mode: "dark".to_string(),
            _variant: "tonalspot".to_string(),
            _colours: HashMap::new(),
            _default: true,
            notify: false,
        };
        // Cargar colores iniciales
        scheme._colours = scheme.read_colours_from_file(&scheme.get_colours_path())?;
        Ok(scheme)
    }

    /// Constructor desde JSON (equivalente a __init__ con dict)
    pub fn from_json(json_str: &str) -> Result<Self, Box<dyn Error>> {
        let mut scheme: Scheme = serde_json::from_str(json_str)?;
        scheme._default = false;
        scheme.notify = false;
        Ok(scheme)
    }

    // --- GETTERS ---
    pub fn name(&self) -> &str { &self._name }
    pub fn flavour(&self) -> &str { &self._flavour }
    pub fn mode(&self) -> &str { &self._mode }
    pub fn variant(&self) -> &str { &self._variant }
    pub fn colours(&self) -> &HashMap<String, String> { &self._colours }
    pub fn is_default(&self) -> bool { self._default }

    // --- SETTERS ---
    
    pub fn set_name(&mut self, name: &str) -> Result<(), Box<dyn Error>> {
        if name == self._name { return Ok(()); }

        let valid_names = get_scheme_names();
        if !valid_names.contains(&name.to_string()) {
            if self.notify {
                notify("-u", "critical", "Unable to set scheme", 
                    &format!("\"{}\" is not a valid scheme.\nValid schemes are: {:?}", name, valid_names));
            }
            return Err(format!("Invalid scheme name: {}", name).into());
        }

        self._name = name.to_string();
        self._check_flavour();
        self._check_mode();
        self._update_colours()?;
        self.save()
    }

    pub fn set_flavour(&mut self, flavour: &str) -> Result<(), Box<dyn Error>> {
        if flavour == self._flavour { return Ok(()); }

        let valid_flavours = get_scheme_flavours(&self._name);
        if !valid_flavours.contains(&flavour.to_string()) {
            if self.notify {
                notify("-u", "critical", "Unable to set scheme flavour",
                    &format!("\"{}\" is not a valid flavour of scheme \"{}\".\nValid flavours are: {:?}", flavour, self._name, valid_flavours));
            }
            return Err(format!("Invalid scheme flavour: \"{}\". Valid flavours: {:?}", flavour, valid_flavours).into());
        }

        self._flavour = flavour.to_string();
        self._check_mode();
        self.update_colours()
    }

    pub fn set_mode(&mut self, mode: &str) -> Result<(), Box<dyn Error>> {
        if mode == self._mode { return Ok(()); }

        let valid_modes = get_scheme_modes(&self._name, &self._flavour);
        if !valid_modes.contains(&mode.to_string()) {
            if self.notify {
                notify("-u", "critical", "Unable to set scheme mode",
                    &format!("Scheme \"{} {}\" does not have a {} mode.", self._name, self._flavour, mode));
            }
            return Err(format!("Invalid scheme mode: \"{}\". Valid modes: {:?}", mode, valid_modes).into());
        }

        self._mode = mode.to_string();
        self.update_colours()
    }

    pub fn set_variant(&mut self, variant: &str) -> Result<(), Box<dyn Error>> {
        if variant == self._variant { return Ok(()); }
        self._variant = variant.to_string();
        self.update_colours()
    }

    pub fn set_default(&mut self, state: bool) -> Result<(), Box<dyn Error>> {
        if state == self._default { return Ok(()); }
        self._default = state;
        self.save()
    }

    // --- METHODS ---

    pub fn get_colours_path(&self) -> PathBuf {
        scheme_data_dir()
            .join(&self._name)
            .join(&self._flavour)
            .join(&self._mode)
            .with_extension("txt")
    }

    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        if let Some(parent) = scheme_path().parent() {
            fs::create_dir_all(parent)?;
        }
        let json = serde_json::to_string_pretty(self)?;
        atomic_dump(&scheme_path(), &json)?;
        Ok(())
    }

    pub fn set_random(&mut self) -> Result<(), Box<dyn Error>> {
        let mut rng = rand::thread_rng();
        
        let names = get_scheme_names();
        if let Some(n) = names.choose(&mut rng) {
            self._name = n.clone();
        }

        let flavours = get_scheme_flavours(&self._name);
        if let Some(f) = flavours.choose(&mut rng) {
            self._flavour = f.clone();
        }

        let modes = get_scheme_modes(&self._name, &self._flavour);
        if let Some(m) = modes.choose(&mut rng) {
            self._mode = m.clone();
        }

        self.update_colours()
    }

    pub fn update_colours(&mut self) -> Result<(), Box<dyn Error>> {
        self._update_colours()?;
        self.save()
    }

    // --- INTERNAL HELPERS ---

    fn _check_flavour(&mut self) {
        let flavours = get_scheme_flavours(&self._name);
        if !flavours.contains(&self._flavour) {
            if let Some(first) = flavours.first() {
                self._flavour = first.clone();
            }
        }
    }

    fn _check_mode(&mut self) {
        let modes = get_scheme_modes(&self._name, &self._flavour);
        if !modes.contains(&self._mode) {
            if let Some(first) = modes.first() {
                self._mode = first.clone();
            }
        }
    }

    fn _update_colours(&mut self) -> Result<(), Box<dyn Error>> {
        if self._name == "dynamic" {
            // Aquí iría la lógica para llamar a tu función de Rust que genera colores desde imagen
            // Como referencia a lo anterior, aquí invocarías la lógica de gen_scheme() o similar.
            
            // SIMULACIÓN:
            // use crate::utils::material::get_colours_for_image;
            // self._colours = get_colours_for_image().map_err(|_| ...)?;
            
            // Por ahora lanzamos error simulado si falla la lógica:
            let wallpaper_exists = true; // Simular comprobación
            if !wallpaper_exists {
                 if self.notify {
                    notify("-u", "critical", "Unable to set dynamic scheme", "No wallpaper set...");
                 }
                 return Err("No wallpaper set".into());
            }
            // self._colours = ... (Resultado de la generación dinámica)
        } else {
            let path = self.get_colours_path();
            match self.read_colours_from_file(&path) {
                Ok(c) => self._colours = c,
                Err(e) => return Err(format!("Could not read colours from {:?}: {}", path, e).into()),
            }
        }
        Ok(())
    }

    fn read_colours_from_file(&self, path: &Path) -> Result<HashMap<String, String>, Box<dyn Error>> {
        let content = fs::read_to_string(path)?;
        let mut colours = HashMap::new();
        for line in content.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                colours.insert(parts[0].to_string(), parts[1].to_string());
            }
        }
        Ok(colours)
    }
}

// --- DISPLAY IMPLEMENTATION ---

impl fmt::Display for Scheme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Current scheme:")?;
        writeln!(f, "    Name: {}", self._name)?;
        writeln!(f, "    Flavour: {}", self._flavour)?;
        writeln!(f, "    Mode: {}", self._mode)?;
        writeln!(f, "    Variant: {}", self._variant)?;
        writeln!(f, "    Colours:")?;

        for (name, color_hex) in &self._colours {
            // Parse HEX string (RRGGBB) to integers for ANSI output
            // Asume que color_hex viene limpio (ej: "FF0000" o "FF0000FF")
            // El Python original hace slicing [0:2], [2:4], [4:6]
            let r = u8::from_str_radix(&color_hex[0..2], 16).unwrap_or(0);
            let g = u8::from_str_radix(&color_hex[2..4], 16).unwrap_or(0);
            let b = u8::from_str_radix(&color_hex[4..6], 16).unwrap_or(0);

            // ANSI escape sequence: \x1b[38;2;R;G;Bm
            writeln!(
                f,
                "        {}: \x1b[38;2;{};{};{}m{}\x1b[0m",
                name, r, g, b, color_hex
            )?;
        }
        Ok(())
    }
}

// --- GLOBAL FUNCTIONS ---

pub fn get_scheme_path_fn() -> PathBuf {
    // Nota: en Rust no podemos llamar a get_scheme() global fácilmente sin Singleton/Mutex
    // Aquí implementamos la lógica suponiendo que instanciamos un scheme
    if let Ok(scheme) = get_scheme() {
        scheme.get_colours_path()
    } else {
        PathBuf::new() // Fallback
    }
}

/// Función equivalente a get_scheme() en Python.
/// En Rust, esto debería cargar desde disco cada vez o usar un Lazy/OnceCell global.
pub fn get_scheme() -> Result<Scheme, Box<dyn Error>> {
    let path = scheme_path();
    
    // Intentar leer y parsear JSON
    if let Ok(content) = fs::read_to_string(&path) {
        if let Ok(scheme) = Scheme::from_json(&content) {
            return Ok(scheme);
        }
    }

    // Fallback si falla la lectura o parseo
    let scheme = Scheme::default_scheme()?;
    scheme.save()?;
    Ok(scheme)
}

pub fn get_scheme_names() -> Vec<String> {
    let mut names = Vec::new();
    if let Ok(entries) = fs::read_dir(scheme_data_dir()) {
        for entry in entries.flatten() {
            if let Ok(ft) = entry.file_type() {
                if ft.is_dir() {
                    if let Ok(name) = entry.file_name().into_string() {
                        names.push(name);
                    }
                }
            }
        }
    }
    names.push("dynamic".to_string());
    names
}

pub fn get_scheme_flavours(name: &str) -> Vec<String> {
    let target_name = if name.is_empty() {
        // En Rust esto es complejo si no pasamos el scheme instance.
        // Asumimos que el caller pasa el nombre correcto o leemos de disco.
        if let Ok(s) = get_scheme() { s.name().to_string() } else { return vec![] }
    } else {
        name.to_string()
    };

    if target_name == "dynamic" {
        return vec!["default".to_string()];
    }

    let mut flavours = Vec::new();
    let path = scheme_data_dir().join(target_name);
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            if let Ok(ft) = entry.file_type() {
                if ft.is_dir() {
                    if let Ok(n) = entry.file_name().into_string() {
                        flavours.push(n);
                    }
                }
            }
        }
    }
    flavours
}

pub fn get_scheme_modes(name: &str, flavour: &str) -> Vec<String> {
    let (target_name, target_flavour) = if name.is_empty() {
        if let Ok(s) = get_scheme() { 
            (s.name().to_string(), s.flavour().to_string()) 
        } else { 
            return vec![] 
        }
    } else {
        (name.to_string(), flavour.to_string())
    };

    if target_name == "dynamic" {
        return vec!["light".to_string(), "dark".to_string()];
    }

    let mut modes = Vec::new();
    let path = scheme_data_dir().join(target_name).join(target_flavour);
    
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            if let Ok(ft) = entry.file_type() {
                if ft.is_file() {
                    let path_buf = entry.path();
                    if let Some(stem) = path_buf.file_stem() {
                        if let Some(stem_str) = stem.to_str() {
                            modes.push(stem_str.to_string());
                        }
                    }
                }
            }
        }
    }
    modes
}
