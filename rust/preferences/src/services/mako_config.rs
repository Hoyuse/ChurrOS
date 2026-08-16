// ==========================================
// MakoConfig — ~/.config/mako/config
// (equivalente a services/dotfiles/mako_config.py)
// ==========================================

use std::fs;
use std::path::PathBuf;
use std::process::Command;

pub struct MakoConfig;

fn config_path() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/root".to_string());
    PathBuf::from(home).join(".config").join("mako").join("config")
}

fn read_lines() -> Vec<String> {
    match fs::read_to_string(config_path()) {
        Ok(content) => content.lines().map(|s| s.to_string()).collect(),
        Err(_) => Vec::new(),
    }
}

fn write_atomic(lines: &[String]) {
    let path = config_path();
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    let tmp = path.with_extension("cfg.tmp");
    let content = lines.join("\n");
    if fs::write(&tmp, content).is_ok() {
        let _ = fs::rename(&tmp, &path);
    }
}

fn find_section(lines: &[String], section: &str) -> isize {
    for (i, line) in lines.iter().enumerate() {
        if line.trim() == format!("[{section}]") {
            return i as isize;
        }
    }
    -1
}

fn set_key(lines: &mut Vec<String>, section: &str, key: &str, value: &str) {
    let mut section_idx = find_section(lines, section);
    if section_idx == -1 {
        lines.push(format!("\n[{section}]\n"));
        section_idx = (lines.len() - 1) as isize;
    }

    let mut end = lines.len();
    for j in (section_idx as usize + 1)..lines.len() {
        let stripped = lines[j].trim();
        if stripped.starts_with('[') && stripped.ends_with(']') {
            end = j;
            break;
        }
    }

    for j in (section_idx as usize + 1)..end {
        let stripped = lines[j].trim();
        if stripped.starts_with(&format!("{key}=")) || stripped.starts_with(&format!("{key} ")) {
            let prefix: String = lines[j]
                .chars()
                .take_while(|c| *c == ' ' || *c == '\t')
                .collect();
            lines[j] = format!("{prefix}{key}={value}");
            return;
        }
    }

    let insert = format!("{key}={value}");
    lines.insert(end, insert);
}

fn get_key(section: &str, key: &str, default: &str) -> Option<String> {
    let lines = read_lines();
    let idx = find_section(&lines, section);
    if idx < 0 {
        return Some(default.to_string());
    }
    for j in (idx as usize + 1)..lines.len() {
        let stripped = lines[j].trim();
        if stripped.starts_with('[') && stripped.ends_with(']') {
            break;
        }
        if let Some(rest) = stripped.strip_prefix(&format!("{key}=")) {
            return Some(rest.trim().to_string());
        }
    }
    Some(default.to_string())
}

/// None = clave ausente (usa el default pasado por el caller)
fn get_key_opt(section: &str, key: &str) -> Option<String> {
    let lines = read_lines();
    let idx = find_section(&lines, section);
    if idx < 0 {
        return None;
    }
    for j in (idx as usize + 1)..lines.len() {
        let stripped = lines[j].trim();
        if stripped.starts_with('[') && stripped.ends_with(']') {
            break;
        }
        if let Some(rest) = stripped.strip_prefix(&format!("{key}=")) {
            return Some(rest.trim().to_string());
        }
    }
    None
}

fn get_key_bool(section: &str, key: &str, default: bool) -> bool {
    match get_key_opt(section, key) {
        // mako acepta true/false/yes/no/1/0
        Some(v) => matches!(
            v.trim().to_ascii_lowercase().as_str(),
            "true" | "yes" | "1" | "on"
        ),
        None => default,
    }
}

fn get_key_int(section: &str, key: &str, default: i64) -> i64 {
    match get_key_opt(section, key) {
        Some(v) => v.parse().unwrap_or(default),
        None => default,
    }
}

impl MakoConfig {
    // ------------------------------------------------------------ Getters

    pub fn get_font() -> String {
        get_key("default", "font", "JetBrainsMono Nerd Font:size=11").unwrap()
    }

    pub fn get_background_color() -> String {
        get_key("default", "background-color", "#1e1e2e").unwrap()
    }

    pub fn get_text_color() -> String {
        get_key("default", "text-color", "#cdd6f4").unwrap()
    }

    pub fn get_border_color() -> String {
        get_key("default", "border-color", "#f97316").unwrap()
    }

    pub fn get_border_size() -> i64 {
        get_key_int("default", "border-size", 2)
    }

    pub fn get_border_radius() -> i64 {
        get_key_int("default", "border-radius", 8)
    }

    pub fn get_padding() -> String {
        get_key("default", "padding", "12,16").unwrap()
    }

    pub fn get_margin() -> i64 {
        get_key_int("default", "margin", 8)
    }

    pub fn get_default_timeout() -> i64 {
        get_key_int("default", "default-timeout", 5000)
    }

    pub fn get_width() -> i64 {
        get_key_int("default", "width", 380)
    }

    pub fn get_anchor() -> String {
        get_key("default", "anchor", "top-right").unwrap()
    }

    pub fn get_markup() -> bool {
        get_key_bool("default", "markup", true)
    }

    pub fn get_actions() -> bool {
        get_key_bool("default", "actions", true)
    }

    pub fn get_icons() -> bool {
        get_key_bool("default", "icons", true)
    }

    pub fn get_history() -> bool {
        get_key_bool("default", "history", true)
    }

    pub fn get_max_icon_size() -> i64 {
        get_key_int("default", "max-icon-size", 48)
    }

    // ------------------------------------------------------------- Setters

    pub fn set_font(font: &str) {
        let mut lines = read_lines();
        set_key(&mut lines, "default", "font", font);
        write_atomic(&lines);
    }

    pub fn set_appearance(
        background_color: Option<&str>,
        text_color: Option<&str>,
        border_color: Option<&str>,
        border_size: Option<i64>,
        border_radius: Option<i64>,
    ) {
        let mut lines = read_lines();
        if let Some(v) = background_color {
            set_key(&mut lines, "default", "background-color", v);
        }
        if let Some(v) = text_color {
            set_key(&mut lines, "default", "text-color", v);
        }
        if let Some(v) = border_color {
            set_key(&mut lines, "default", "border-color", v);
        }
        if let Some(v) = border_size {
            set_key(&mut lines, "default", "border-size", &v.to_string());
        }
        if let Some(v) = border_radius {
            set_key(&mut lines, "default", "border-radius", &v.to_string());
        }
        write_atomic(&lines);
    }

    pub fn set_layout(
        padding: Option<&str>,
        margin: Option<i64>,
        default_timeout: Option<i64>,
        width: Option<i64>,
    ) {
        let mut lines = read_lines();
        if let Some(v) = padding {
            set_key(&mut lines, "default", "padding", v);
        }
        if let Some(v) = margin {
            set_key(&mut lines, "default", "margin", &v.to_string());
        }
        if let Some(v) = default_timeout {
            set_key(&mut lines, "default", "default-timeout", &v.to_string());
        }
        if let Some(v) = width {
            set_key(&mut lines, "default", "width", &v.to_string());
        }
        write_atomic(&lines);
    }

    pub fn set_anchor(anchor: &str) {
        let mut lines = read_lines();
        set_key(&mut lines, "default", "anchor", anchor);
        write_atomic(&lines);
    }

    pub fn set_behaviors(
        markup: Option<bool>,
        actions: Option<bool>,
        icons: Option<bool>,
        history: Option<bool>,
        max_icon_size: Option<i64>,
    ) {
        let mut lines = read_lines();
        if let Some(v) = markup {
            set_key(&mut lines, "default", "markup", if v { "true" } else { "false" });
        }
        if let Some(v) = actions {
            set_key(&mut lines, "default", "actions", if v { "true" } else { "false" });
        }
        if let Some(v) = icons {
            set_key(&mut lines, "default", "icons", if v { "true" } else { "false" });
        }
        if let Some(v) = history {
            set_key(&mut lines, "default", "history", if v { "true" } else { "false" });
        }
        if let Some(v) = max_icon_size {
            set_key(&mut lines, "default", "max-icon-size", &v.to_string());
        }
        write_atomic(&lines);
    }

    #[allow(dead_code)] // portado por paridad; sin uso en las páginas actuales
    pub fn set_color(key: &str, hex_color: &str) {
        let mut lines = read_lines();
        set_key(&mut lines, "default", key, hex_color);
        write_atomic(&lines);
    }

    /// makoctl reload
    pub fn reload() {
        let _ = Command::new("makoctl")
            .args(["reload"])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn();
    }
}
