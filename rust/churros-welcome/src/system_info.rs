// ==========================================
// Información del sistema (equivalente a utils/system.py)
// ==========================================

use std::fs;

// ==========================================
// CPU
// ==========================================

pub fn get_cpu() -> String {
    if let Ok(content) = fs::read_to_string("/proc/cpuinfo") {
        for line in content.lines() {
            let line = line.trim();
            if let Some(rest) = line.strip_prefix("model name") {
                if let Some((_, value)) = rest.split_once(':') {
                    return value.trim().to_string();
                }
            }
            if let Some(rest) = line.strip_prefix("Hardware") {
                if let Some((_, value)) = rest.split_once(':') {
                    return value.trim().to_string();
                }
            }
            if let Some(rest) = line.strip_prefix("CPU") {
                if let Some((_, value)) = rest.split_once(':') {
                    return value.trim().to_string();
                }
            }
        }
    }
    "Desconocido".to_string()
}

// ==========================================
// Kernel
// ==========================================

pub fn get_kernel() -> String {
    fs::read_to_string("/proc/sys/kernel/osrelease")
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|_| "Desconocido".to_string())
}

// ==========================================
// Hostname
// ==========================================

pub fn get_hostname() -> String {
    fs::read_to_string("/proc/sys/kernel/hostname")
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|_| "Desconocido".to_string())
}

// ==========================================
// Memoria RAM (total, en GiB)
// ==========================================

pub fn get_memory() -> String {
    if let Ok(content) = fs::read_to_string("/proc/meminfo") {
        for line in content.lines() {
            if let Some(rest) = line.strip_prefix("MemTotal:") {
                if let Some(kib_str) = rest.split_whitespace().next() {
                    if let Ok(total_kib) = kib_str.parse::<f64>() {
                        return format!("{:.1} GiB", total_kib / 1024.0 / 1024.0);
                    }
                }
            }
        }
    }
    "Desconocido".to_string()
}

// ==========================================
// Sistema Operativo
// ==========================================

pub fn get_os() -> String {
    if let Ok(content) = fs::read_to_string("/etc/os-release") {
        for line in content.lines() {
            if let Some(rest) = line.strip_prefix("PRETTY_NAME=") {
                return rest.trim_matches('"').trim().to_string();
            }
        }
    }
    "Linux".to_string()
}

// ==========================================
// Arquitectura
// ==========================================

pub fn get_architecture() -> String {
    std::env::consts::ARCH.to_string()
}
