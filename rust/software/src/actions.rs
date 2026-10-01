// ==========================================
// Acciones: instalar y desinstalar
//
// Ninguna app puede hacer esto sola, así que todo pasa por churros-pkexec
// (polkit) hacia churros-pkg. AUR va por otro lado a propósito: se lanza en
// una terminal como el usuario, porque compilar un PKGBUILD con makepkg como
// root sería root sin más.
//
// Los constructores de comandos son funciones puras y están probados: lo que
// se acaba ejecutar tiene que poder comprobarse sin ser root.
// ==========================================

use crate::model::{Package, Source};
use std::process::Command;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Install,
    Remove,
}

impl Action {
    pub fn verb(self) -> &'static str {
        match self {
            Action::Install => "install",
            Action::Remove => "remove",
        }
    }

    pub fn is_removing(self) -> bool {
        matches!(self, Action::Remove)
    }
}

/// Traducción de un error del helper a algo que una persona pueda entender.
pub fn describe_error(code: i32, stderr: &str) -> String {
    match code {
        0 => "Operación completada.".into(),
        1 => format!("Operación no válida. {stderr}"),
        2 => "El paquete ya no está en los repositorios.".into(),
        3 => format!("El gestor de paquetes falló. {stderr}"),
        _ => format!("Error inesperado ({code}). {stderr}"),
    }
}

/// Comando privilegiado para un paquete de Arch.
pub fn pacman_command(pkg: &Package, action: Action) -> Vec<String> {
    vec![
        "churros-pkexec".into(),
        "churros-pkg".into(),
        action.verb().into(),
        pkg.identifier().into(),
    ]
}

/// Comando privilegiado para un flatpak. El id va después de `--`, porque es
/// un dato que viene de la lista de la app, no de la persona.
pub fn flatpak_command(pkg: &Package, action: Action) -> Vec<String> {
    let sub = match (action, pkg.source) {
        // En AUR no se llega aquí.
        (_, Source::Aur) => "search",
        (Action::Install, _) => "install",
        (Action::Remove, _) => "uninstall",
    };
    vec![
        "churros-pkexec".into(),
        "flatpak".into(),
        sub.into(),
        "--".into(),
        pkg.identifier().into(),
    ]
}

/// Orden de terminal para instalar un paquete del AUR. Se lanza como el
/// usuario: yay pide el sudo él mismo y así el usuario ve qué se compila.
pub fn aur_command(pkg: &Package) -> String {
    format!(
        "yay -S --needed --noconfirm '{}'; echo; echo '[Churros] Proceso terminado. Pulsa Enter para cerrar.'; read -r",
        pkg.name.replace('\'', "'\\''")
    )
}

/// Validación previa del nombre de paquete, espejo de la del helper.
/// Duplicarla aquí evita pedir la contraseña para algo que se va a rechazar.
pub fn looks_like_package_name(name: &str) -> bool {
    !name.is_empty()
        && name.len() <= 255
        && name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || "@._+:-".contains(c))
        && !name.starts_with('-')
}

/// Ejecuta un comando privilegiado y devuelve (código, stderr).
pub fn run_privileged(argv: &[String]) -> (i32, String) {
    match Command::new(&argv[0]).args(&argv[1..]).output() {
        Ok(out) => (
            out.status.code().unwrap_or(-1),
            String::from_utf8_lossy(&out.stderr).trim().to_string(),
        ),
        Err(e) => (-1, e.to_string()),
    }
}

/// Abre una terminal con el comando indicado. Se usa solo para AUR.
pub fn spawn_terminal(command: &str) -> Result<(), String> {
    for term in ["foot", "alacritty", "kitty", "gnome-terminal", "xfce4-terminal"] {
        let argv: Vec<&str> = match term {
            // foot y los terminales XYZ reciben -e como argumento simple.
            "foot" | "alacritty" | "kitty" => vec![term, "-e", "sh", "-c", command],
            // Los de GNOME/XFCE separan cada argumento.
            _ => vec![term, "--", "sh", "-c", command],
        };
        if which(term).is_some() {
            return match Command::new(term).args(&argv[1..]).spawn() {
                Ok(_) => Ok(()),
                Err(e) => Err(format!("No se pudo abrir {term}: {e}")),
            };
        }
    }
    Err("No hay ninguna terminal instalada".into())
}

fn which(program: &str) -> Option<String> {
    let path = std::env::var_os("PATH")?;
    std::env::split_paths(&path).find_map(|dir| {
        let candidate = dir.join(program);
        candidate.is_file().then(|| candidate.to_string_lossy().into_owned())
    })
}

/// Decide cómo llevar a cabo la acción sobre un paquete, y devuelve el texto
/// que se le enseña a la persona.
pub fn plan(pkg: &Package, action: Action) -> Result<Plan, String> {
    if !looks_like_package_name(pkg.identifier()) {
        return Err(format!("El nombre «{}» no es válido", pkg.identifier()));
    }
    Ok(match pkg.source {
        Source::Pacman => Plan::Privileged(pacman_command(pkg, action)),
        Source::Flatpak => Plan::Privileged(flatpak_command(pkg, action)),
        Source::Aur => {
            if action.is_removing() {
                // El AUR no tiene uninstall: yay -Rns lo resuelve, y también
                // necesita sudo, así que va por la terminal.
                Plan::Terminal(format!(
                    "yay -Rns --noconfirm '{}'; read -r",
                    pkg.name.replace('\'', "'\\''")
                ))
            } else {
                Plan::Terminal(aur_command(pkg))
            }
        }
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Plan {
    /// Se ejecuta vía churros-pkexec, con diálogo de contraseña.
    Privileged(Vec<String>),
    /// Se abre en una terminal como el usuario.
    Terminal(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::Package;

    fn pacman(name: &str) -> Package {
        Package::new(name, Source::Pacman)
    }

    fn flatpak(id: &str) -> Package {
        Package::new(id, Source::Flatpak)
    }

    fn aur(name: &str) -> Package {
        Package::new(name, Source::Aur)
    }

    #[test]
    fn pacman_command_goes_through_the_helper() {
        let cmd = pacman_command(&pacman("firefox"), Action::Install);
        assert_eq!(cmd, vec!["churros-pkexec", "churros-pkg", "install", "firefox"]);
        assert!(cmd.contains(&"churros-pkg".to_string()));
        // Nunca pacman directo: la polka no lo permitiría con esta forma.
        assert!(!cmd.contains(&"pacman".to_string()));
    }

    #[test]
    fn pacman_command_for_remove() {
        let cmd = pacman_command(&pacman("firefox"), Action::Remove);
        assert_eq!(cmd[2], "remove");
    }

    #[test]
    fn flatpak_command_uses_the_right_subcommand_and_separator() {
        let cmd = flatpak_command(&flatpak("org.kde.krita"), Action::Install);
        assert_eq!(cmd[2], "install");
        assert!(cmd.contains(&"--".to_string()));

        let rm = flatpak_command(&flatpak("org.kde.krita"), Action::Remove);
        assert_eq!(rm[2], "uninstall");
        assert!(rm.contains(&"--".to_string()));
    }

    #[test]
    fn aur_command_quotes_and_runs_as_the_user() {
        let cmd = aur_command(&aur("mi-paquete"));
        assert!(cmd.starts_with("yay -S"), "{cmd}");
        assert!(cmd.contains("read -r"), "debe esperar antes de cerrar");
        assert!(!cmd.contains("pkexec"), "AUR nunca pasa por pkexec");
        assert!(!cmd.contains("sudo "), "yay pide el sudo él mismo");
    }

    #[test]
    fn aur_command_cannot_break_out_with_a_quote() {
        let cmd = aur_command(&aur("evil'; rm -rf /; echo '"));
        // La comilla suelta queda escapada, no cierra la cadena.
        assert_eq!(cmd.matches('\'').count() % 2, 0);
        assert!(cmd.contains("'\\''"));
    }

    #[test]
    fn aur_remove_uses_rns_in_a_terminal() {
        let cmd = aur_command(&aur("x"));
        let rm = format!("yay -Rns --noconfirm '{}'; read -r", "x");
        assert_eq!(Plan::Terminal(rm), plan(&aur("x"), Action::Remove).unwrap());
        assert!(!cmd.contains("Rns"));
    }

    #[test]
    fn plan_rejects_names_the_helper_would_reject() {
        for bad in ["--hookdir=/tmp/x", "-S", "con espacio", "../../etc/passwd", ""] {
            let pkg = pacman(bad);
            assert!(
                plan(&pkg, Action::Install).is_err(),
                "debería rechazar «{bad}»"
            );
        }
    }

    #[test]
    fn name_validation_accepts_real_package_names() {
        for good in ["firefox", "python-pywal", "gtk4", "lib32-gcc-libs", "7zip", "xterm+git"] {
            assert!(looks_like_package_name(good), "debería aceptar «{good}»");
        }
    }

    #[test]
    fn name_validation_rejects_option_shaped_names() {
        for bad in ["-S", "--needed", "a b", "a/b", "", "pkg;rm"] {
            assert!(!looks_like_package_name(bad), "debería rechazar «{bad}»");
        }
    }

    #[test]
    fn errors_are_translated_for_the_user() {
        assert_eq!(describe_error(2, ""), "El paquete ya no está en los repositorios.");
        assert!(describe_error(3, "boom").contains("boom"));
        assert!(describe_error(1, "malo").contains("malo"));
        assert!(describe_error(99, "").contains("99"));
    }

    #[test]
    fn plan_for_each_source() {
        assert!(matches!(plan(&pacman("a"), Action::Install).unwrap(), Plan::Privileged(_)));
        assert!(matches!(plan(&flatpak("o.x"), Action::Install).unwrap(), Plan::Privileged(_)));
        assert!(matches!(plan(&aur("a"), Action::Install).unwrap(), Plan::Terminal(_)));
    }
}
