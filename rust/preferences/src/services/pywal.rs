// ==========================================
// PywalService — colores dinámicos desde el wallpaper
// (equivalente a services/pywal_service.py)
// ==========================================

use std::fs;
use std::path::{Path, PathBuf};

use serde_json::{json, Value};

use crate::services::accent::AccentService;
use crate::services::settings;
use crate::services::wallpaper::WallpaperService;
use crate::services::waybar::WaybarService;

pub struct PywalService;

fn home() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/root".to_string());
    PathBuf::from(home)
}

fn cache_file() -> PathBuf {
    home().join(".cache").join("wal").join("colors.json")
}

fn which(name: &str) -> bool {
    let path_var = std::env::var("PATH").unwrap_or_default();
    path_var
        .split(':')
        .any(|dir| Path::new(dir).join(name).is_file())
}

impl PywalService {
    pub fn available() -> bool {
        which("wal")
    }

    pub fn enabled() -> bool {
        settings::get_bool("theme.dynamic_colors", false)
    }

    fn current_wallpaper() -> Option<String> {
        let path = WallpaperService::current();
        if !path.is_empty() && Path::new(&path).is_file() {
            return Some(path);
        }
        let default = "/usr/share/churros/wallpapers/default.jpeg";
        if Path::new(default).is_file() {
            return Some(default.to_string());
        }
        None
    }

    /// Corre `wal -i <wallpaper>` y devuelve la paleta (colors.json) o None.
    pub fn generate() -> Option<Value> {
        let wallpaper = Self::current_wallpaper()?;
        if !Self::available() {
            return None;
        }
        let _ = std::process::Command::new("wal")
            .args(["-q", "-i", &wallpaper, "-n", "-e"])
            .status();
        Self::read_cache()
    }

    fn read_cache() -> Option<Value> {
        let raw = fs::read_to_string(cache_file()).ok()?;
        serde_json::from_str(&raw).ok()
    }

    /// Aplica la paleta: accent GTK (accent.css) + colores de waybar.
    pub fn apply_accent(palette: &Value) -> bool {
        let colors = palette.get("colors").and_then(|c| c.as_object());
        let specials = palette.get("special").and_then(|c| c.as_object());

        let get_color = |name: &str| -> Option<&str> {
            colors
                .and_then(|c| c.get(name))
                .and_then(|v| v.as_str())
        };
        let get_special = |name: &str| -> Option<&str> {
            specials
                .and_then(|s| s.get(name))
                .and_then(|v| v.as_str())
        };

        // Acento: color vivo de la paleta (color1 -> color4 -> foreground)
        let accent = get_color("color1")
            .or_else(|| get_color("color4"))
            .or_else(|| get_special("foreground"))
            .unwrap_or("#DE8636");
        let bg = get_special("background").unwrap_or("#111827");
        let fg = get_special("foreground").unwrap_or("#F8FAFC");

        // Acento GTK (accent.css)
        AccentService::set_hex(accent);

        // Waybar: colors-waybar.css + recarga
        WaybarService::apply_pywal_colors(bg, fg, accent);

        true
    }

    /// Habilita/deshabilita los colores dinámicos.
    pub fn toggle(value: bool) -> bool {
        if value {
            Self::enable()
        } else {
            settings::set("theme.dynamic_colors", json!(false));
            // Volver al color guardado por nombre
            AccentService::set(&AccentService::current());
            true
        }
    }

    fn enable() -> bool {
        settings::set("theme.dynamic_colors", json!(true));
        let Some(palette) = Self::generate() else {
            return false;
        };
        Self::apply_accent(&palette)
    }

    /// Hook de WallpaperService.set: si los colores dinámicos están activos,
    /// regenera la paleta desde el wallpaper y la aplica.
    pub fn regenerate_if_enabled() -> bool {
        if !Self::enabled() {
            return false;
        }
        let Some(palette) = Self::generate() else {
            return false;
        };
        Self::apply_accent(&palette)
    }
}
