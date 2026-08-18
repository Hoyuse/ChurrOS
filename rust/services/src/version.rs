// Distro version from the repo-root `VERSION` file (compile-time).

/// Product version baked into every crate that depends on `churros-services`.
///
/// The string is the contents of `VERSION` at the repository root, trimmed.
/// Changing that file and rebuilding the ISO is enough to update welcome,
/// Settings, and any other caller.
pub fn distro() -> &'static str {
    include_str!("../../../VERSION").trim()
}

/// `VERSION_ID` from `/etc/os-release`, falling back to [`distro`].
///
/// The ISO build stamps `VERSION_ID` into os-release so fastfetch and the
/// installed system can read the same number without going through Rust.
pub fn from_os_release() -> String {
    if let Ok(content) = std::fs::read_to_string("/etc/os-release") {
        for line in content.lines() {
            let Some(rest) = line.strip_prefix("VERSION_ID=") else {
                continue;
            };
            let value = rest.trim().trim_matches('"');
            if !value.is_empty() {
                return value.to_string();
            }
        }
    }
    distro().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distro_matches_version_file_on_disk() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../VERSION");
        let on_disk = std::fs::read_to_string(path)
            .unwrap_or_else(|err| panic!("no se pudo leer {path}: {err}"));
        let expected = on_disk.trim();
        assert!(!expected.is_empty(), "VERSION no puede estar vacío");
        assert!(
            expected
                .chars()
                .all(|c| c.is_ascii_digit() || c == '.'),
            "VERSION debe ser numérico con puntos, llegó {expected:?}"
        );
        assert_eq!(distro(), expected);
    }

    #[test]
    fn from_os_release_never_empty() {
        assert!(!from_os_release().is_empty());
    }
}
