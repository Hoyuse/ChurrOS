// ==========================================
// UsersService — cuenta del sistema y autologin de greetd
// (equivalente a services/users.py)
// ==========================================

use std::fs;
use std::process::Command;

pub struct UsersService;

const GREETD_PATH: &str = "/etc/greetd/config.toml";

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

/// Línea tipo `command = "/usr/bin/niri"` (regex ^\s*command\s*=\s*"[^"]+").
fn line_is_command(line: &str) -> bool {
    let t = line.trim();
    if !t.starts_with("command") {
        return false;
    }
    let after = t["command".len()..].trim_start();
    let Some(after) = after.strip_prefix('=') else {
        return false;
    };
    let after = after.trim_start();
    if !after.starts_with('"') || after.len() < 2 {
        return false;
    }
    after[1..].find('"').is_some() && after[1..].find('"').unwrap() >= 1
}

/// Línea tipo `command = ...` (regex ^\s*command\s*=).
fn line_has_command_eq(line: &str) -> bool {
    let t = line.trim();
    if !t.starts_with("command") {
        return false;
    }
    t["command".len()..].trim_start().starts_with('=')
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

    /// ¿Hay autologin configurado en greetd? Solo cuenta la sección
    /// `[default_session]` (un `command` en `[terminal]`/`[initial_session]`
    /// no es autologin).
    pub fn auto_login() -> bool {
        let Ok(content) = fs::read_to_string(GREETD_PATH) else {
            return false;
        };
        let mut in_default = false;
        for line in content.lines() {
            if let Some(sec) = section_name(line) {
                in_default = sec == "default_session";
                continue;
            }
            if in_default && line_is_command(line) {
                return true;
            }
        }
        false
    }

    /// Activa/desactiva el autologin editando /etc/greetd/config.toml
    /// (equivalente a set_auto_login; escritura atómica tmp + rename).
    pub fn set_auto_login(value: bool) -> bool {
        let Ok(content) = fs::read_to_string(GREETD_PATH) else {
            return false;
        };

        // ¿Hay un `command` dentro de [default_session]?
        let mut has_command = false;
        let mut in_default = false;
        for line in content.lines() {
            if let Some(sec) = section_name(line) {
                in_default = sec == "default_session";
                continue;
            }
            if in_default && line_has_command_eq(line) {
                has_command = true;
                break;
            }
        }

        if value && has_command {
            return true;
        }
        if !value && !has_command {
            return true;
        }

        let new_content = if value {
            // Insertar `command = "/usr/bin/niri"` tras [default_session]
            let mut lines: Vec<&str> = content.lines().collect();
            let has_session = lines.iter().any(|l| l.trim() == "[default_session]");
            if has_session {
                let mut out: Vec<String> = Vec::with_capacity(lines.len() + 1);
                let mut inserted = false;
                for line in lines {
                    out.push(line.to_string());
                    if !inserted && line.trim() == "[default_session]" {
                        out.push("command = \"/usr/bin/niri\"".to_string());
                        inserted = true;
                    }
                }
                out.join("\n")
            } else {
                format!("{content}\n[default_session]\ncommand = \"/usr/bin/niri\"\n")
            }
        } else {
            // Quitar la línea `command = "..."` SOLO dentro de [default_session]
            let mut out: Vec<String> = Vec::with_capacity(content.lines().count());
            let mut in_default = false;
            let mut removed = false;
            for line in content.lines() {
                if let Some(sec) = section_name(line) {
                    in_default = sec == "default_session";
                    out.push(line.to_string());
                    continue;
                }
                if in_default && !removed && line_is_command(line) {
                    removed = true;
                    continue;
                }
                out.push(line.to_string());
            }
            out.join("\n")
        };

        if new_content == content {
            return true;
        }

        // /etc/greetd no es escribible por el usuario: escribir a un temporal
        // propio y copiarlo con privilegios (churros-pkexec: pkexec/sudo -n).
        // El patrón es el mismo que usa datetime.rs con timedatectl.
        let tmp = std::env::temp_dir().join(format!("churros-greetd-{}.toml", std::process::id()));
        if fs::write(&tmp, &new_content).is_err() {
            return false;
        }
        let tmp_str = tmp.to_string_lossy().to_string();
        let ok = if getuid() == 0 {
            Command::new("install")
                .args(["-m", "644", &tmp_str, GREETD_PATH])
                .status()
                .map(|s| s.success())
                .unwrap_or(false)
        } else {
            Command::new("churros-pkexec")
                .args(["install", "-m", "644", &tmp_str, GREETD_PATH])
                .status()
                .map(|s| s.success())
                .unwrap_or(false)
        };
        let _ = fs::remove_file(&tmp);
        ok
    }
}
