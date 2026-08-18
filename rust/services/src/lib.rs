// ==========================================
// churros-services — capa de servicios de ChurrOS
// (port Rust de usr/share/churros/services/*.py)
// ==========================================

pub mod audio;
pub mod battery;
pub mod bluetooth;
pub mod brightness;
pub mod ethernet;
pub mod jsonc;
pub mod power;
pub mod version;
pub mod waybar_style;
pub mod wifi;

use std::io::Read;
use std::process::{Command, Stdio};
use std::time::Duration;

use wait_timeout::ChildExt;

/// Resultado de ejecutar un comando: (returncode, stdout, stderr).
pub type RunOut = (i32, String, String);

/// Ejecuta un comando capturando stdout/stderr, con timeout opcional.
/// Devuelve None si falla el spawn, el timeout se agota o la salida no es UTF-8
/// (equivalente al `try/except` de los módulos Python).
pub fn run(cmd: &[&str], timeout_ms: u64) -> Option<RunOut> {
    let mut child = Command::new(cmd[0])
        .args(&cmd[1..])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .ok()?;

    let timed_out = if timeout_ms > 0 {
        match child
            .wait_timeout(Duration::from_millis(timeout_ms))
            .ok()?
        {
            Some(_) => false,
            None => {
                let _ = child.kill();
                let _ = child.wait();
                true
            }
        }
    } else {
        let _ = child.wait();
        false
    };

    if timed_out {
        return None;
    }

    let code = child.wait().map(|s| s.code().unwrap_or(1)).unwrap_or(1);

    let mut stdout = String::new();
    let mut stderr = String::new();
    if let Some(mut so) = child.stdout.take() {
        let _ = so.read_to_string(&mut stdout);
    }
    if let Some(mut se) = child.stderr.take() {
        let _ = se.read_to_string(&mut stderr);
    }

    Some((code, stdout, stderr))
}

/// Ejecuta un comando al vuelo (fire-and-forget, equivale a subprocess.Popen).
pub fn spawn(cmd: &[&str]) {
    let _ = Command::new(cmd[0])
        .args(&cmd[1..])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn();
}

/// Comprueba si un binario existe en el PATH (equivale a shutil.which).
pub fn which(bin: &str) -> bool {
    std::env::var_os("PATH")
        .map(|paths| {
            std::env::split_paths(&paths).any(|d| d.join(bin).is_file())
        })
        .unwrap_or(false)
}
