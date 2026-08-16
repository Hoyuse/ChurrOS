// ==========================================
// ThemeService — dark/light + gtk settings + señales a waybar/foot
// (equivalente a services/theme.py)
// ==========================================

use std::cell::RefCell;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

use serde_json::json;

use crate::services::settings;

// Listeners de cambio de tema (dark/light). El patrón es el mismo que
// Search::connect_search: los registra la ventana y ThemeService::set los
// invoca al final para refrescar el tema en vivo sin depender de gsettings/
// dconf/DBus. Todo corre en el hilo principal de GTK.
thread_local! {
    static THEME_LISTENERS: RefCell<Vec<Box<dyn Fn(bool)>>> = RefCell::new(Vec::new());
}

fn cache_dir() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/root".to_string());
    PathBuf::from(home).join(".cache").join("churros-theme")
}

fn dark_flag() -> PathBuf {
    cache_dir().join("dark-flag")
}

fn build_env() -> Vec<(String, String)> {
    let mut env: Vec<(String, String)> = std::env::vars().collect();
    if env.iter().all(|(k, _)| k != "WAYLAND_DISPLAY") {
        let uid = unsafe { libc_getuid() };
        let xrd = format!("/run/user/{uid}");
        if std::path::Path::new(&xrd).is_dir() {
            if let Ok(entries) = fs::read_dir(&xrd) {
                for entry in entries.flatten() {
                    let name = entry.file_name().to_string_lossy().to_string();
                    if name.starts_with("wayland-") {
                        env.push(("WAYLAND_DISPLAY".to_string(), name));
                        break;
                    }
                }
            }
        }
    }
    if env.iter().all(|(k, _)| k != "XDG_RUNTIME_DIR") {
        let uid = unsafe { libc_getuid() };
        env.push(("XDG_RUNTIME_DIR".to_string(), format!("/run/user/{uid}")));
    }
    env
}

// getuid sin dependencia libc: leer /proc/self/status o usar el uid del
// propietario del proceso via /proc/self (mejor: std no expone uid).
// Se usa el crate libc indirectamente a través de glib? No — lo resolvemos
// leyendo /proc/self/loginuid o simplemente el uid del archivo /proc/self.
fn libc_getuid() -> u32 {
    // Lectura de /proc/self/status línea Uid:
    if let Ok(content) = fs::read_to_string("/proc/self/status") {
        for line in content.lines() {
            if let Some(rest) = line.strip_prefix("Uid:") {
                if let Some(first) = rest.split_whitespace().next() {
                    if let Ok(uid) = first.parse::<u32>() {
                        return uid;
                    }
                }
            }
        }
    }
    1000
}

/// Actualiza SOLO las claves de tema en el ini, preservando el resto
/// (cursor, fuente, etc.). El Python sobrescribía el archivo entero y
/// perdía las claves que escriben otras páginas (p. ej. cursor.rs).
fn update_ini_key(ini: &PathBuf, key: &str, value: &str) {
    if let Some(parent) = ini.parent() {
        let _ = fs::create_dir_all(parent);
    }
    let existing = fs::read_to_string(ini).unwrap_or_default();
    let mut lines: Vec<String> = existing.lines().map(|s| s.to_string()).collect();

    let mut seen = false;
    for line in lines.iter_mut() {
        if line.trim_start().starts_with(&format!("{key}=")) {
            *line = format!("{key}={value}");
            seen = true;
        }
    }
    if !seen {
        lines.push(format!("{key}={value}"));
    }

    let joined = lines.join("\n");
    let content = if joined.contains("[Settings]") {
        joined
    } else {
        format!("[Settings]\n{joined}")
    };
    let _ = fs::write(ini, content + "\n");
}

fn write_gtk_settings(dark: bool) {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/root".to_string());
    let icon_theme = settings::get_string("icons.theme", if dark { "Papirus-Dark" } else { "Papirus" });
    let gtk_theme = if dark { "Adwaita-dark" } else { "Adwaita" };

    for dir in ["gtk-3.0", "gtk-4.0"] {
        let ini = PathBuf::from(&home).join(".config").join(dir).join("settings.ini");
        update_ini_key(&ini, "gtk-theme-name", gtk_theme);
        update_ini_key(
            &ini,
            "gtk-application-prefer-dark-theme",
            if dark { "1" } else { "0" },
        );
        update_ini_key(&ini, "gtk-icon-theme-name", &icon_theme);
    }

    // Mecanismo en vivo de GTK4: color-scheme via gsettings (el portal de
    // settings lo propaga y el listener de window.rs refresca la clase CSS
    // "light" sin reiniciar la app). El gtk-theme-name de settings.ini solo
    // aplica al arrancar; cambiarlo en caliente puede re-tematizar a medias.
    let _ = Command::new("gsettings")
        .args([
            "set",
            "org.gnome.desktop.interface",
            "color-scheme",
            if dark { "prefer-dark" } else { "default" },
        ])
        .output();
    let _ = Command::new("gsettings")
        .args([
            "set",
            "org.gnome.desktop.interface",
            "gtk-theme",
            gtk_theme,
        ])
        .output();

    // Flag en caché para que is_dark() sea rápido sin leer settings.json
    if let Some(parent) = dark_flag().parent() {
        let _ = fs::create_dir_all(parent);
    }
    let _ = fs::write(dark_flag(), if dark { "1" } else { "0" });
}

pub struct ThemeService;

impl ThemeService {
    pub fn is_dark() -> bool {
        if let Ok(content) = fs::read_to_string(dark_flag()) {
            return content.trim() == "1";
        }
        if let Some(cached) = settings::get("theme.dark", json!(null)).as_bool() {
            return cached;
        }
        true
    }

    pub fn set(dark: bool) {
        settings::set("theme.dark", json!(dark));
        write_gtk_settings(dark);

        let env = build_env();

        // waybar: SIGUSR2 recarga; foot: SIGUSR1 dark, SIGUSR2 light
        let env_refs: Vec<(&str, &str)> = env
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect();
        let _ = Command::new("pkill")
            .args(["-SIGUSR2", "waybar"])
            .envs(env_refs.iter().map(|(k, v)| (*k, *v)))
            .output();
        let _ = Command::new("pkill")
            .args([if dark { "-SIGUSR1" } else { "-SIGUSR2" }, "foot"])
            .envs(env_refs.iter().map(|(k, v)| (*k, *v)))
            .output();

        // TODO: pywal integration (services/pywal_service.py)

        // Colores dinámicos: regenerar paleta si pywal está activo (paridad
        // con theme.py del Python).
        if crate::services::pywal::PywalService::enabled() {
            let _ = crate::services::pywal::PywalService::regenerate_if_enabled();
        }

        // Notificar en vivo a la ventana (refresh_theme) sin round-trip
        // externo: mismo hook que _refresh_root_theme() del Python.
        Self::notify(dark);
    }

    /// Registra un callback que se invoca en cada cambio de tema.
    pub fn on_change(cb: impl Fn(bool) + 'static) {
        THEME_LISTENERS.with(|l| l.borrow_mut().push(Box::new(cb)));
    }

    fn notify(dark: bool) {
        THEME_LISTENERS.with(|l| {
            for cb in l.borrow().iter() {
                cb(dark);
            }
        });
    }

    pub fn toggle() {
        Self::set(!Self::is_dark());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn on_change_notifies_listeners() {
        let received = Rc::new(RefCell::new(Vec::new()));
        let r1 = received.clone();
        ThemeService::on_change(move |dark| r1.borrow_mut().push(dark));
        let r2 = received.clone();
        ThemeService::on_change(move |dark| r2.borrow_mut().push(dark));

        ThemeService::notify(true);
        ThemeService::notify(false);

        assert_eq!(*received.borrow(), vec![true, true, false, false]);
    }
}
