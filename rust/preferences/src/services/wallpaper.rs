// ==========================================
// WallpaperService — estado del wallpaper (equivalente a services/wallpaper.py)
// ==========================================

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::Duration;

use crate::services::settings;

pub struct WallpaperService;

fn build_env() -> Vec<(String, String)> {
    let mut env: Vec<(String, String)> = std::env::vars().collect();

    if !env.iter().any(|(k, _)| k == "WAYLAND_DISPLAY") {
        let uid = current_uid();
        let xrd = format!("/run/user/{uid}");
        if Path::new(&xrd).is_dir() {
            if let Ok(entries) = fs::read_dir(&xrd) {
                let mut socks: Vec<String> = entries
                    .flatten()
                    .filter_map(|e| {
                        let name = e.file_name().to_string_lossy().to_string();
                        name.starts_with("wayland-").then_some(name)
                    })
                    .collect();
                socks.sort();
                if let Some(sock) = socks.first() {
                    env.push(("WAYLAND_DISPLAY".to_string(), sock.clone()));
                    env.push(("XDG_RUNTIME_DIR".to_string(), xrd.clone()));
                }
            }
        }
    }

    if !env.iter().any(|(k, _)| k == "XDG_RUNTIME_DIR") {
        env.push(("XDG_RUNTIME_DIR".to_string(), format!("/run/user/{}", current_uid())));
    }

    env
}

fn current_uid() -> u32 {
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

/// shutil.which equivalente: busca el binario en $PATH
fn which(name: &str) -> bool {
    let path_var = std::env::var("PATH").unwrap_or_default();
    path_var.split(':').any(|dir| Path::new(dir).join(name).is_file())
}

/// Ejecuta un comando con timeout y captura stdout/stderr. None si falla/timeout.
fn run_with_timeout(
    args: &[&str],
    timeout: Duration,
    env_refs: &[(&str, &str)],
) -> Option<std::process::Output> {
    let mut child = Command::new(args[0])
        .args(&args[1..])
        .envs(env_refs.iter().map(|(k, v)| (*k, *v)))
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .ok()?;

    let deadline = std::time::Instant::now() + timeout;
    let status = loop {
        match child.try_wait() {
            Ok(Some(st)) => break st,
            Ok(None) => {
                if std::time::Instant::now() >= deadline {
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
        let _ = std::io::Read::read_to_end(&mut out, &mut stdout);
    }
    if let Some(mut err) = child.stderr.take() {
        let _ = std::io::Read::read_to_end(&mut err, &mut stderr);
    }
    let _ = child.wait();
    Some(std::process::Output {
        status,
        stdout,
        stderr,
    })
}

impl WallpaperService {
    /// Ruta del wallpaper actual (settings.json wallpaper.path)
    pub fn current() -> String {
        settings::get_string("wallpaper.path", "")
    }

    pub fn user_dir() -> PathBuf {
        let home = std::env::var("HOME").unwrap_or_else(|_| "/root".to_string());
        PathBuf::from(home)
            .join(".local/share/churros/wallpapers")
    }

    /// Directorios donde se buscan wallpapers (orden de prioridad)
    pub fn wallpaper_dirs() -> Vec<PathBuf> {
        let home = std::env::var("HOME").unwrap_or_else(|_| "/root".to_string());
        vec![
            PathBuf::from("/usr/share/churros/wallpapers"),
            PathBuf::from("/usr/share/backgrounds"),
            Self::user_dir(),
            PathBuf::from(&home).join("Pictures/Wallpapers"),
            PathBuf::from(&home).join("Pictures"),
        ]
    }

    /// Escanea los directorios y devuelve wallpapers (ext: jpg jpeg png webp gif)
    pub fn list() -> Vec<PathBuf> {
        let mut found = Vec::new();
        for dir in Self::wallpaper_dirs() {
            if let Ok(entries) = std::fs::read_dir(&dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_file() {
                        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                            let ext = ext.to_lowercase();
                            if matches!(ext.as_str(), "jpg" | "jpeg" | "png" | "webp" | "gif") {
                                found.push(path);
                            }
                        }
                    }
                }
            }
        }
        found.sort();
        found.dedup();
        found
    }

    /// Guarda el wallpaper en settings.json y lo aplica en vivo.
    /// Devuelve si se aplicó correctamente (equivalente a WallpaperService.set).
    pub fn set(path: &str) -> bool {
        settings::set("wallpaper.path", serde_json::json!(path));
        let applied = Self::apply(path);
        // Colores dinámicos: regenerar paleta pywal si está activo.
        crate::services::pywal::PywalService::regenerate_if_enabled();
        applied
    }

    /// Aplica el wallpaper con churros-apply-wallpaper o swaybg
    /// (equivalente a WallpaperService.apply del Python).
    pub fn apply(path: &str) -> bool {
        if path.is_empty() || !Path::new(path).is_file() {
            println!("[wallpaper] ruta invalida: {path}");
            return false;
        }

        let env = build_env();
        let env_refs: Vec<(&str, &str)> = env
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect();

        // Backend 1: wrapper churros-apply-wallpaper (con timeout: no bloquear
        // la UI si el wrapper se cuelga).
        if which("churros-apply-wallpaper") {
            let r = run_with_timeout(
                &["churros-apply-wallpaper", path],
                Duration::from_secs(10),
                &env_refs,
            );
            match r {
                Some(out) => {
                    println!(
                        "[wallpaper] wrapper stdout: {}",
                        String::from_utf8_lossy(&out.stdout)
                    );
                    if !out.stderr.is_empty() {
                        println!(
                            "[wallpaper] wrapper stderr: {}",
                            String::from_utf8_lossy(&out.stderr)
                        );
                    }
                    if out.status.success() {
                        return true;
                    }
                    println!("[wallpaper] wrapper fallo rc={:?}", out.status.code());
                }
                None => println!("[wallpaper] wrapper timeout/ex"),
            }
        }

        // Backend 2: swaybg
        if which("swaybg") {
            let _ = Command::new("pkill")
                .args(["-x", "swaybg"])
                .envs(env_refs.iter().map(|(k, v)| (*k, *v)))
                .output();
            let _ = Command::new("swaybg")
                .args(["-i", path, "-m", "fill"])
                .envs(env_refs.iter().map(|(k, v)| (*k, *v)))
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn();
            std::thread::sleep(Duration::from_millis(500));
            let ok = Command::new("pgrep")
                .args(["-x", "swaybg"])
                .envs(env_refs.iter().map(|(k, v)| (*k, *v)))
                .output()
                .map(|o| o.status.success())
                .unwrap_or(false);
            if ok {
                println!("[wallpaper] swaybg OK: {path}");
                return true;
            }
        }

        println!("[wallpaper] NINGUN backend funciono");
        false
    }

    /// Copia la imagen a ~/.local/share/churros/wallpapers evitando colisiones
    /// de nombre (name_1.ext, name_2.ext...). Devuelve la ruta destino o None.
    /// (equivalente a WallpaperService.import_image del Python)
    pub fn import_image(source_path: &str) -> Option<String> {
        if source_path.is_empty() || !Path::new(source_path).is_file() {
            return None;
        }

        let user_dir = Self::user_dir();
        if fs::create_dir_all(&user_dir).is_err() {
            return None;
        }

        let base = Path::new(source_path)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("wallpaper")
            .to_string();

        let (name, ext) = match base.rsplit_once('.') {
            Some((n, e)) => (n.to_string(), format!(".{e}")),
            None => (base.clone(), String::new()),
        };

        let mut dest = user_dir.join(&base);
        let mut n = 1u32;
        while dest.exists() {
            dest = user_dir.join(format!("{name}_{n}{ext}"));
            n += 1;
        }

        if fs::copy(source_path, &dest).is_err() {
            return None;
        }

        Some(dest.to_string_lossy().to_string())
    }
}
