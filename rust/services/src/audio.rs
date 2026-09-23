// Servicio de audio vía wpctl (equivalente a services/audio.py)

use crate::{run, spawn};

const SINK: &str = "@DEFAULT_AUDIO_SINK@";
const SOURCE: &str = "@DEFAULT_AUDIO_SOURCE@";

fn has_wpctl() -> bool {
    run(&["wpctl", "status"], 2000).is_some()
}

fn get_default_volume(target: &str) -> (u8, bool) {
    let Some((_, out, _)) = run(&["wpctl", "get-volume", target], 2000) else {
        return (0, false);
    };
    let out = out.trim();
    let volume = out
        .split_whitespace()
        .nth(1)
        .and_then(|v| v.parse::<f64>().ok())
        .unwrap_or(0.0);
    ( (volume * 100.0) as u8, out.contains("MUTED") )
}

#[derive(Debug, Clone)]
pub struct AudioDevice {
    pub id: u32,
    pub name: String,
    pub default: bool,
}

fn list_devices(kind: &str) -> Vec<AudioDevice> {
    let mut devices = Vec::new();
    let Some((_, out, _)) = run(&["wpctl", "status"], 2000) else {
        return devices;
    };

    let target = if kind == "sink" { "Sinks:" } else { "Sources:" };
    let mut in_section = false;

    for line in out.lines() {
        let stripped = line.trim();

        if stripped.ends_with(':') {
            if stripped.ends_with(target) {
                in_section = true;
            } else if in_section {
                break;
            }
            continue;
        }
        if !in_section || stripped.is_empty() {
            continue;
        }

        // Quitar los caracteres de árbol (│ ├ └ ─) que añade wpctl moderno.
        let body = stripped.trim_start_matches(|c: char| matches!(c, '│' | '├' | '└' | '─' | ' '));
        if body.is_empty() {
            continue;
        }

        // Entrada de dispositivo: "*   50. Nombre [vol: 0.85]" o "52. Nombre"
        let (is_default, body) = match body.strip_prefix('*') {
            Some(body) => (true, body.trim_start()),
            None => (false, body),
        };

        let mut parts = body.splitn(2, ". ");
        let (Some(id_str), Some(name)) = (parts.next(), parts.next()) else {
            continue;
        };
        let Ok(id) = id_str.trim().parse::<u32>() else {
            continue;
        };

        // Quitar el sufijo "[vol: ...]" que añaden wpctl modernos.
        let name = name.trim().split('[').next().unwrap_or(name).trim();

        devices.push(AudioDevice {
            id,
            name: name.to_string(),
            default: is_default,
        });
    }

    devices
}

pub fn available() -> bool {
    has_wpctl()
}

pub fn get_volume() -> u8 {
    get_default_volume(SINK).0
}

pub fn get_input_volume() -> u8 {
    get_default_volume(SOURCE).0
}

pub fn get_volume_status() -> (u8, bool) {
    get_default_volume(SINK)
}

pub fn is_muted() -> bool {
    get_default_volume(SINK).1
}

pub fn is_input_muted() -> bool {
    get_default_volume(SOURCE).1
}

pub fn set_volume(value: u8) {
    spawn(&["wpctl", "set-volume", SINK, &format!("{value}%")]);
}

pub fn set_input_volume(value: u8) {
    spawn(&["wpctl", "set-volume", SOURCE, &format!("{value}%")]);
}

pub fn set_mute(muted: bool) {
    spawn(&["wpctl", "set-mute", SINK, if muted { "1" } else { "0" }]);
}

pub fn set_input_mute(muted: bool) {
    spawn(&["wpctl", "set-mute", SOURCE, if muted { "1" } else { "0" }]);
}

pub fn list_sinks() -> Vec<AudioDevice> {
    list_devices("sink")
}

pub fn list_sources() -> Vec<AudioDevice> {
    list_devices("source")
}

pub fn set_default_sink(node_id: u32) {
    spawn(&["wpctl", "set-default", &node_id.to_string()]);
}
