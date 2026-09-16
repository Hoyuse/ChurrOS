// ==========================================
// Información del sistema (equivalente a utils/system.py)
// ==========================================

use std::fs;

// ==========================================
// CPU
// ==========================================

pub fn get_cpu() -> String {
    if let Ok(content) = fs::read_to_string("/proc/cpuinfo") {
        if let Some(model) = cpu_model(&content) {
            return model;
        }
    }
    "Desconocido".to_string()
}

fn cpu_model(content: &str) -> Option<String> {
    const MODEL_KEYS: [&str; 4] = ["model name", "Model", "Hardware", "Processor"];

    content.lines().find_map(|line| {
        let (key, value) = line.split_once(':')?;
        MODEL_KEYS.contains(&key.trim()).then(|| value.trim().to_string())
    })
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

#[cfg(test)]
mod tests {
    use super::cpu_model;

    #[test]
    fn reads_x86_model_name() {
        assert_eq!(
            cpu_model("processor\t: 0\nmodel name\t: Test CPU\n"),
            Some("Test CPU".to_string())
        );
    }

    #[test]
    fn reads_arm_hardware_field() {
        assert_eq!(
            cpu_model("processor\t: 0\nHardware\t: Test ARM board\n"),
            Some("Test ARM board".to_string())
        );
    }

    #[test]
    fn ignores_unrelated_cpuinfo_fields() {
        assert_eq!(cpu_model("processor\t: 0\nCPU architecture\t: 8\n"), None);
    }
}
