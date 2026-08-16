// ==========================================
// LogsService — journalctl de niri + validacion de config
// (equivalente a services/logs_service.py)
// ==========================================

use std::io::Read;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

pub struct LogsService;

/// Ejecuta un comando con timeout (std no tiene wait_timeout).
/// Devuelve el Output completo o None si falla/excede el timeout.
fn run_with_timeout(args: &[&str], timeout_secs: u64) -> Option<std::process::Output> {
    let mut child = Command::new(args[0])
        .args(&args[1..])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
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

    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    if let Some(mut out) = child.stdout.take() {
        let _ = out.read_to_end(&mut stdout);
    }
    if let Some(mut err) = child.stderr.take() {
        let _ = err.read_to_end(&mut stderr);
    }
    let _ = child.wait();

    Some(std::process::Output {
        status,
        stdout,
        stderr,
    })
}

fn decode(bytes: Vec<u8>) -> String {
    String::from_utf8_lossy(&bytes).to_string()
}

impl LogsService {
    /// Últimos logs de niri del journal (equivalente a niri_logs).
    pub fn niri_logs(limit: i64) -> String {
        // 1) _COMM=niri
        if let Some(out) = run_with_timeout(
            &[
                "journalctl",
                "-b0",
                "--no-pager",
                "--no-hostname",
                "-o",
                "short-iso",
                "-n",
                &limit.to_string(),
                "_COMM=niri",
            ],
            8,
        ) {
            let output = decode(out.stdout);
            if !output.trim().is_empty() {
                return output;
            }
        }

        // 2) -u greetd
        if let Some(out) = run_with_timeout(
            &[
                "journalctl",
                "-b0",
                "--no-pager",
                "--no-hostname",
                "-o",
                "short-iso",
                "-n",
                &limit.to_string(),
                "-u",
                "greetd",
            ],
            8,
        ) {
            return decode(out.stdout);
        }

        // 3) --grep=niri
        if let Some(out) = run_with_timeout(
            &[
                "journalctl",
                "-b0",
                "--no-pager",
                "--no-hostname",
                "-o",
                "short-iso",
                "-n",
                &limit.to_string(),
                "--grep=niri",
            ],
            8,
        ) {
            return decode(out.stdout);
        }

        String::new()
    }

    /// Ejecuta `niri validate` (equivalente a niri_validate).
    /// Devuelve (ok, mensaje). Si niri no existe -> (true, "").
    pub fn niri_validate() -> (bool, String) {
        match Command::new("niri")
            .arg("validate")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
        {
            Ok(out) => {
                let stderr = decode(out.stderr).trim().to_string();
                if out.status.success() {
                    (true, String::new())
                } else {
                    (false, stderr)
                }
            }
            Err(e) => {
                if e.kind() == std::io::ErrorKind::NotFound {
                    (true, String::new())
                } else {
                    (false, e.to_string())
                }
            }
        }
    }
}
