// ==========================================
// UpdateService — actualizaciones de pacman + flatpak
// (timer de systemd + notificaciones vía mako)
// ==========================================

use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::time::{Duration, Instant};

use serde_json::json;

use crate::services::settings;

pub struct UpdateService;

/// Release de utilidades de ChurrOS (del updates.json del servidor).
#[derive(Debug, Clone, PartialEq)]
pub struct ChurrosUpdate {
    pub version: String,
    pub file: String,
    pub sha256: String,
}

/// Parsea el updates.json del servidor de releases.
fn parse_updates_json(raw: &str) -> Option<ChurrosUpdate> {
    let v: serde_json::Value = serde_json::from_str(raw).ok()?;
    Some(ChurrosUpdate {
        version: v.get("version")?.as_str()?.to_string(),
        file: v.get("file")?.as_str()?.to_string(),
        sha256: v.get("sha256")?.as_str()?.to_string(),
    })
}

fn home() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/root".to_string());
    PathBuf::from(home)
}

fn user_systemd_dir() -> PathBuf {
    home().join(".config").join("systemd").join("user")
}

/// Ejecuta un comando con timeout y captura stdout (salida pequeña).
fn run_capture(args: &[&str], timeout_secs: u64) -> Option<String> {
    let mut child = Command::new(args[0])
        .args(&args[1..])
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .ok()?;

    let deadline = Instant::now() + Duration::from_secs(timeout_secs);
    let status = loop {
        match child.try_wait() {
            Ok(Some(st)) => break st,
            Ok(None) => {
                if Instant::now() >= deadline {
                    let _ = child.kill();
                    let _ = child.wait();
                    return None;
                }
                std::thread::sleep(Duration::from_millis(50));
            }
            Err(_) => return None,
        }
    };

    let mut buf = String::new();
    if let Some(mut out) = child.stdout.take() {
        use std::io::Read;
        let _ = out.read_to_string(&mut buf);
    }
    let _ = status;
    let _ = child.wait();
    Some(buf.trim().to_string())
}

/// Ejecuta un comando con streaming: llama `cb` con cada línea de salida
/// (stdout+stderr combinados) según se produce. Evita el deadlock del pipe
/// leyendo ambos en threads mientras el proceso corre.
fn run_streaming(args: &[&str], cb: &dyn Fn(&str)) -> bool {
    let mut child = match Command::new(args[0])
        .args(&args[1..])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    {
        Ok(c) => c,
        Err(_) => return false,
    };

    fn spawn_reader<R: std::io::Read + Send + 'static>(stream: R, tx: mpsc::Sender<String>) {
        std::thread::spawn(move || {
            let reader = BufReader::new(stream);
            for line in reader.lines().map_while(Result::ok) {
                if tx.send(line).is_err() {
                    break;
                }
            }
        });
    }

    let (tx, rx) = mpsc::channel::<String>();
    if let Some(s) = child.stdout.take() {
        spawn_reader(s, tx.clone());
    }
    if let Some(e) = child.stderr.take() {
        spawn_reader(e, tx.clone());
    }
    drop(tx);

    while let Ok(line) = rx.recv() {
        cb(&line);
    }

    child.wait().map(|s| s.success()).unwrap_or(false)
}

impl UpdateService {
    // -------------------------------------------------------- settings

    pub fn enabled() -> bool {
        settings::get_bool("updates.enabled", true)
    }

    pub fn set_enabled(enabled: bool) {
        settings::set("updates.enabled", json!(enabled));
    }

    pub fn interval() -> String {
        settings::get_string("updates.interval", "daily")
    }

    pub fn set_interval(interval: &str) {
        settings::set("updates.interval", json!(interval));
    }

    fn on_calendar(interval: &str) -> &'static str {
        match interval {
            "weekly" => "Mon *-*-* 04:00:00",
            "monthly" => "*-*-01 04:00:00",
            _ => "*-*-* 04:00:00",
        }
    }

    // -------------------------------------------------------- checks

    /// Lista de paquetes actualizables de pacman (`pacman -Qu`).
    pub fn check_pacman() -> Option<Vec<String>> {
        // Refrescar las bases (root) y luego listar upgrades.
        let _ = run_capture(&["churros-pkexec", "pacman", "-Sy"], 60);
        let out = run_capture(&["pacman", "-Qu"], 30)?;
        Some(out.lines().map(|s| s.to_string()).collect())
    }

    /// Lista de actualizaciones flatpak (`flatpak remote-ls --updates`).
    pub fn check_flatpak() -> Option<Vec<String>> {
        let out = run_capture(&["flatpak", "remote-ls", "--updates"], 30)?;
        Some(out.lines().map(|s| s.to_string()).collect())
    }

    // -------------------------------------------------------- updates

    pub fn update_pacman(cb: &dyn Fn(&str)) -> bool {
        run_streaming(
            &["churros-pkexec", "pacman", "-Syu", "--noconfirm"],
            cb,
        )
    }

    pub fn update_flatpak(cb: &dyn Fn(&str)) -> bool {
        run_streaming(&["churros-pkexec", "flatpak", "update", "-y"], cb)
    }

    // ------------------------------------------- utilidades de ChurrOS

    /// URL base del servidor de releases de utilidades de ChurrOS.
    pub fn churros_url() -> String {
        settings::get_string(
            "updates.churros_url",
            "https://download.churroslinux.org/churros/",
        )
    }

    /// Versión instalada de las utilidades (lee /etc/churros-version).
    pub fn installed_churros_version() -> String {
        std::fs::read_to_string("/etc/churros-version")
            .map(|s| s.trim().to_string())
            .unwrap_or_else(|_| "?".to_string())
    }

    /// Comprueba si hay una versión nueva de las utilidades de ChurrOS.
    /// `Some` = hay actualización disponible (versión != instalada).
    pub fn check_churros() -> Option<ChurrosUpdate> {
        let base = Self::churros_url();
        let url = format!("{base}updates.json");
        let out = run_capture(
            &["curl", "-fsSL", "--connect-timeout", "10", url.as_str()],
            15,
        )?;
        let update = parse_updates_json(&out)?;
        if update.version == Self::installed_churros_version() {
            None
        } else {
            Some(update)
        }
    }

    /// Actualiza las utilidades de ChurrOS vía churros-update-utils (root).
    pub fn update_churros(cb: &dyn Fn(&str)) -> bool {
        let url = Self::churros_url();
        run_streaming(&["churros-pkexec", "churros-update-utils", url.as_str()], cb)
    }

    // -------------------------------------------------------- timer

    /// Aplica el estado actual (enabled + intervalo) al timer de systemd.
    /// Escribe el drop-in con el OnCalendar y habilita/deshabilita el timer.
    pub fn apply_timer() {
        let drop_in_dir = user_systemd_dir().join("churros-update.timer.d");
        let _ = std::fs::create_dir_all(&drop_in_dir);
        let oncalendar = Self::on_calendar(&Self::interval());
        let content = format!("[Timer]\nOnCalendar={oncalendar}\n");
        let _ = std::fs::write(drop_in_dir.join("schedule.conf"), content);

        let _ = Command::new("systemctl")
            .args(["--user", "daemon-reload"])
            .status();

        if Self::enabled() {
            let _ = Command::new("systemctl")
                .args(["--user", "enable", "--now", "churros-update.timer"])
                .status();
        } else {
            let _ = Command::new("systemctl")
                .args(["--user", "disable", "--now", "churros-update.timer"])
                .status();
        }
    }

    /// Habilita el timer por primera vez si está activado y aún no enabled.
    pub fn ensure_timer() {
        if !Self::enabled() {
            return;
        }
        let active = Command::new("systemctl")
            .args(["--user", "is-enabled", "churros-update.timer"])
            .status()
            .map(|s| s.success())
            .unwrap_or(false);
        if !active {
            Self::apply_timer();
        }
    }

    // -------------------------------------------------------- notify

    pub fn notify(summary: &str, body: &str) {
        let _ = Command::new("notify-send")
            .args(["-a", "ChurrOS", summary, body])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_updates_json_valid() {
        let raw = r#"{
            "version": "0.7",
            "date": "2026-08-20",
            "file": "churros-utils-0.7.tar.zst",
            "sha256": "abc123"
        }"#;
        let u = parse_updates_json(raw).unwrap();
        assert_eq!(u.version, "0.7");
        assert_eq!(u.file, "churros-utils-0.7.tar.zst");
        assert_eq!(u.sha256, "abc123");
    }

    #[test]
    fn parse_updates_json_invalid() {
        assert!(parse_updates_json("not json").is_none());
        assert!(parse_updates_json(r#"{"version":"0.7"}"#).is_none());
    }
}
