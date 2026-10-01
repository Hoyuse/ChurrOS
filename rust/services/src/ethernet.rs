// Servicio de ethernet vía nmcli (equivalente a services/ethernet.py)

use crate::run;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EthernetInfo {
    pub available: bool,
    pub device: Option<String>,
    pub connected: bool,
    pub connection: String,
    pub speed: Option<u32>,
    pub ip: Option<String>,
}

fn run_stdout(command: &[&str]) -> Option<(i32, String)> {
    let (code, out, _) = run(command, 5000)?;
    Some((code, out))
}

pub fn get() -> EthernetInfo {
    let mut info = EthernetInfo::default();

    let (code, out) = run_stdout(&[
        "nmcli", "-t", "-f", "DEVICE,TYPE,STATE,CONNECTION", "device",
    ])
    .unwrap_or((1, String::new()));

    if code != 0 {
        return info;
    }

    for line in out.lines() {
        let mut parts: Vec<&str> = line.splitn(4, ':').collect();
        while parts.len() < 4 {
            parts.push("");
        }
        let (device, dev_type, state, connection) =
            (parts[0], parts[1], parts[2], parts[3]);

        if dev_type != "ethernet" {
            continue;
        }

        info.available = true;
        info.device = Some(device.to_string());
        info.connected = state == "connected";
        info.connection = connection.to_string();
        break;
    }

    if !info.available {
        return info;
    }

    if info.connected {
        if let Some(dev) = &info.device {
            info.speed = speed(dev);
            info.ip = ip(dev);
        }
    }

    info
}

pub fn speed(device: &str) -> Option<u32> {
    std::fs::read_to_string(format!("/sys/class/net/{device}/speed"))
        .ok()?
        .trim()
        .parse::<u32>()
        .ok()
}

pub fn ip(device: &str) -> Option<String> {
    let (code, out) = run_stdout(&["ip", "-4", "addr", "show", device])?;
    if code != 0 {
        return None;
    }
    out.lines()
        .map(str::trim)
        .find(|l| l.starts_with("inet"))
        .and_then(|l| l.split_whitespace().nth(1))
        .map(|s| s.to_string())
}

pub fn disconnect() {
    let info = get();
    if let Some(device) = info.device {
        let _ = run(&["nmcli", "device", "disconnect", &device], 5000);
    }
}

pub fn connect() {
    let info = get();
    if let Some(device) = info.device {
        let _ = run(&["nmcli", "device", "connect", &device], 5000);
    }
}
