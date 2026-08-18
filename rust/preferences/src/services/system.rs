// ==========================================
// SystemService (equivalente a services/system.py)
// ==========================================

use std::process::Command;

pub struct SystemService;

impl SystemService {
    pub fn distro() -> &'static str {
        "ChurrOS"
    }

    pub fn version() -> &'static str {
        churros_services::version::distro()
    }

    pub fn edition() -> &'static str {
        "Developer Preview"
    }

    pub fn session() -> String {
        std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_else(|_| "Desconocida".to_string())
    }

    pub fn hostname() -> String {
        std::fs::read_to_string("/proc/sys/kernel/hostname")
            .map(|s| s.trim().to_string())
            .unwrap_or_else(|_| "Desconocido".to_string())
    }

    pub fn username() -> String {
        std::env::var("USER").unwrap_or_else(|_| "Desconocido".to_string())
    }

    pub fn kernel() -> String {
        std::fs::read_to_string("/proc/sys/kernel/osrelease")
            .map(|s| s.trim().to_string())
            .unwrap_or_else(|_| "Desconocido".to_string())
    }

    pub fn package_manager() -> &'static str {
        "Pacman"
    }

    pub fn base() -> &'static str {
        "Arch Linux"
    }

    pub fn cpu() -> String {
        if let Ok(content) = std::fs::read_to_string("/proc/cpuinfo") {
            for line in content.lines() {
                if let Some(rest) = line.strip_prefix("model name") {
                    if let Some((_, value)) = rest.split_once(':') {
                        return value.trim().to_string();
                    }
                }
            }
        }
        "Desconocido".to_string()
    }

    pub fn memory() -> String {
        if let Ok(content) = std::fs::read_to_string("/proc/meminfo") {
            for line in content.lines() {
                if let Some(rest) = line.strip_prefix("MemTotal:") {
                    if let Some(kib) = rest.split_whitespace().next().and_then(|v| v.parse::<f64>().ok()) {
                        return format!("{:.1} GB", kib / 1024.0 / 1024.0);
                    }
                }
            }
        }
        "Desconocida".to_string()
    }

    pub fn gpu() -> String {
        let output = Command::new("lspci").output();
        if let Ok(output) = output {
            let stdout = String::from_utf8_lossy(&output.stdout);
            for line in stdout.lines() {
                if line.contains("VGA") || line.contains("3D") {
                    if let Some((_, desc)) = line.split_once(": ") {
                        return desc.to_string();
                    }
                }
            }
        }
        "Desconocida".to_string()
    }
}
