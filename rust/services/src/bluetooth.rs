// Servicio de bluetooth vía bluetoothctl/rfkill (equivalente a services/bluetooth.py)

use std::path::Path;

use crate::{run, spawn, which};

const SHOW_TIMEOUT: u64 = 2000;

fn show_output() -> Option<String> {
    let out = run(&["bluetoothctl", "show"], SHOW_TIMEOUT)?;
    Some(out.1)
}

fn powered_from(show: &str) -> bool {
    show.lines()
        .find(|l| l.contains("Powered:"))
        .map(|l| l.splitn(2, ':').nth(1).unwrap_or("").trim().to_lowercase() == "yes")
        .unwrap_or(false)
}

#[derive(Debug, Clone)]
pub struct BtDevice {
    pub address: String,
    pub name: String,
    pub connected: bool,
}

pub fn available() -> bool {
    if !which("bluetoothctl") {
        return Path::new("/sys/class/bluetooth").is_dir();
    }
    match show_output() {
        Some(out) => !out.contains("No default controller"),
        None => false,
    }
}

pub fn is_enabled() -> bool {
    if !which("bluetoothctl") {
        return false;
    }
    match show_output() {
        Some(out) => powered_from(&out),
        None => false,
    }
}

pub fn is_blocked() -> bool {
    if !which("rfkill") || !which("bluetoothctl") {
        return false;
    }
    match run(&["rfkill", "list", "bluetooth"], 2000) {
        Some((_, out, _)) => {
            out.contains("Soft blocked: yes") || out.contains("Hard blocked: yes")
        }
        None => false,
    }
}

pub fn enable() -> bool {
    if !which("bluetoothctl") {
        return false;
    }
    run(&["bluetoothctl", "power", "on"], 3000).is_some()
}

pub fn disable() -> bool {
    if !which("bluetoothctl") {
        return false;
    }
    run(&["bluetoothctl", "power", "off"], 3000).is_some()
}

pub fn scan_start() {
    if which("bluetoothctl") {
        spawn(&["bluetoothctl", "--timeout", "10", "scan", "on"]);
    }
}

pub fn scan_stop() {
    if which("bluetoothctl") {
        spawn(&["bluetoothctl", "scan", "off"]);
    }
}

fn is_connected(address: &str) -> bool {
    match run(&["bluetoothctl", "info", address], 2000) {
        Some((_, out, _)) => out
            .lines()
            .find(|l| l.contains("Connected:"))
            .map(|l| l.splitn(2, ':').nth(1).unwrap_or("").trim().to_lowercase() == "yes")
            .unwrap_or(false),
        None => false,
    }
}

pub fn list_devices() -> Vec<BtDevice> {
    let mut devices = Vec::new();
    if !which("bluetoothctl") {
        return devices;
    }
    let Some((_, out, _)) = run(&["bluetoothctl", "devices"], 3000) else {
        return devices;
    };

    for line in out.lines() {
        let line = line.trim();
        if let Some(rest) = line.strip_prefix("Device ") {
            let mut parts = rest.splitn(3, ' ');
            if let (Some(addr), Some(_), Some(name)) = (parts.next(), parts.next(), parts.next()) {
                devices.push(BtDevice {
                    address: addr.to_string(),
                    name: name.to_string(),
                    connected: is_connected(addr),
                });
            }
        }
    }

    devices
}

pub fn connected_device() -> Option<String> {
    if !which("bluetoothctl") {
        return None;
    }
    let (_, out, _) = run(&["bluetoothctl", "devices", "Connected"], 1500)?;
    for line in out.lines() {
        let line = line.trim();
        if let Some(rest) = line.strip_prefix("Device ") {
            let mut parts = rest.splitn(2, ' ');
            let (_addr, name) = (parts.next(), parts.next());
            if let Some(name) = name {
                let name = name.trim();
                if !name.is_empty() {
                    return Some(name.to_string());
                }
            }
        }
    }
    None
}

pub fn connect(address: &str) -> bool {
    if !which("bluetoothctl") {
        return false;
    }
    run(&["bluetoothctl", "connect", address], 10_000).is_some()
}

pub fn disconnect(address: &str) -> bool {
    if !which("bluetoothctl") {
        return false;
    }
    run(&["bluetoothctl", "disconnect", address], 5000).is_some()
}

pub fn pair(address: &str) -> bool {
    if !which("bluetoothctl") {
        return false;
    }
    run(&["bluetoothctl", "pair", address], 15_000).is_some()
}

pub fn remove(address: &str) -> bool {
    if !which("bluetoothctl") {
        return false;
    }
    run(&["bluetoothctl", "remove", address], 3000).is_some()
}
