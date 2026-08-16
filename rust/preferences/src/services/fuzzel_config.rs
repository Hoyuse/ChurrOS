// ==========================================
// FuzzelConfig — ~/.config/fuzzel/fuzzel.ini
// (equivalente a services/dotfiles/fuzzel_config.py)
// ==========================================

use std::fs;
use std::path::PathBuf;
use std::process::Command;

pub struct FuzzelConfig;

fn config_path() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/root".to_string());
    PathBuf::from(home)
        .join(".config")
        .join("fuzzel")
        .join("fuzzel.ini")
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
    let tmp = path.with_extension("ini.tmp");
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

/// Equivalente a _set_key (sin indentar las líneas nuevas).
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

fn get_key(section: &str, key: &str, default: &str) -> String {
    let lines = read_lines();
    let idx = find_section(&lines, section);
    if idx < 0 {
        return default.to_string();
    }
    for j in (idx as usize + 1)..lines.len() {
        let stripped = lines[j].trim();
        if stripped.starts_with('[') && stripped.ends_with(']') {
            break;
        }
        if let Some(rest) = stripped.strip_prefix(&format!("{key}=")) {
            return rest.trim().to_string();
        }
    }
    default.to_string()
}

fn get_key_int(section: &str, key: &str, default: i64) -> i64 {
    get_key(section, key, &default.to_string())
        .parse()
        .unwrap_or(default)
}

impl FuzzelConfig {
    // ------------------------------------------------------------ Getters

    pub fn get_font() -> String {
        get_key("main", "font", "JetBrainsMono Nerd Font:size=13")
    }

    pub fn get_icon_theme() -> String {
        get_key("main", "icon-theme", "")
    }

    pub fn get_width() -> i64 {
        get_key_int("main", "width", 48)
    }

    pub fn get_lines() -> i64 {
        get_key_int("main", "lines", 12)
    }

    pub fn get_horizontal_pad() -> i64 {
        get_key_int("main", "horizontal-pad", 36)
    }

    pub fn get_vertical_pad() -> i64 {
        get_key_int("main", "vertical-pad", 14)
    }

    pub fn get_inner_pad() -> i64 {
        get_key_int("main", "inner-pad", 4)
    }

    pub fn get_line_height() -> i64 {
        get_key_int("main", "line-height", 24)
    }

    pub fn get_letter_spacing() -> i64 {
        get_key_int("main", "letter-spacing", 1)
    }

    // ------------------------------------------------------------- Setters

    pub fn set_font(font: &str) {
        let mut lines = read_lines();
        set_key(&mut lines, "main", "font", font);
        write_atomic(&lines);
    }

    pub fn set_icon_theme(theme: &str) {
        let mut lines = read_lines();
        set_key(&mut lines, "main", "icon-theme", theme);
        write_atomic(&lines);
    }

    #[allow(clippy::too_many_arguments)]
    pub fn set_layout(width: i64, lines_count: i64, h_pad: i64, v_pad: i64, inner_pad: i64, line_height: i64, letter_spacing: i64) {
        let mut lines = read_lines();
        set_key(&mut lines, "main", "width", &width.to_string());
        set_key(&mut lines, "main", "lines", &lines_count.to_string());
        set_key(&mut lines, "main", "horizontal-pad", &h_pad.to_string());
        set_key(&mut lines, "main", "vertical-pad", &v_pad.to_string());
        set_key(&mut lines, "main", "inner-pad", &inner_pad.to_string());
        set_key(&mut lines, "main", "line-height", &line_height.to_string());
        set_key(&mut lines, "main", "letter-spacing", &letter_spacing.to_string());
        write_atomic(&lines);
    }

    #[allow(dead_code)] // portado por paridad; sin uso en las páginas actuales
    pub fn set_color(key: &str, hex_color: &str) {
        let mut lines = read_lines();
        set_key(&mut lines, "colors", key, hex_color);
        write_atomic(&lines);
    }

    /// Recarga fuzzel (cierra la instancia actual para que relea su config).
    pub fn reload() {
        // OJO: `pkill -fuzzel` NO hace nada (se parsea como -f -u "zzel").
        let _ = Command::new("pkill")
            .args(["-x", "fuzzel"])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn();
    }
}
