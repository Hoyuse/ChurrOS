use std::cell::{Cell, RefCell};
use std::collections::HashSet;
use std::path::PathBuf;
use std::process::Command;
use std::rc::Rc;
use std::time::Duration;

use adw::prelude::*;
use glib::ControlFlow;
use gtk::prelude::*;

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

fn detect_terminal() -> (String, Vec<String>) {
    let ed = churros_services::version::edition();
    if ed == "kde" && which_exists("konsole") {
        return ("konsole".to_string(), vec!["--hide-menubar".to_string(), "-e".to_string()]);
    } else if (ed == "xfce" || ed == "server") && which_exists("xfce4-terminal") {
        return ("xfce4-terminal".to_string(), vec!["--hide-menubar".to_string(), "-x".to_string()]);
    }

    if which_exists("foot") {
        ("foot".to_string(), vec!["-a".to_string(), "churros-installer".to_string(), "-W".to_string(), "80x24".to_string(), "-e".to_string()])
    } else if which_exists("konsole") {
        ("konsole".to_string(), vec!["--hide-menubar".to_string(), "-e".to_string()])
    } else if which_exists("xfce4-terminal") {
        ("xfce4-terminal".to_string(), vec!["--hide-menubar".to_string(), "-x".to_string()])
    } else {
        ("xterm".to_string(), vec!["-e".to_string()])
    }
}

fn which_exists(bin: &str) -> bool {
    Command::new("which")
        .arg(bin)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

pub fn start_installation(window: adw::ApplicationWindow) {
    let mut pkgs: Vec<String> = SELECTED_PACKAGES.with(|set| {
        set.borrow().iter().cloned().collect()
    });

    let drivers = detect_drivers();
    pkgs.extend(drivers);

    if pkgs.is_empty() {
        STATUS_LABEL.with(|l| {
            if let Some(lbl) = l.borrow().as_ref() {
                lbl.set_label("No seleccionaste paquetes. ¡Todo listo! Cerrando...");
            }
        });
        PROGRESS_BAR.with(|p| {
            if let Some(bar) = p.borrow().as_ref() {
                bar.set_fraction(1.0);
            }
        });
        handle_autostart(false);
        let w = window.clone();
        glib::timeout_add_local(Duration::from_millis(1000), move || {
            w.close();
            ControlFlow::Break
        });
        return;
    }

    STATUS_LABEL.with(|l| {
        if let Some(lbl) = l.borrow().as_ref() {
            lbl.set_label("Instalando paquetes seleccionados...");
        }
    });

    let pkgs_str = pkgs.join(" ");
    let sh_cmd = format!(
        "yay -Sy --needed --noconfirm {}; STATUS=$?; if [ $STATUS -eq 0 ]; then echo '\n[ChurrOS] ¡Instalación completada con éxito!'; sleep 2; else echo '\n[ChurrOS] Ocurrió un error. Presiona Enter para cerrar.'; read; fi", 
        pkgs_str
    );

    let (term_bin, mut term_flags) = detect_terminal();
    let mut args = vec![term_bin];
    args.append(&mut term_flags);
    args.push("sh".to_string());
    args.push("-c".to_string());
    args.push(sh_cmd);

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

    let is_running = Rc::new(Cell::new(true));
    let is_running_pulse = is_running.clone();

    // Start a pulsing animation
    glib::timeout_add_local(Duration::from_millis(100), move || {
        if !is_running_pulse.get() {
            return ControlFlow::Break;
        }
        PROGRESS_BAR.with(|p| {
            if let Some(bar) = p.borrow().as_ref() {
                bar.pulse();
            }
        });
        ControlFlow::Continue
    });

    let window_clone = window.clone();
    let is_running_finish = is_running.clone();
    subprocess.wait_async(
        gio::Cancellable::NONE,
        move |_res| {
            is_running_finish.set(false); // Stop pulsing

            PROGRESS_BAR.with(|p| {
                if let Some(bar) = p.borrow().as_ref() {
                    bar.set_fraction(1.0);
                }
            });
            STATUS_LABEL.with(|l| {
                if let Some(lbl) = l.borrow().as_ref() {
                    lbl.set_label("¡Instalación completada! Cerrando ChurrOS Tour...");
                }
            });

            handle_autostart(false);

            let w = window_clone.clone();
            glib::timeout_add_local(Duration::from_millis(800), move || {
                w.close();
                ControlFlow::Break
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
