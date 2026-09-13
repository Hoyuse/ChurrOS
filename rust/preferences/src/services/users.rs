// ==========================================
// UsersService — cuenta del sistema y autologin de greetd / LightDM
// ==========================================

use std::fs;
use std::process::Command;

pub struct UsersService;

const GREETD_CONFIG_PATH: &str = "/etc/greetd/config.toml";
const REGREET_CONFIG_PATH: &str = "/etc/greetd/regreet.toml";
const LIGHTDM_AUTOLOGIN_PATH: &str = "/etc/lightdm/lightdm.conf.d/autologin.conf";
const LIGHTDM_MAIN_PATH: &str = "/etc/lightdm/lightdm.conf";

fn getuid() -> u32 {
    Command::new("id")
        .arg("-u")
        .output()
        .ok()
        .and_then(|o| String::from_utf8_lossy(&o.stdout).trim().parse().ok())
        .unwrap_or(0)
}

/// Campos de /etc/passwd para el uid actual: (name, uid, gid, gecos, shell)
fn passwd_entry() -> Option<(String, String, String, String, String)> {
    let content = fs::read_to_string("/etc/passwd").ok()?;
    let uid = getuid().to_string();
    for line in content.lines() {
        let fields: Vec<&str> = line.split(':').collect();
        if fields.len() >= 7 && fields[2] == uid {
            return Some((
                fields[0].to_string(),
                fields[2].to_string(),
                fields[3].to_string(),
                fields[4].to_string(),
                fields[6].to_string(),
            ));
        }
    }
    None
}

/// ¿La línea inicia una sección `[nombre]`? Devuelve el nombre.
fn section_name(line: &str) -> Option<&str> {
    let t = line.trim();
    if t.starts_with('[') && t.ends_with(']') {
        Some(&t[1..t.len() - 1])
    } else {
        None
    }
}

/// Verifica si greetd tiene `[initial_session]` configurado
fn check_greetd_autologin() -> bool {
    let Ok(content) = fs::read_to_string(GREETD_CONFIG_PATH) else {
        return false;
    };
    let mut in_initial = false;
    for line in content.lines() {
        let trimmed = line.trim();
        if let Some(sec) = section_name(trimmed) {
            in_initial = sec.eq_ignore_ascii_case("initial_session");
            continue;
        }
        if in_initial && trimmed.starts_with("user") {
            if let Some((_, val)) = trimmed.split_once('=') {
                let user_val = val.trim().trim_matches('"').trim_matches('\'');
                if !user_val.is_empty() {
                    return true;
                }
            }
        }
    }
    false
}

/// Verifica si un archivo INI tiene `autologin-user=<usuario>` no vacío
fn check_ini_autologin(path: &str) -> bool {
    let Ok(content) = fs::read_to_string(path) else {
        return false;
    };
    let mut in_seat = false;
    for line in content.lines() {
        let trimmed = line.trim();
        if let Some(sec) = section_name(trimmed) {
            in_seat = sec.to_lowercase().starts_with("seat");
            continue;
        }
        if (in_seat || !trimmed.starts_with('[')) && trimmed.to_lowercase().starts_with("autologin-user") {
            if let Some((key, val)) = trimmed.split_once('=') {
                if key.trim().eq_ignore_ascii_case("autologin-user") {
                    let user_val = val.trim();
                    if !user_val.is_empty() && user_val != "false" {
                        return true;
                    }
                }
            }
        }
    }
    false
}

impl UsersService {
    pub fn username() -> String {
        std::env::var("USER").unwrap_or_else(|_| {
            passwd_entry()
                .map(|(name, _, _, _, _)| name)
                .unwrap_or_else(|| "Desconocido".to_string())
        })
    }

    pub fn full_name() -> String {
        if let Some((_, _, _, gecos, _)) = passwd_entry() {
            let name = gecos.split(',').next().unwrap_or("").trim();
            if !name.is_empty() {
                return name.to_string();
            }
        }
        Self::username()
    }

    pub fn home() -> String {
        std::env::var("HOME").unwrap_or_default()
    }

    pub fn shell() -> String {
        passwd_entry()
            .map(|(_, _, _, _, shell)| shell)
            .unwrap_or_else(|| "Desconocido".to_string())
    }

    pub fn uid() -> String {
        getuid().to_string()
    }

    pub fn gid() -> String {
        Command::new("id")
            .arg("-g")
            .output()
            .ok()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| "0".to_string())
    }

    pub fn hostname() -> String {
        fs::read_to_string("/proc/sys/kernel/hostname")
            .map(|s| s.trim().to_string())
            .unwrap_or_else(|_| "Desconocido".to_string())
    }

    /// ¿Hay autologin configurado en greetd o LightDM?
    pub fn auto_login() -> bool {
        if std::path::Path::new(GREETD_CONFIG_PATH).exists() {
            check_greetd_autologin()
        } else {
            check_ini_autologin(LIGHTDM_AUTOLOGIN_PATH) || check_ini_autologin(LIGHTDM_MAIN_PATH)
        }
    }

    /// Activa/desactiva el autologin editando la configuración del Display Manager activo
    pub fn set_auto_login(value: bool) -> bool {
        let current = Self::auto_login();
        if value == current {
            return true;
        }

        let user = Self::username();
        let desktop = churros_services::version::edition();
        let session_cmd = if desktop.contains("xfce") {
            "startxfce4"
        } else {
            "niri"
        };

        // Si greetd está disponible o en uso:
        if std::path::Path::new("/etc/greetd").exists() || std::path::Path::new(GREETD_CONFIG_PATH).exists() {
            let new_content = if value {
                format!(
                    "[terminal]\nvt = 7\n\n[default_session]\ncommand = \"env WLR_NO_HARDWARE_CURSORS=1 XCURSOR_THEME=Adwaita XCURSOR_SIZE=24 cage -s -- regreet\"\nuser = \"greeter\"\n\n[initial_session]\ncommand = \"{session_cmd}\"\nuser = \"{user}\"\n"
                )
            } else {
                format!(
                    "[terminal]\nvt = 7\n\n[default_session]\ncommand = \"env WLR_NO_HARDWARE_CURSORS=1 XCURSOR_THEME=Adwaita XCURSOR_SIZE=24 cage -s -- regreet\"\nuser = \"greeter\"\n"
                )
            };

            return Self::write_root_file(GREETD_CONFIG_PATH, &new_content);
        }

        // Fallback para LightDM:
        if value {
            let session_name = if desktop.contains("xfce") {
                "xfce"
            } else {
                "niri"
            };

            let new_content = format!(
                "[Seat:*]\nautologin-user={user}\nautologin-user-timeout=0\nautologin-session={session_name}\n"
            );

            Self::write_root_file(LIGHTDM_AUTOLOGIN_PATH, &new_content)
        } else {
            if std::path::Path::new(LIGHTDM_AUTOLOGIN_PATH).exists() {
                if getuid() == 0 {
                    Command::new("rm")
                        .args(["-f", LIGHTDM_AUTOLOGIN_PATH])
                        .status()
                        .map(|s| s.success())
                        .unwrap_or(false)
                } else {
                    Command::new("churros-pkexec")
                        .args(["rm", "-f", LIGHTDM_AUTOLOGIN_PATH])
                        .status()
                        .map(|s| s.success())
                        .unwrap_or(false)
                }
            } else {
                true
            }
        }
    }

    /// Obtiene el fondo de pantalla configurado en ReGreet
    pub fn regreet_wallpaper() -> String {
        let Ok(content) = fs::read_to_string(REGREET_CONFIG_PATH) else {
            return "/usr/share/churros/wallpapers/default.png".to_string();
        };
        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("path") && trimmed.contains('=') {
                if let Some((_, val)) = trimmed.split_once('=') {
                    let p = val.trim().trim_matches('"').trim_matches('\'');
                    if !p.is_empty() {
                        return p.to_string();
                    }
                }
            }
        }
        "/usr/share/churros/wallpapers/default.png".to_string()
    }

    /// Actualiza el fondo de pantalla en ReGreet
    pub fn set_regreet_wallpaper(wallpaper_path: &str) -> bool {
        let content = fs::read_to_string(REGREET_CONFIG_PATH).unwrap_or_else(|_| {
            format!("[background]\npath = \"{}\"\nfit = \"Cover\"\n", wallpaper_path)
        });

        let mut new_lines = Vec::new();
        let mut replaced = false;
        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("path") && trimmed.contains('=') {
                new_lines.push(format!("path = \"{}\"", wallpaper_path));
                replaced = true;
            } else {
                new_lines.push(line.to_string());
            }
        }
        if !replaced {
            new_lines.push(format!("[background]\npath = \"{}\"\nfit = \"Cover\"", wallpaper_path));
        }
        let new_content = new_lines.join("\n") + "\n";
        Self::write_root_file(REGREET_CONFIG_PATH, &new_content)
    }

    /// Obtiene el mensaje de bienvenida de ReGreet
    pub fn regreet_greeting() -> String {
        let Ok(content) = fs::read_to_string(REGREET_CONFIG_PATH) else {
            return "Bienvenido a ChurrOS".to_string();
        };
        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("greeting_msg") && trimmed.contains('=') {
                if let Some((_, val)) = trimmed.split_once('=') {
                    let msg = val.trim().trim_matches('"').trim_matches('\'');
                    if !msg.is_empty() {
                        return msg.to_string();
                    }
                }
            }
        }
        "Bienvenido a ChurrOS".to_string()
    }

    /// Actualiza el mensaje de bienvenida de ReGreet
    pub fn set_regreet_greeting(greeting: &str) -> bool {
        let content = fs::read_to_string(REGREET_CONFIG_PATH).unwrap_or_else(|_| {
            format!("[appearance]\ngreeting_msg = \"{}\"\n", greeting)
        });

        let mut new_lines = Vec::new();
        let mut replaced = false;
        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("greeting_msg") && trimmed.contains('=') {
                new_lines.push(format!("greeting_msg = \"{}\"", greeting));
                replaced = true;
            } else {
                new_lines.push(line.to_string());
            }
        }
        if !replaced {
            new_lines.push(format!("[appearance]\ngreeting_msg = \"{}\"", greeting));
        }
        let new_content = new_lines.join("\n") + "\n";
        Self::write_root_file(REGREET_CONFIG_PATH, &new_content)
    }

    fn write_root_file(target_path: &str, content: &str) -> bool {
        let tmp = std::env::temp_dir().join(format!("churros-cfg-{}.tmp", std::process::id()));
        if fs::write(&tmp, content).is_err() {
            return false;
        }
        let tmp_str = tmp.to_string_lossy().to_string();

        let target_dir = std::path::Path::new(target_path).parent().unwrap_or(std::path::Path::new("/etc"));
        let dir_str = target_dir.to_string_lossy().to_string();

        let _ = if getuid() == 0 {
            Command::new("mkdir").args(["-p", &dir_str]).status()
        } else {
            Command::new("churros-pkexec").args(["mkdir", "-p", &dir_str]).status()
        };

        let ok = if getuid() == 0 {
            Command::new("install")
                .args(["-m", "644", &tmp_str, target_path])
                .status()
                .map(|s| s.success())
                .unwrap_or(false)
        } else {
            Command::new("churros-pkexec")
                .args(["install", "-m", "644", &tmp_str, target_path])
                .status()
                .map(|s| s.success())
                .unwrap_or(false)
        };
        let _ = fs::remove_file(&tmp);
        ok
    }
}
