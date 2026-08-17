// ==========================================
// BackupService — exportar/importar/restablecer config de ChurrOS
// (equivalente a services/backup_service.py)
// ==========================================

use serde_json::Value;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::Command;

pub struct BackupService;

fn home() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/root".to_string());
    PathBuf::from(home)
}

fn churros_dir() -> PathBuf {
    home().join(".config").join("churros")
}

fn settings_file() -> PathBuf {
    churros_dir().join("settings.json")
}

/// Dotfiles incluidos en el backup: (nombre, ruta)
fn dotfiles() -> Vec<(&'static str, PathBuf)> {
    vec![
        ("niri", home().join(".config").join("niri")),
        ("foot", home().join(".config").join("foot")),
        ("fuzzel", home().join(".config").join("fuzzel")),
        ("mako", home().join(".config").join("mako")),
        ("waybar", home().join(".config").join("waybar")),
    ]
}

const DEFAULTS_DIR: &str = "/usr/share/churros/defaults";

/// Une `base` + `rel` rechazando rutas absolutas y componentes ".."
/// (protección contra path traversal en backups maliciosos).
fn safe_join(base: &Path, rel: &str) -> Option<PathBuf> {
    let rel_path = Path::new(rel);
    if rel_path.is_absolute() {
        return None;
    }
    if rel_path.components().any(|c| {
        matches!(
            c,
            std::path::Component::ParentDir
                | std::path::Component::RootDir
                | std::path::Component::Prefix(_)
        )
    }) {
        return None;
    }
    Some(base.join(rel_path))
}

/// Copia recursiva de directorio (como shutil.copytree).
fn copy_dir_all(src: &Path, dst: &Path) -> std::io::Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let target = dst.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_dir_all(&entry.path(), &target)?;
        } else {
            fs::copy(entry.path(), target)?;
        }
    }
    Ok(())
}

impl BackupService {
    /// Exporta settings.json + dotfiles a un .tar (equivalente a export_to).
    pub fn export_to(dest_path: &str) -> Result<String, String> {
        let any_dotfile = dotfiles().iter().any(|(_, p)| p.exists());
        if !churros_dir().is_dir() && !any_dotfile {
            return Err("No hay configuracion que exportar".to_string());
        }

        let dest = PathBuf::from(dest_path);
        let directory = dest
            .parent()
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|| PathBuf::from("."));
        fs::create_dir_all(&directory).map_err(|e| e.to_string())?;

        // Archivo temporal en el mismo directorio (como mkstemp)
        let tmp = directory.join(format!(
            "churros-backup-{}-{}.tar.zst",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_nanos())
                .unwrap_or(0)
        ));

        let result = (|| -> std::io::Result<()> {
            let file = fs::File::create(&tmp)?;
            // Compresión zstd real (la extensión es .tar.zst).
            let encoder = zstd::stream::Encoder::new(file, 3)?;
            let mut tar = tar::Builder::new(encoder);

            if settings_file().exists() {
                tar.append_path_with_name(&settings_file(), "churros/settings.json")?;
            }

            for (name, path) in dotfiles() {
                if path.exists() {
                    tar.append_dir_all(&format!("dotfiles/{name}"), &path)?;
                }
            }

            let encoder = tar.into_inner()?;
            encoder.finish()?;
            Ok(())
        })();

        match result {
            Ok(()) => {
                fs::rename(&tmp, &dest).map_err(|e| e.to_string())?;
                Ok(dest_path.to_string())
            }
            Err(e) => {
                let _ = fs::remove_file(&tmp);
                Err(e.to_string())
            }
        }
    }

    /// Importa un backup de ChurrOS (equivalente a import_from).
    pub fn import_from(src_path: &str) -> Result<bool, String> {
        let src = PathBuf::from(src_path);
        if !src.is_file() {
            return Err(format!("El archivo no existe: {src_path}"));
        }

        // Leer todos los miembros del tar (path, is_dir, contenido)
        let mut items: Vec<(String, bool, Vec<u8>)> = Vec::new();
        let mut has_churros = false;
        let mut has_dotfiles = false;

        let file = fs::File::open(&src).map_err(|e| format!("Archivo invalido: {e}"))?;
        let raw = std::io::Read::bytes(file)
            .collect::<Result<Vec<u8>, _>>()
            .map_err(|e| format!("Archivo invalido: {e}"))?;

        // Backup comprimido con zstd (magic 28 B5 2F FD) o tar plano (legacy).
        let reader: Box<dyn Read> = if raw.starts_with(&[0x28, 0xB5, 0x2F, 0xFD]) {
            Box::new(
                zstd::stream::read::Decoder::new(&raw[..])
                    .map_err(|e| format!("Archivo invalido: {e}"))?,
            )
        } else {
            Box::new(&raw[..])
        };
        let mut archive = tar::Archive::new(reader);

        let entries = archive
            .entries()
            .map_err(|e| format!("Archivo invalido: {e}"))?;

        for entry in entries {
            let mut entry = entry.map_err(|e| format!("Archivo invalido: {e}"))?;
            let name = entry
                .path()
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_default();
            let is_dir = entry.header().entry_type().is_dir();

            if name.starts_with("churros/") || name == "churros" {
                has_churros = true;
            }
            if name.starts_with("dotfiles/") || name == "dotfiles" {
                has_dotfiles = true;
            }

            let mut data = Vec::new();
            if !is_dir {
                let _ = entry.read_to_end(&mut data);
            }
            items.push((name, is_dir, data));
        }

        if !has_churros && !has_dotfiles {
            return Err("El archivo no es un backup de ChurrOS".to_string());
        }

        // Sanitizar rutas: rechazar absolutas y ".." (path traversal).
        for (name, is_dir, data) in items {
            if name.starts_with("churros/") {
                if is_dir {
                    continue;
                }
                let rel = name.trim_start_matches("churros/");
                let Some(target) = safe_join(&churros_dir(), rel) else {
                    continue;
                };
                if let Some(parent) = target.parent() {
                    fs::create_dir_all(parent).map_err(|e| e.to_string())?;
                }
                fs::write(&target, data).map_err(|e| e.to_string())?;
            } else if name.starts_with("dotfiles/") {
                let mut parts = name.splitn(3, '/');
                let _ = parts.next(); // "dotfiles"
                let Some(df_name) = parts.next() else { continue };
                let rest = parts.next().unwrap_or("");

                // Solo dotfiles conocidos (evita escribir dirs arbitrarios).
                if !dotfiles().iter().any(|(n, _)| *n == df_name) {
                    continue;
                }

                let target_dir = home().join(".config").join(df_name);

                if is_dir {
                    fs::create_dir_all(&target_dir).map_err(|e| e.to_string())?;
                    continue;
                }
                if rest.is_empty() {
                    continue;
                }

                let Some(target) = safe_join(&target_dir, rest) else {
                    continue;
                };
                if let Some(parent) = target.parent() {
                    fs::create_dir_all(parent).map_err(|e| e.to_string())?;
                }
                fs::write(&target, data).map_err(|e| e.to_string())?;
            }
        }

        Self::reload_services();
        Ok(true)
    }

    /// Restablece settings.json y los dotfiles desde /usr/share/churros/defaults.
    pub fn reset_to_defaults() -> Result<bool, String> {
        let defaults_dir = PathBuf::from(DEFAULTS_DIR);
        if !defaults_dir.is_dir() {
            return Err(format!("Defaults no encontrados: {DEFAULTS_DIR}"));
        }

        Self::restore_settings();
        Self::restore_dotfiles();
        Self::reload_services();
        Ok(true)
    }

    fn restore_settings() {
        let defaults = serde_json::json!({
            "theme": { "dark": false, "dynamic_colors": true },
            "accent": { "color": "Orange" },
            "wallpaper": { "path": "" },
            "icons": { "theme": "Papirus" },
            "cursor": { "theme": "Bibata" },
            "fonts": { "family": "Inter", "scale": 1.0 }
        });
        crate::services::settings::save(&defaults);
    }

    fn restore_dotfiles() {
        let defaults_dir = PathBuf::from(DEFAULTS_DIR);
        let Ok(entries) = fs::read_dir(&defaults_dir) else {
            return;
        };
        for entry in entries.flatten() {
            let src = entry.path();
            if !src.is_dir() {
                continue;
            }
            let dst = home().join(".config").join(entry.file_name());
            if dst.exists() {
                let _ = fs::remove_dir_all(&dst);
            }
            let _ = copy_dir_all(&src, &dst);
        }
    }

    /// Recarga waybar/mako/fuzzel (equivalente a _reload_services).
    pub fn reload_services() {
        for cmd in [
            vec!["pkill", "-x", "-USR2", "waybar"],
            vec!["makoctl", "reload"],
            vec!["pkill", "-x", "fuzzel"],
        ] {
            let _ = Command::new(&cmd[0])
                .args(&cmd[1..])
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::null())
                .spawn();
        }
    }

    /// Settings defaults (paridad con SettingsService.DEFAULTS de Python).
    #[allow(dead_code)]
    pub fn defaults() -> Value {
        serde_json::json!({
            "theme": { "dark": false, "dynamic_colors": true },
            "accent": { "color": "Orange" },
            "wallpaper": { "path": "" },
            "icons": { "theme": "Papirus" },
            "cursor": { "theme": "Bibata" },
            "fonts": { "family": "Inter", "scale": 1.0 }
        })
    }
}
