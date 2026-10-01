// ==========================================
// Modelo de datos de la tienda
//
// Todo lo que se puede probar sin GTK vive aquí: los parsers de la salida de
// pacman, flatpak y de la API del AUR, y la lógica de filtrado.
// ==========================================

use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Source {
    /// Repos de Arch (los de pacman.conf).
    Pacman,
    /// Paquetes planos de Flathub.
    Flatpak,
    /// AUR.
    Aur,
}

impl Source {
    pub fn label(self) -> &'static str {
        match self {
            Source::Pacman => "Arch",
            Source::Flatpak => "Flatpak",
            Source::Aur => "AUR",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub description: String,
    pub source: Source,
    /// Si está instalado en el sistema.
    pub installed: bool,
    /// Versión instalada, si la hay. Solo informativo: la lista de instalados
    /// de pacman no siempre trae descripción.
    pub installed_version: Option<String>,
}

impl Package {
    pub fn new(name: impl Into<String>, source: Source) -> Self {
        Package {
            name: name.into(),
            version: String::new(),
            description: String::new(),
            source,
            installed: false,
            installed_version: None,
        }
    }

    /// Identificador para Flatpak: `org.kde.krita` no tiene versión, así que
    /// se identifica por id completo.
    pub fn identifier(&self) -> &str {
        &self.name
    }

    /// Texto que se busca cuando el usuario escribe en el buscador.
    pub fn matches(&self, needle: &str) -> bool {
        if needle.is_empty() {
            return true;
        }
        let n = needle.to_lowercase();
        self.name.to_lowercase().contains(&n)
            || self.description.to_lowercase().contains(&n)
    }
}

/// Mezcla el catálogo disponible con la lista de instalados.
///
/// Un paquete puede estar disponible en Arch y estar además instalado desde
/// Flatpak (por ejemplo `ffmpeg`). Se conserva la entrada de Arch y se marca
/// como instalada, en vez de duplicar la fila.
pub fn merge(installed: &[Package], available: Vec<Package>) -> Vec<Package> {
    let mut by_name: BTreeMap<(Source, String), Package> = BTreeMap::new();

    for pkg in available {
        by_name.insert((pkg.source, pkg.name.clone()), pkg);
    }

    for inst in installed {
        match by_name.get_mut(&(inst.source, inst.name.clone())) {
            Some(entry) => {
                entry.installed = true;
                entry.installed_version = Some(inst.version.clone());
            }
            None => {
                // Instalado pero no está en el catálogo (repo deshabilitado,
                // AUR sin_index, paquete local...): se añade para que el
                // usuario lo vea y pueda desinstalarlo.
                let mut pkg = inst.clone();
                pkg.installed = true;
                pkg.installed_version = Some(inst.version.clone());
                by_name.insert((pkg.source, pkg.name.clone()), pkg);
            }
        }
    }

    by_name.into_values().collect()
}

/// Filtra por texto y por origen, y ordena: instalados primero y luego por
/// nombre, que es lo que espera la persona que busca una herramienta.
pub fn filter(packages: &[Package], query: &str, sources: &[Source]) -> Vec<Package> {
    let mut out: Vec<Package> = packages
        .iter()
        .filter(|p| sources.is_empty() || sources.contains(&p.source))
        .filter(|p| p.matches(query))
        .cloned()
        .collect();

    out.sort_by(|a, b| {
        b.installed
            .cmp(&a.installed)
            .then_with(|| a.source.cmp(&b.source))
            .then_with(|| a.name.to_lowercase().cmp(&b.name.to_lowercase()))
    });

    out
}

/// Resumen para la cabecera: cuántos instalados y cuántos en total.
pub fn counts(packages: &[Package]) -> (usize, usize) {
    let installed = packages.iter().filter(|p| p.installed).count();
    (installed, packages.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pkg(name: &str, source: Source) -> Package {
        Package::new(name, source)
    }

    #[test]
    fn matches_is_case_insensitive_and_covers_description() {
        let mut p = pkg("ffmpeg", Source::Pacman);
        p.description = "Multimedia tools".into();
        assert!(p.matches("FFMPEG"));
        assert!(p.matches("multimedia"));
        assert!(!p.matches("kde"));
        assert!(p.matches(""));
    }

    #[test]
    fn merge_marks_installed_and_keeps_description() {
        let installed = vec![pkg("ffmpeg", Source::Pacman)];
        let mut available = pkg("ffmpeg", Source::Pacman);
        available.description = "Multimedia".into();

        let merged = merge(&installed, vec![available]);
        assert_eq!(merged.len(), 1);
        assert!(merged[0].installed);
        assert_eq!(merged[0].description, "Multimedia");
    }

    #[test]
    fn merge_does_not_mix_sources() {
        // ffmpeg de Arch y ffmpeg de Flatpak son entradas distintas.
        let installed = vec![pkg("ffmpeg", Source::Flatpak)];
        let merged = merge(&installed, vec![pkg("ffmpeg", Source::Pacman)]);
        assert_eq!(merged.len(), 2);
        assert_eq!(merged.iter().filter(|p| p.installed).count(), 1);
    }

    #[test]
    fn merge_keeps_installed_packages_missing_from_the_catalog() {
        let installed = vec![pkg("mi-paquete-local", Source::Pacman)];
        let merged = merge(&installed, vec![]);
        assert_eq!(merged.len(), 1);
        assert!(merged[0].installed);
    }

    #[test]
    fn filter_selects_sources() {
        let pkgs = vec![
            pkg("a", Source::Pacman),
            pkg("b", Source::Flatpak),
            pkg("c", Source::Aur),
        ];
        let only_aur = filter(&pkgs, "", &[Source::Aur]);
        assert_eq!(only_aur.len(), 1);
        assert_eq!(only_aur[0].source, Source::Aur);

        let all = filter(&pkgs, "", &[]);
        assert_eq!(all.len(), 3);
    }

    #[test]
    fn filter_sorts_installed_first() {
        let mut installed = pkg("zzz-instalado", Source::Pacman);
        installed.installed = true;
        let pkgs = vec![pkg("aaa-disponible", Source::Pacman), installed];
        let out = filter(&pkgs, "", &[]);
        assert_eq!(out[0].name, "zzz-instalado");
        assert_eq!(out[1].name, "aaa-disponible");
    }

    #[test]
    fn counts_reports_installed_and_total() {
        let mut a = pkg("a", Source::Pacman);
        a.installed = true;
        let pkgs = vec![a, pkg("b", Source::Pacman), pkg("c", Source::Flatpak)];
        assert_eq!(counts(&pkgs), (1, 3));
    }
}
