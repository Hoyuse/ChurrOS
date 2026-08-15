// Parser JSONC mínimo: comentarios // fuera de strings + trailing commas.
// Extraído de WaybarService para poder testearlo sin GTK ni $HOME.

use serde_json::Value;

/// El texto no es JSON válido después de quitar comentarios y comas finales.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JsoncError(pub String);

/// Quita comentarios `//` que no estén dentro de un string.
pub fn strip_line_comments(raw: &str) -> String {
    let mut cleaned = String::with_capacity(raw.len());
    let mut in_string = false;
    let mut in_comment = false;
    let mut escape = false;

    for ch in raw.chars() {
        if in_comment {
            if ch == '\n' {
                in_comment = false;
                cleaned.push('\n');
            }
            continue;
        }
        if in_string {
            cleaned.push(ch);
            if escape {
                escape = false;
                continue;
            }
            if ch == '\\' {
                escape = true;
                continue;
            }
            if ch == '"' {
                in_string = false;
            }
            continue;
        }
        if ch == '/' && cleaned.ends_with('/') {
            cleaned.pop();
            in_comment = true;
            continue;
        }
        if ch == '"' {
            in_string = true;
        }
        cleaned.push(ch);
    }

    cleaned
}

/// Elimina comas finales: `,}` → `}` y `,]` → `]`.
pub fn remove_trailing_commas(text: &str) -> String {
    let chars: Vec<char> = text.chars().collect();
    let mut out = String::with_capacity(text.len());
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        if c == ',' {
            let mut j = i + 1;
            while j < chars.len()
                && (chars[j] == ' ' || chars[j] == '\t' || chars[j] == '\n' || chars[j] == '\r')
            {
                j += 1;
            }
            if j < chars.len() && (chars[j] == '}' || chars[j] == ']') {
                i += 1;
                continue;
            }
        }
        out.push(c);
        i += 1;
    }
    out
}

/// Parsea JSONC. Si el texto no es válido, devuelve error (nunca un `{}` silencioso).
pub fn parse(text: &str) -> Result<Value, JsoncError> {
    let stripped = strip_line_comments(text);
    let cleaned = remove_trailing_commas(&stripped);
    serde_json::from_str(&cleaned).map_err(|err| JsoncError(err.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn plain_json() {
        let value = parse(r#"{"position":"top","height":30}"#).unwrap();
        assert_eq!(value["position"], "top");
        assert_eq!(value["height"], 30);
    }

    #[test]
    fn strips_line_comments() {
        let raw = r#"
        {
            // barra arriba
            "position": "top",
            "height": 30 // px
        }
        "#;
        let value = parse(raw).unwrap();
        assert_eq!(value["position"], "top");
        assert_eq!(value["height"], 30);
    }

    #[test]
    fn keeps_slashes_inside_strings() {
        let value = parse(r#"{"url":"https://churros.local","note":"usa // así"}"#).unwrap();
        assert_eq!(value["url"], "https://churros.local");
        assert_eq!(value["note"], "usa // así");
    }

    #[test]
    fn keeps_escaped_quotes_inside_strings() {
        let value = parse(r#"{"label":"di \"hola\""}"#).unwrap();
        assert_eq!(value["label"], r#"di "hola""#);
    }

    #[test]
    fn removes_trailing_commas() {
        let raw = r#"
        {
            "modules": ["clock", "battery",],
            "nested": { "a": 1, },
        }
        "#;
        let value = parse(raw).unwrap();
        assert_eq!(value["modules"], json!(["clock", "battery"]));
        assert_eq!(value["nested"]["a"], 1);
    }

    #[test]
    fn invalid_json_is_error_not_empty_object() {
        // El fallo que #18 quería pillar: un parse roto no puede parecer un archivo vacío.
        assert!(parse("{ esto no es json").is_err());
    }

    #[test]
    fn empty_object_is_valid() {
        assert_eq!(parse("{}").unwrap(), json!({}));
    }

    #[test]
    fn parses_skel_waybar_config() {
        let path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../archiso/airootfs/etc/skel/.config/waybar/config.jsonc"
        );
        let raw = std::fs::read_to_string(path)
            .unwrap_or_else(|err| panic!("no se pudo leer {path}: {err}"));
        let value = parse(&raw).expect("el config.jsonc del skel tiene que parsear");
        assert_eq!(value["position"], "top");
        assert_eq!(value["layer"], "top");
        let right = value["modules-right"]
            .as_array()
            .expect("modules-right es una lista");
        assert!(right.iter().any(|m| m == "clock"));
        assert!(value.get("niri/workspaces").is_some());
    }
}
