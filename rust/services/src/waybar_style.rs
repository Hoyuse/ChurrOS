// Helpers de CSS de Waybar: parchear sin regenerar el archivo,
// sanitizar selectores inválidos y normalizar colores para GTK.

use serde_json::{json, Map, Value};

/// Recorta `#rrggbbaa` a `#rrggbb`. GTK/@define-color de Waybar se cae
/// con hex de 8 dígitos en varias versiones.
pub fn css_color(value: &str) -> String {
    let v = value.trim().trim_end_matches(';').trim();
    let hex = v.strip_prefix('#').unwrap_or("");
    if (hex.len() == 6 || hex.len() == 8) && hex.chars().all(|c| c.is_ascii_hexdigit()) {
        format!("#{}", &hex[..6])
    } else {
        v.to_string()
    }
}

/// El CSS generado por churros-settings usaba `#custom/sep`, que GTK no
/// parsea (el widget de Waybar es `#custom-sep`). Eso tumbaba la barra
/// al recargar el estilo.
pub fn sanitize_selectors(css: &str) -> String {
    css.replace("#custom/", "#custom-")
}

/// El template "Liquid Glass (auto-generated)" pisaba el style.css del
/// skel (incluido `#custom-power`) y metía selectores inválidos.
pub fn should_replace_style(css: &str) -> bool {
    let t = css.trim();
    t.is_empty() || t.contains("auto-generated") || t.contains("#custom/")
}

/// Lee `@define-color nombre valor;` → objeto JSON {nombre: valor}.
pub fn parse_define_colors(css: &str) -> Value {
    let mut map = Map::new();
    for line in css.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with("@import") || line.starts_with("/*") {
            continue;
        }
        if let Some(rest) = line.strip_prefix("@define-color") {
            let rest = rest.trim();
            let mut parts = rest.splitn(2, char::is_whitespace);
            let Some(name) = parts.next().map(str::trim).filter(|s| !s.is_empty()) else {
                continue;
            };
            let Some(value) = parts.next().map(|s| s.trim().trim_end_matches(';').trim()) else {
                continue;
            };
            if value.is_empty() {
                continue;
            }
            map.insert(
                name.trim_start_matches('@').to_string(),
                json!(value.to_string()),
            );
        }
    }
    Value::Object(map)
}

/// El archivo de colores guarda `color4`/`color1`, no `accent`.
pub fn accent_hex(colors: &Value) -> Option<String> {
    ["accent", "color4", "color1"]
        .iter()
        .find_map(|key| colors.get(*key).and_then(Value::as_str))
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

/// Primera declaración `key:` (conserva indentación y `;`).
pub fn patch_first_decl(css: &str, key: &str, value: &str) -> String {
    let prefix = format!("{key}:");
    let ends_with_nl = css.ends_with('\n');
    let mut done = false;
    let mut lines: Vec<String> = Vec::new();
    for line in css.lines() {
        if done {
            lines.push(line.to_string());
            continue;
        }
        let trimmed = line.trim();
        if trimmed.starts_with(&prefix) {
            let indent: String = line.chars().take_while(|c| c.is_whitespace()).collect();
            let semi = if trimmed.ends_with(';') { ";" } else { "" };
            lines.push(format!("{indent}{key}: {value}{semi}"));
            done = true;
        } else {
            lines.push(line.to_string());
        }
    }
    let mut out = lines.join("\n");
    if ends_with_nl {
        out.push('\n');
    }
    out
}

/// Primer `alpha(@background, …)` del archivo (el del bloque `*`).
pub fn patch_first_background_alpha(css: &str, alpha: f64) -> String {
    const NEEDLE: &str = "alpha(@background,";
    let Some(start) = css.find(NEEDLE) else {
        return css.to_string();
    };
    let after = start + NEEDLE.len();
    let Some(rel) = css[after..].find(')') else {
        return css.to_string();
    };
    format!(
        "{} {:.2}{}",
        &css[..after],
        alpha.clamp(0.0, 1.0),
        &css[after + rel..]
    )
}

/// Aplica familia, tamaño y opacidad sobre el CSS existente.
pub fn patch_style(css: &str, font_family: &str, font_size: i64, bg_alpha: f64) -> String {
    let mut out = sanitize_selectors(css);
    out = patch_first_decl(&out, "font-family", &format!("'{font_family}'"));
    out = patch_first_decl(&out, "font-size", &format!("{font_size}px"));
    patch_first_background_alpha(&out, bg_alpha)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn skel_style() -> String {
        let path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../archiso/airootfs/etc/skel/.config/waybar/style.css"
        );
        std::fs::read_to_string(path).unwrap_or_else(|err| panic!("leer {path}: {err}"))
    }

    #[test]
    fn css_color_strips_alpha_hex() {
        assert_eq!(css_color("#2a1612cc"), "#2a1612");
        assert_eq!(css_color("#DE8636"), "#DE8636");
        assert_eq!(css_color("  #c9c4c3; "), "#c9c4c3");
        assert_eq!(css_color("rgba(0,0,0,0.5)"), "rgba(0,0,0,0.5)");
    }

    #[test]
    fn sanitize_fixes_custom_slash_selectors() {
        let raw = "#custom/sep { color: red; }\n#custom-power { color: blue; }\n";
        let out = sanitize_selectors(raw);
        assert!(out.contains("#custom-sep"));
        assert!(!out.contains("#custom/"));
        assert!(out.contains("#custom-power"));
    }

    #[test]
    fn should_replace_generated_and_invalid_css() {
        assert!(should_replace_style(""));
        assert!(should_replace_style(
            "/* ChurrOS Waybar — Liquid Glass   (auto-generated) */\n"
        ));
        assert!(should_replace_style("#custom/sep { }\n"));
        assert!(!should_replace_style(&skel_style()));
    }

    #[test]
    fn accent_falls_back_to_color4() {
        let colors = parse_define_colors(
            "@define-color background #111111;\n@define-color color4 #DE8636;\n",
        );
        assert_eq!(accent_hex(&colors).as_deref(), Some("#DE8636"));
        assert!(colors.get("accent").is_none());
    }

    #[test]
    fn patch_style_keeps_skel_rules() {
        let original = skel_style();
        let patched = patch_style(&original, "Inter", 16, 0.5);
        assert!(patched.contains("#workspaces"), "se perdió #workspaces");
        assert!(patched.contains("#custom-power"), "se perdió #custom-power");
        assert!(patched.contains("#mpris"), "se perdió #mpris");
        assert!(patched.contains("font-family: 'Inter';"));
        assert!(patched.contains("font-size: 16px;"));
        assert!(patched.contains("alpha(@background, 0.50)"));
        assert!(
            patched.contains("font-size: 10px"),
            "no debe tocar el font-size del indicador de grabación"
        );
        assert!(!patched.contains("#custom/"));
    }
}
