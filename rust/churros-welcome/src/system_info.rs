// ==========================================
// Información del sistema (equivalente a utils/system.py)
//
// Los parsers son funciones puras sobre el texto de /proc y /etc: se
// pueden probar sin levantar el sistema (ver tests al final del archivo).
// Los getters se limitan a leer el fichero y delegar en ellos.
// ==========================================

use std::fs;

const UNKNOWN: &str = "Desconocido";

// ==========================================
// CPU
// ==========================================

/// Extrae el modelo de CPU de /proc/cpuinfo.
///
/// x86 usa `model name`; aarch64 usa `Model`, `CPU model` o `Hardware`, y en
/// otras arquitecturas solo aparece `Processor`. Se prueban en orden y se
/// devuelve el primer valor no vacío.
pub fn parse_cpu(cpuinfo: &str) -> Option<String> {
    const KEYS: [&str; 5] = [
        "model name",
        "CPU model",
        "Model",
        "Hardware",
        "Processor",
    ];

    for line in cpuinfo.lines() {
        let Some((key, value)) = line.split_once(':') else {
            continue;
        };
        let key = key.trim();
        if !KEYS.contains(&key) {
            continue;
        }
        let value = value.trim();
        if !value.is_empty() {
            return Some(value.to_string());
        }
    }

    None
}

pub fn get_cpu() -> String {
    fs::read_to_string("/proc/cpuinfo")
        .ok()
        .and_then(|content| parse_cpu(&content))
        .unwrap_or_else(|| UNKNOWN.to_string())
}

// ==========================================
// Memoria RAM (usada, total y porcentaje)
// ==========================================

/// Devuelve (usada KiB, total KiB) a partir de /proc/meminfo.
///
/// "Usada" es total - available, que es la cifra que ve la gente en un gestor
/// de tareas. Si el kernel no publica `MemAvailable` ( kernels antiguos) se
/// reconstruye con free + buffers + cached + reclaimable, que es lo que hacía
/// el cálculo clásico.
pub fn parse_memory(meminfo: &str) -> Option<(u64, u64)> {
    let value = |key: &str| -> Option<u64> {
        meminfo.lines().find_map(|line| {
            let (name, rest) = line.split_once(':')?;
            if name.trim() != key {
                return None;
            }
            rest.split_whitespace().next()?.parse::<u64>().ok()
        })
    };

    let total = value("MemTotal")?;

    let available = value("MemAvailable").or_else(|| {
        Some(
            value("MemFree")?
                + value("Buffers").unwrap_or(0)
                + value("Cached").unwrap_or(0)
                + value("SReclaimable").unwrap_or(0),
        )
    })?;

    Some((total.saturating_sub(available), total))
}

fn format_gib(kib: u64) -> String {
    format!("{:.1} GiB", kib as f64 / 1024.0 / 1024.0)
}

pub fn get_memory() -> String {
    match fs::read_to_string("/proc/meminfo")
        .ok()
        .and_then(|content| parse_memory(&content))
    {
        Some((used, total)) => {
            let percent = if total == 0 {
                0
            } else {
                ((used as f64 / total as f64) * 100.0).round() as u64
            };
            format!(
                "{} / {} ({} %)",
                format_gib(used),
                format_gib(total),
                percent
            )
        }
        None => UNKNOWN.to_string(),
    }
}

// ==========================================
// Uptime
// ==========================================

/// Segundos de uptime a partir del contenido de /proc/uptime
/// ("segundos_uptime segundos_inactivo ...").
pub fn parse_uptime(uptime: &str) -> Option<f64> {
    uptime.split_whitespace().next()?.parse::<f64>().ok()
}

/// Formatea una duración en segundos de forma legible y compacta.
pub fn format_uptime(seconds: f64) -> String {
    let total = seconds.max(0.0) as u64;

    let days = total / 86_400;
    let hours = (total % 86_400) / 3_600;
    let minutes = (total % 3_600) / 60;

    match (days, hours, minutes) {
        (0, 0, m) => format!("{m} min"),
        (0, h, m) if m == 0 => format!("{h} h"),
        (0, h, m) => format!("{h} h {m} min"),
        (d, 0, 0) => format!("{d} d"),
        (d, h, 0) => format!("{d} d {h} h"),
        (d, h, m) => format!("{d} d {h} h {m} min"),
    }
}

pub fn get_uptime() -> String {
    fs::read_to_string("/proc/uptime")
        .ok()
        .and_then(|content| parse_uptime(&content))
        .map(format_uptime)
        .unwrap_or_else(|| UNKNOWN.to_string())
}

// ==========================================
// Kernel
// ==========================================

pub fn get_kernel() -> String {
    fs::read_to_string("/proc/sys/kernel/osrelease")
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|_| UNKNOWN.to_string())
}

// ==========================================
// Hostname
// ==========================================

pub fn get_hostname() -> String {
    fs::read_to_string("/proc/sys/kernel/hostname")
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|_| UNKNOWN.to_string())
}

// ==========================================
// Sistema Operativo
// ==========================================

pub fn parse_os_release(os_release: &str) -> Option<String> {
    os_release.lines().find_map(|line| {
        let (key, value) = line.split_once('=')?;
        if key.trim() != "PRETTY_NAME" {
            return None;
        }
        let value = value.trim().trim_matches('"').trim();
        if value.is_empty() {
            None
        } else {
            Some(value.to_string())
        }
    })
}

pub fn get_os() -> String {
    fs::read_to_string("/etc/os-release")
        .ok()
        .and_then(|content| parse_os_release(&content))
        .unwrap_or_else(|| UNKNOWN.to_string())
}

// ==========================================
// Arquitectura
// ==========================================

pub fn get_architecture() -> String {
    std::env::consts::ARCH.to_string()
}

// ==========================================
// Sistema de archivos de raíz
// ==========================================

/// Formatea el espacio de una línea "libres de total" a partir de bytes.
/// Es una función aparte para poder probarla sin montar nada.
pub fn format_filesystem(total_bytes: u64, available_bytes: u64) -> String {
    let gib = 1024.0 * 1024.0 * 1024.0;
    let total = total_bytes as f64 / gib;
    let available = available_bytes as f64 / gib;

    if total >= 100.0 {
        // A partir de 100 GiB los decimales son ruido.
        format!("{available:.0} GiB de {total:.0} GiB libres")
    } else {
        format!("{available:.1} GiB de {total:.1} GiB libres")
    }
}

/// Espacio libre y total de `/`, que es donde se instala el sistema.
pub fn get_root_filesystem() -> String {
    use std::ffi::CString;

    let Ok(path) = CString::new("/") else {
        return UNKNOWN.to_string();
    };

    // statvfs no asigna ni puede fallar por memoria: un zeroed es un valor
    // válido de entrada.
    let mut stat: libc::statvfs = unsafe { std::mem::zeroed() };
    let rc = unsafe { libc::statvfs(path.as_ptr(), &mut stat) };
    if rc != 0 || stat.f_frsize == 0 {
        return UNKNOWN.to_string();
    }

    let fragment = stat.f_frsize as u64;
    let total = (stat.f_blocks as u64).saturating_mul(fragment);
    // f_bavail son bloques para usuario sin privilegios; f_bfree incluye los
    // reservados a root y exagera el espacio libre.
    let available = (stat.f_bavail as u64).saturating_mul(fragment);

    format_filesystem(total, available)
}

// ==========================================
// Tests
// ==========================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_cpu_reads_model_name() {
        let cpuinfo = "processor\t: 0\nmodel name\t: AMD Ryzen 7 5800X 8-Core\n";
        assert_eq!(
            parse_cpu(cpuinfo),
            Some("AMD Ryzen 7 5800X 8-Core".to_string())
        );
    }

    #[test]
    fn parse_cpu_falls_back_to_arm_keys() {
        // aarch64 no publica "model name".
        let cpuinfo = "processor\t: 0\nModel name\t: bogus\nHardware\t: BCM2712\n";
        assert_eq!(parse_cpu(cpuinfo), Some("BCM2712".to_string()));

        let cpuinfo = "processor\t: 0\nCPU model\t: ARMv7 Processor rev 4\n";
        assert_eq!(
            parse_cpu(cpuinfo),
            Some("ARMv7 Processor rev 4".to_string())
        );
    }

    #[test]
    fn parse_cpu_returns_none_when_absent() {
        assert_eq!(parse_cpu("processor\t: 0\nBogoMIPS\t: 50.00\n"), None);
        assert_eq!(parse_cpu(""), None);
        assert_eq!(parse_cpu("model name\t: \n"), None);
    }

    const MEMINFO: &str = "\
MemTotal:       16384000 kB
MemFree:         2000000 kB
MemAvailable:    8192000 kB
Buffers:          500000 kB
Cached:          3000000 kB
";

    #[test]
    fn parse_memory_uses_mem_available() {
        let (used, total) = parse_memory(MEMINFO).expect("meminfo valido");
        assert_eq!(total, 16_384_000);
        assert_eq!(used, 16_384_000 - 8_192_000);
    }

    #[test]
    fn parse_memory_falls_back_without_mem_available() {
        let meminfo = "\
MemTotal:       1000 kB
MemFree:         100 kB
Buffers:         100 kB
Cached:          300 kB
SReclaimable:    100 kB
";
        let (used, total) = parse_memory(meminfo).expect("meminfo valido");
        assert_eq!(total, 1_000);
        // available = 100 + 100 + 300 + 100 = 600
        assert_eq!(used, 400);
    }

    #[test]
    fn parse_memory_needs_mem_total() {
        assert_eq!(parse_memory("MemFree: 100 kB\n"), None);
    }

    #[test]
    fn parse_uptime_reads_first_field() {
        assert_eq!(parse_uptime("3725.19 2100.00\n"), Some(3725.19));
        assert_eq!(parse_uptime(""), None);
        assert_eq!(parse_uptime("no-es-un-numero 1.0"), None);
    }

    #[test]
    fn format_uptime_is_compact() {
        assert_eq!(format_uptime(0.0), "0 min");
        assert_eq!(format_uptime(45.0), "0 min");
        assert_eq!(format_uptime(600.0), "10 min");
        assert_eq!(format_uptime(3600.0), "1 h");
        assert_eq!(format_uptime(3660.0), "1 h 1 min");
        assert_eq!(format_uptime(86_400.0), "1 d");
        assert_eq!(format_uptime(90_000.0), "1 d 1 h");
        assert_eq!(format_uptime(90_060.0), "1 d 1 h 1 min");
    }

    #[test]
    fn parse_os_release_reads_pretty_name() {
        let os_release = "NAME=\"ChurrOS\"\nPRETTY_NAME=\"ChurrOS 1.2 (rolling)\"\nID=churros\n";
        assert_eq!(
            parse_os_release(os_release),
            Some("ChurrOS 1.2 (rolling)".to_string())
        );
    }

    #[test]
    fn format_filesystem_switches_to_whole_gib_when_large() {
        const GIB: u64 = 1024 * 1024 * 1024;
        assert_eq!(
            format_filesystem(64 * GIB, 12 * GIB),
            "12 GiB de 64 GiB libres"
        );
        assert_eq!(
            format_filesystem(512 * GIB, 100 * GIB),
            "100 GiB de 512 GiB libres"
        );
        assert_eq!(
            format_filesystem(4 * GIB, 1536 * 1024 * 1024),
            "1.5 GiB de 4.0 GiB libres"
        );
    }

    #[test]
    fn parse_os_release_returns_none_when_absent() {
        assert_eq!(parse_os_release("NAME=\"ChurrOS\"\n"), None);
        assert_eq!(parse_os_release(""), None);
    }
}