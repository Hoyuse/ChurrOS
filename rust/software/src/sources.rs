// ==========================================
// Fuentes de paquetes
//
// Tres orígenes, un mismo modelo:
//   - pacman  : instalados (`pacman -Qq`) y catálogo (`pacman -Sl`)
//   - flatpak : instalados (`flatpak list`) y búsqueda en Flathub
//   - AUR     : RPC de archlinux.org vía curl
//
// Los parsers son funciones puras sobre texto: se prueban sin red y sin
// root, que es la única forma de que esto se pueda verificar en CI.
// ==========================================

use crate::model::{Package, Source};

const TIMEOUT: u32 = 20;

// ==========================================
// pacman
// ==========================================

/// Salida de `pacman -Sl` (catálogo de los repos):
/// ```
/// extra/dbus 1.16.2-1 [installed]
///     D-Bus message bus daemon
/// ```
/// Ojo: repositorio y nombre van en el MISMO campo separados por `/`, y la
/// versión en el siguiente. El repositorio puede traer epoch (`extra/foo
/// 1:2.0-1`) y `[installed]` solo aparece si el paquete está instalado.
pub fn parse_catalog(output: &str) -> Vec<Package> {
    let mut out = Vec::new();
    let mut current: Option<Package> = None;

    for raw in output.lines() {
        if raw.starts_with(' ') || raw.trim().is_empty() {
            if let Some(pkg) = current.as_mut() {
                let desc = raw.trim();
                // Si la descripción ocupa varias líneas, se queda con la
                // primera: es el resumen. Las siguientes no lo pisan.
                if !desc.is_empty() && pkg.description.is_empty() {
                    pkg.description = desc.to_string();
                }
            }
            continue;
        }

        if let Some(pkg) = current.take() {
            out.push(pkg);
        }

        let mut fields = raw.split_whitespace();
        // `repo/nombre` tiene que traer la barra: una línea sin ella no es una
        // entrada de catálogo (basura, avisos de pacman) y se descarta.
        let (Some(head), Some(version)) = (fields.next(), fields.next()) else {
            continue;
        };
        let Some((_repo, name)) = head.split_once('/') else {
            continue;
        };
        if name.is_empty() {
            continue;
        }
        let version = version.trim_end_matches(" [installed]");

        current = Some(Package {
            name: name.to_string(),
            version: version.to_string(),
            description: String::new(),
            source: Source::Pacman,
            installed: raw.contains("[installed]"),
            installed_version: None,
        });
    }

    if let Some(pkg) = current {
        out.push(pkg);
    }
    out
}

/// Salida de `pacman -Q`:
/// ```
/// bash 5.2.026-1
/// firefox 141.0-1
/// ```
pub fn parse_installed_pacman(output: &str) -> Vec<Package> {
    output
        .lines()
        .filter_map(|line| {
            let (name, version) = line.split_once(' ')?;
            if name.is_empty() {
                return None;
            }
            let mut pkg = Package::new(name, Source::Pacman);
            pkg.version = version.trim().to_string();
            pkg.installed = true;
            pkg.installed_version = Some(version.trim().to_string());
            Some(pkg)
        })
        .collect()
}

pub fn installed_pacman() -> Vec<Package> {
    match std::process::Command::new("pacman")
        .args(["-Q"])
        .output()
    {
        Ok(out) => parse_installed_pacman(&String::from_utf8_lossy(&out.stdout)),
        Err(_) => Vec::new(),
    }
}

/// Catálogo de los repos de Arch. Son ~90 000 paquetes, así que el resultado
/// se cachea en memoria (lo llama el modelo, no el botón de recargar).
pub fn catalog_pacman() -> Vec<Package> {
    match std::process::Command::new("pacman")
        .args(["-Sl"])
        .output()
    {
        Ok(out) => parse_catalog(&String::from_utf8_lossy(&out.stdout)),
        Err(_) => Vec::new(),
    }
}

// ==========================================
// flatpak
// ==========================================

/// Salida de `flatpak list --columns=name,version,application` (los campos van
/// separados por tabulador):
/// ```text
/// Firefox<TAB>120.0.1<TAB>org.mozilla.firefox
/// ```
pub fn parse_flatpak_list(output: &str) -> Vec<Package> {
    output
        .lines()
        .filter_map(|line| {
            let mut cols = line.split('\t').filter(|c| !c.is_empty());
            let name = cols.next()?;
            let version = cols.next().unwrap_or("");
            let id = cols.next().unwrap_or(name);
            if id.is_empty() {
                return None;
            }
            let mut pkg = Package::new(id, Source::Flatpak);
            pkg.version = version.to_string();
            pkg.description = name.to_string();
            pkg.installed = true;
            pkg.installed_version = Some(version.to_string());
            Some(pkg)
        })
        .collect()
}

/// Salida de `flatpak search --columns=name,version,application,description`:
/// cada resultado ocupa varias líneas y los campos vienen separados por
/// tabulador; un resultado sin descripción son menos columnas.
pub fn parse_flatpak_search(output: &str) -> Vec<Package> {
    output
        .lines()
        .filter_map(|line| {
            let cols: Vec<&str> = line.split('\t').collect();
            if cols.len() < 3 {
                return None;
            }
            let (name, version, id) = (cols[0], cols[1], cols[2]);
            if id.trim().is_empty() {
                return None;
            }
            let mut pkg = Package::new(id.trim(), Source::Flatpak);
            pkg.version = version.trim().to_string();
            pkg.description = name.trim().to_string();
            if let Some(d) = cols.get(3) {
                let d = d.trim();
                if !d.is_empty() {
                    pkg.description = format!("{name} · {d}");
                }
            }
            Some(pkg)
        })
        .collect()
}

pub fn installed_flatpak() -> Vec<Package> {
    match std::process::Command::new("flatpak")
        .args(["list", "--columns=name,version,application"])
        .output()
    {
        Ok(out) => parse_flatpak_list(&String::from_utf8_lossy(&out.stdout)),
        Err(_) => Vec::new(),
    }
}

pub fn search_flatpak(query: &str) -> Vec<Package> {
    if query.trim().is_empty() {
        return Vec::new();
    }
    match std::process::Command::new("flatpak")
        .args([
            "search",
            "--columns=name,version,application,description",
            query,
        ])
        .output()
    {
        Ok(out) => parse_flatpak_search(&String::from_utf8_lossy(&out.stdout)),
        Err(_) => Vec::new(),
    }
}

// ==========================================
// AUR
// ==========================================

/// Construye la URL de la RPC del AUR. Se separa del `curl` para poder
/// probarla: lo que se manda a la red tiene que poder comprobarse.
pub fn aur_search_url(query: &str) -> Option<String> {
    let q = query.trim();
    if q.is_empty() {
        return None;
    }
    // Se permiten espacios porque la búsqueda es de gente ("kde plasma") y se
    // codifican como %20. Todo lo demás fuera de este alfabeto se rechaza: un
    // `&` o un `=` manipulation los parámetros de la RPC.
    if !q
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.' | '+' | ' '))
    {
        return None;
    }
    Some(format!(
        "https://aur.archlinux.org/rpc/v5/search/{}",
        urlencode(q)
    ))
}

/// Sustituto mínimo de percent-encoding: la entrada ya está restringida a un
/// alfabeto seguro, así que solo hay que escapar el espacio.
fn urlencode(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' | '~' => c.to_string(),
            other => format!("%{:02X}", other as u32),
        })
        .collect()
}

/// Respuesta de la RPC v5:
/// ```json
/// {"resultcount":1,"results":[{"Name":"foo","Version":"1.0","Description":"...","NumVotes":12}]}
/// ```
pub fn parse_aur_search(json: &str) -> Vec<Package> {
    let Ok(value) = serde_json::from_str::<serde_json::Value>(json) else {
        return Vec::new();
    };
    let Some(results) = value.get("results").and_then(|r| r.as_array()) else {
        return Vec::new();
    };

    results
        .iter()
        .filter_map(|entry| {
            let name = entry.get("Name")?.as_str()?.trim().to_string();
            if name.is_empty() {
                return None;
            }
            let mut pkg = Package::new(name, Source::Aur);
            pkg.version = entry
                .get("Version")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let votes = entry.get("NumVotes").and_then(|v| v.as_i64()).unwrap_or(0);
            let desc = entry
                .get("Description")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .trim();
            pkg.description = if votes > 0 {
                format!("{desc} (+{votes} votos)")
            } else {
                desc.to_string()
            };
            Some(pkg)
        })
        .collect()
}

pub fn search_aur(query: &str) -> Vec<Package> {
    let Some(url) = aur_search_url(query) else {
        return Vec::new();
    };
    let out = match std::process::Command::new("curl")
        .args([
            "-fsSL",
            "--proto",
            "=https",
            "--tlsv1.2",
            "--connect-timeout",
            "10",
            "--max-time",
            "15",
            &url,
        ])
        .output()
    {
        Ok(o) => o,
        Err(_) => return Vec::new(),
    };
    if !out.status.success() {
        return Vec::new();
    }
    parse_aur_search(&String::from_utf8_lossy(&out.stdout))
}

// ==========================================
// Tests de los parsers
// ==========================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalog_reads_repo_version_and_description() {
        let out = "core/bash 5.2.026-1\n    The GNU Bourne Again shell\nextra/firefox 141.0-1 [installed]\n    A web browser\n";
        let pkgs = parse_catalog(out);
        assert_eq!(pkgs.len(), 2);

        assert_eq!(pkgs[0].name, "bash");
        assert_eq!(pkgs[0].version, "5.2.026-1");
        assert_eq!(pkgs[0].description, "The GNU Bourne Again shell");
        assert!(!pkgs[0].installed);

        assert_eq!(pkgs[1].name, "firefox");
        assert_eq!(pkgs[1].version, "141.0-1");
        assert!(pkgs[1].installed);
    }

    #[test]
    fn catalog_handles_epoch_and_multiline_description() {
        let out = "extra/foo 1:2.0-1\n    Line one\n    Line two\n";
        let pkgs = parse_catalog(out);
        assert_eq!(pkgs[0].version, "1:2.0-1");
        // La segunda línea no pisa el resumen con la segunda.
        assert_eq!(pkgs[0].description, "Line one");
    }

    #[test]
    fn catalog_ignores_garbage_lines() {
        let out = "\n\nbasura sin campos\ncore/bash 5.2-1\n    shell\n";
        let pkgs = parse_catalog(out);
        assert_eq!(pkgs.len(), 1);
        assert_eq!(pkgs[0].name, "bash");
    }

    #[test]
    fn installed_pacman_reads_name_and_version() {
        let out = "bash 5.2.026-1\nfirefox 141.0-1\n";
        let pkgs = parse_installed_pacman(out);
        assert_eq!(pkgs.len(), 2);
        assert_eq!(pkgs[0].name, "bash");
        assert_eq!(pkgs[0].version, "5.2.026-1");
        assert!(pkgs[0].installed);
        assert_eq!(pkgs[0].installed_version.as_deref(), Some("5.2.026-1"));
    }

    #[test]
    fn installed_pacman_skips_empty_lines() {
        let pkgs = parse_installed_pacman("bash 5.2-1\n\n");
        assert_eq!(pkgs.len(), 1);
    }

    #[test]
    fn flatpak_list_maps_application_id() {
        let out = "Firefox\t120.0.1\torg.mozilla.firefox\nKrita\t5.2.0\torg.kde.krita\n";
        let pkgs = parse_flatpak_list(out);
        assert_eq!(pkgs.len(), 2);
        assert_eq!(pkgs[0].name, "org.mozilla.firefox");
        assert_eq!(pkgs[0].version, "120.0.1");
        assert!(pkgs[0].installed);
        assert_eq!(pkgs[1].description, "Krita");
    }

    #[test]
    fn flatpak_search_reads_four_columns() {
        let out = "Firefox\t120.0.1\torg.mozilla.firefox\tWeb browser\n";
        let pkgs = parse_flatpak_search(out);
        assert_eq!(pkgs.len(), 1);
        assert_eq!(pkgs[0].name, "org.mozilla.firefox");
        assert!(pkgs[0].description.contains("Web browser"));
    }

    #[test]
    fn flatpak_search_accepts_three_columns() {
        let out = "Krita\t5.2.0\torg.kde.krita\n";
        let pkgs = parse_flatpak_search(out);
        assert_eq!(pkgs.len(), 1);
        assert_eq!(pkgs[0].name, "org.kde.krita");
    }

    #[test]
    fn flatpak_search_ignores_short_lines() {
        assert!(parse_flatpak_search("basura\n\tsolo\t\n").is_empty());
    }

    #[test]
    fn aur_url_is_https_and_percent_encoded() {
        let url = aur_search_url("mi paquete").expect("url");
        assert_eq!(url, "https://aur.archlinux.org/rpc/v5/search/mi%20paquete");
    }

    #[test]
    fn aur_url_rejects_empty_and_dangerous_queries() {
        assert!(aur_search_url("").is_none());
        assert!(aur_search_url("   ").is_none());
        // Una query con=/ o & podría manipular los parámetros de la RPC.
        assert!(aur_search_url("a&b=1").is_none());
        assert!(aur_search_url("../../etc/passwd").is_none());
        // Un espacio sí se acepta (se codifica), un & no.
    }

    #[test]
    fn aur_parse_reads_name_version_and_votes() {
        let json = r#"{"resultcount":1,"results":[
            {"Name":"mi-paquete","Version":"1.2-1","Description":"Hace cosas","NumVotes":42}
        ]}"#;
        let pkgs = parse_aur_search(json);
        assert_eq!(pkgs.len(), 1);
        assert_eq!(pkgs[0].name, "mi-paquete");
        assert_eq!(pkgs[0].version, "1.2-1");
        assert!(pkgs[0].description.contains("Hace cosas"));
        assert!(pkgs[0].description.contains("42"));
        assert_eq!(pkgs[0].source, Source::Aur);
        assert!(!pkgs[0].installed);
    }

    #[test]
    fn aur_parse_survives_broken_json() {
        assert!(parse_aur_search("no es json").is_empty());
        assert!(parse_aur_search("{\"resultcount\":0}").is_empty());
        assert!(parse_aur_search("{\"results\":[{\"Version\":\"1\"}]}").is_empty());
    }

    #[test]
    fn aur_parse_without_votes() {
        let json = r#"{"results":[{"Name":"x","Version":"1","Description":"d"}]}"#;
        let pkgs = parse_aur_search(json);
        assert_eq!(pkgs[0].description, "d");
    }
}
