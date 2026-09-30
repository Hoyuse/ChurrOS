use adw::prelude::*;
use gtk::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use std::process::Command;
use std::collections::HashSet;
use glib::clone;
use std::path::PathBuf;

thread_local! {
    static SELECTED_PACKAGES: RefCell<HashSet<String>> = RefCell::new(HashSet::new());
    static PROGRESS_BAR: RefCell<Option<gtk::ProgressBar>> = RefCell::new(None);
    static STATUS_LABEL: RefCell<Option<gtk::Label>> = RefCell::new(None);
}

pub fn add_packages(pkgs: &[&str]) {
    SELECTED_PACKAGES.with(|set| {
        for p in pkgs {
            set.borrow_mut().insert(p.to_string());
        }
    });
}

pub fn remove_packages(pkgs: &[&str]) {
    SELECTED_PACKAGES.with(|set| {
        for p in pkgs {
            set.borrow_mut().remove(*p);
        }
    });
}

pub fn build() -> gtk::Box {
    let container = gtk::Box::new(gtk::Orientation::Vertical, 10);
    container.set_valign(gtk::Align::Center);
    container.set_halign(gtk::Align::Center);

    let title_label = gtk::Label::new(Some("Instalando y Configurando"));
    title_label.add_css_class("page-title");
    title_label.set_halign(gtk::Align::Center);

    let subtitle = gtk::Label::new(Some("Estamos preparando tu sistema con los paquetes y controladores que elegiste."));
    subtitle.add_css_class("page-subtitle");
    subtitle.set_halign(gtk::Align::Center);
    subtitle.set_wrap(true);
    subtitle.set_justify(gtk::Justification::Center);

    let progress_bar = gtk::ProgressBar::new();
    progress_bar.set_margin_start(32);
    progress_bar.set_margin_end(32);

    let label = gtk::Label::new(Some("Esperando para iniciar..."));

    PROGRESS_BAR.with(|p| *p.borrow_mut() = Some(progress_bar.clone()));
    STATUS_LABEL.with(|l| *l.borrow_mut() = Some(label.clone()));

    container.append(&title_label);
    container.append(&subtitle);
    container.append(&progress_bar);
    container.append(&label);

    container
}

fn detect_drivers() -> Vec<String> {
    let mut drivers = Vec::new();
    
    // Simple GPU detection
    if let Ok(output) = Command::new("lspci").output() {
        let stdout = String::from_utf8_lossy(&output.stdout).to_lowercase();
        if stdout.contains("nvidia") {
            drivers.push("nvidia".to_string());
            drivers.push("nvidia-utils".to_string());
            drivers.push("nvidia-settings".to_string());
        } else if stdout.contains("amd") || stdout.contains("radeon") {
            drivers.push("xf86-video-amdgpu".to_string());
            drivers.push("vulkan-radeon".to_string());
        } else if stdout.contains("intel") {
            drivers.push("vulkan-intel".to_string());
        }
    }
    drivers
}

pub fn start_installation() {
    let mut pkgs: Vec<String> = SELECTED_PACKAGES.with(|set| {
        set.borrow().iter().cloned().collect()
    });

    let drivers = detect_drivers();
    pkgs.extend(drivers);

    if pkgs.is_empty() {
        STATUS_LABEL.with(|l| {
            if let Some(lbl) = l.borrow().as_ref() {
                lbl.set_label("No seleccionaste paquetes. ¡Todo listo!");
            }
        });
        PROGRESS_BAR.with(|p| {
            if let Some(bar) = p.borrow().as_ref() {
                bar.set_fraction(1.0);
            }
        });
        return;
    }

    STATUS_LABEL.with(|l| {
        if let Some(lbl) = l.borrow().as_ref() {
            lbl.set_label("Por favor, sigue las instrucciones en la terminal emergente...");
        }
    });

    // We start foot terminal running the yay installation.
    // This allows the user to see the process, enter passwords and diagnose issues.
    let pkgs_str = pkgs.join(" ");
    let sh_cmd = format!(
        "yay -Sy --needed --noconfirm {}; echo '\n[ChurrOS] Proceso terminado. Presiona Enter para cerrar.'; read", 
        pkgs_str
    );

    let args = vec![
        "foot".to_string(),
        "-a".to_string(),
        "churros-installer".to_string(), // App ID to set float rules if any
        "-W".to_string(), "80x24".to_string(),
        "-e".to_string(),
        "sh".to_string(),
        "-c".to_string(),
        sh_cmd,
    ];

    let os_args: Vec<&std::ffi::OsStr> = args.iter().map(|s| std::ffi::OsStr::new(s)).collect();

    let subprocess = match gio::Subprocess::newv(
        &os_args,
        gio::SubprocessFlags::NONE,
    ) {
        Ok(proc) => proc,
        Err(e) => {
            STATUS_LABEL.with(|l| {
                if let Some(lbl) = l.borrow().as_ref() {
                    lbl.set_label(&format!("Error al lanzar terminal: {}", e));
                }
            });
            return;
        }
    };

    // Start a pulsing animation
    let tick_id = gtk::glib::timeout_add_local(std::time::Duration::from_millis(100), || {
        PROGRESS_BAR.with(|p| {
            if let Some(bar) = p.borrow().as_ref() {
                bar.pulse();
            }
        });
        gtk::glib::ControlFlow::Continue
    });

    subprocess.wait_async(
        gio::Cancellable::NONE,
        move |_res| {
            tick_id.remove(); // Stop pulsing

            PROGRESS_BAR.with(|p| {
                if let Some(bar) = p.borrow().as_ref() {
                    bar.set_fraction(1.0);
                }
            });
            STATUS_LABEL.with(|l| {
                if let Some(lbl) = l.borrow().as_ref() {
                    lbl.set_label("¡Instalación completada!");
                }
            });
        }
    );
}

pub fn handle_autostart(active: bool) {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/root".to_string());
    let autostart_dir = PathBuf::from(home).join(".config/autostart");
    let desktop_file = autostart_dir.join("churros-tour.desktop");

    if active {
        // Asegurarnos de que exista si debe iniciar
        let _ = std::fs::create_dir_all(&autostart_dir);
        let content = "[Desktop Entry]\nType=Application\nName=ChurrOS Tour\nExec=churros-tour\nTerminal=false\nCategories=System;\n";
        let _ = std::fs::write(&desktop_file, content);
    } else {
        // Eliminarlo para que no inicie
        let _ = std::fs::remove_file(desktop_file);
    }
}
