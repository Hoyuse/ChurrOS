// ==========================================
// Acciones: abrir URLs y lanzar el instalador
// (equivalente a utils/browser.py + utils/desktop.py)
// ==========================================

use gtk::prelude::*;

use std::path::PathBuf;
use std::process::Command;

const REPOSITORY: &str = "https://github.com/Hoyuse/ChurrOS";
const DISCORD: &str = "https://discord.gg/tkzAnsVs3";
const WIKI: &str = "https://github.com/Hoyuse/ChurrOS/wiki";
const WEBSITE: &str = "https://github.com/Hoyuse/ChurrOS";

fn open_url(url: &str) {
    if let Err(e) = gio::AppInfo::launch_default_for_uri(url, None::<&gio::AppLaunchContext>) {
        eprintln!("[welcome] error abriendo {url}: {e}");
    }
}

#[allow(dead_code)]
pub fn open_wiki() {
    open_url(WIKI);
}

#[allow(dead_code)]
pub fn open_website() {
    open_url(WEBSITE);
}

// Callbacks de las cards (reciben el botón que las lanzó)

pub fn github_clicked(_button: &gtk::Button) {
    open_url(REPOSITORY);
}

pub fn discord_clicked(_button: &gtk::Button) {
    open_url(DISCORD);
}

pub fn install_clicked(button: &gtk::Button) {
    launch_installer(button);
}

// ==========================================
// Lanzador de .desktop (sustituye a Gio.DesktopAppInfo,
// que no está expuesto en gio-rs 0.22)
// ==========================================

fn find_desktop_file(app_id: &str) -> Option<PathBuf> {
    let mut dirs: Vec<PathBuf> = std::env::var("XDG_DATA_DIRS")
        .unwrap_or_else(|_| "/usr/local/share:/usr/share".to_string())
        .split(':')
        .map(PathBuf::from)
        .collect();

    if let Ok(home) = std::env::var("HOME") {
        dirs.insert(0, PathBuf::from(home).join(".local/share"));
    }

    for dir in dirs {
        let candidate = dir.join("applications").join(app_id);
        if candidate.is_file() {
            return Some(candidate);
        }
    }

    None
}

fn desktop_exec(app_id: &str) -> Option<String> {
    let path = find_desktop_file(app_id)?;
    let content = std::fs::read_to_string(path).ok()?;

    let mut in_entry = false;
    for line in content.lines() {
        let line = line.trim();
        if line.starts_with('[') && line.ends_with(']') {
            in_entry = line == "[Desktop Entry]";
            continue;
        }
        if !in_entry {
            continue;
        }
        if let Some(exec) = line.strip_prefix("Exec=") {
            // Los field codes (%f, %u, %F, %U, %i, %c, %k) se descartan,
            // igual que hace GLib al expandir el campo Exec.
            let cleaned: String = exec
                .split(" %")
                .next()
                .unwrap_or(exec)
                .to_string();
            return Some(cleaned);
        }
    }

    None
}

fn launch_installer(parent: &gtk::Button) {
    if std::env::var("CHURROS_DEV").ok().as_deref() == Some("1") {
        eprintln!("[churros-dev] blocked: launch calamares");
        let dialog = gtk::AlertDialog::builder()
            .message("Preview: the installer is not launched on the host.")
            .modal(true)
            .build();
        if let Some(root) = parent.root().and_downcast::<gtk::Window>() {
            dialog.show(Some(&root));
        } else {
            dialog.show(None::<&gtk::Window>);
        }
        return;
    }

    match desktop_exec("calamares.desktop") {
        Some(exec) => {
            let launched = Command::new("sh")
                .arg("-c")
                .arg(&exec)
                .spawn()
                .map(|_| ())
                .is_ok();

            if !launched {
                show_alert(parent);
            }
        }
        None => show_alert(parent),
    }
}

fn show_alert(parent: &gtk::Button) {
    let dialog = gtk::AlertDialog::builder()
        .message("Installer not available on this system.")
        .modal(true)
        .build();

    if let Some(root) = parent.root().and_downcast::<gtk::Window>() {
        dialog.show(Some(&root));
    } else {
        dialog.show(None::<&gtk::Window>);
    }
}
