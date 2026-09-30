use adw::prelude::*;
use gtk::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

use crate::pages::install;

pub fn build() -> gtk::Box {
    let container = gtk::Box::new(gtk::Orientation::Vertical, 10);
    container.set_valign(gtk::Align::Center);
    container.set_halign(gtk::Align::Center);

    let title = gtk::Label::new(Some("¿Para qué usarás tu sistema?"));
    title.add_css_class("page-title");
    title.set_halign(gtk::Align::Center);

    let subtitle = gtk::Label::new(Some("Despliega cada grupo y selecciona las herramientas que deseas instalar."));
    subtitle.add_css_class("page-subtitle");
    subtitle.set_halign(gtk::Align::Center);

    // Make the list scrollable in case there are many items
    let scrolled_window = gtk::ScrolledWindow::new();
    scrolled_window.set_policy(gtk::PolicyType::Never, gtk::PolicyType::Automatic);
    scrolled_window.set_min_content_height(400); // Give it some height
    scrolled_window.set_vexpand(true);
    
    let listbox = gtk::ListBox::new();
    listbox.add_css_class("boxed-list");
    listbox.set_margin_start(32);
    listbox.set_margin_end(32);
    listbox.set_selection_mode(gtk::SelectionMode::None);

    let categories = vec![
        ("Navegadores Web", "Explora internet de forma rápida y segura", vec![
            ("google-chrome", "Google Chrome"),
            ("brave-bin", "Brave Browser"),
            ("firefox", "Mozilla Firefox"),
            ("tor-browser", "Tor Browser"),
        ]),
        ("Ofimática y Productividad", "Herramientas para documentos y organización", vec![
            ("libreoffice-fresh", "LibreOffice (Suite Libre)"),
            ("onlyoffice-bin", "OnlyOffice (Suite Moderna)"),
            ("obsidian", "Obsidian (Notas y conocimiento)"),
            ("notion-app-electron", "Notion (Workspace)"),
            ("ttf-ms-fonts", "Fuentes de Microsoft"),
            ("hunspell-es_es", "Diccionario en Español"),
        ]),
        ("Gaming", "Steam, Lutris, optimizadores y comunicación", vec![
            ("steam", "Steam"),
            ("lutris", "Lutris (Gestor de juegos)"),
            ("heroic-games-launcher-bin", "Heroic (Lanzador Epic/GOG)"),
            ("wine", "Wine (Compatibilidad con Windows)"),
            ("gamemode", "GameMode (Optimizador de rendimiento)"),
            ("mangohud", "MangoHud (FPS Overlay)"),
            ("discord", "Discord (Chat y Voz)"),
        ]),
        ("Desarrollo y Programación", "Lenguajes, contenedores y editores de código", vec![
            ("visual-studio-code-bin", "Visual Studio Code"),
            ("neovim", "Neovim"),
            ("git", "Git (Control de versiones)"),
            ("docker", "Docker (Contenedores)"),
            ("docker-compose", "Docker Compose"),
            ("nodejs", "Node.js"),
            ("npm", "NPM (Gestor de paquetes)"),
            ("postman-bin", "Postman (Testing de APIs)"),
        ]),
        ("Multimedia y Edición", "Producción de foto, video y audio", vec![
            ("vlc", "VLC (Reproductor de medios)"),
            ("obs-studio", "OBS Studio (Streaming y grabación)"),
            ("gimp", "GIMP (Edición de imágenes)"),
            ("kdenlive", "Kdenlive (Edición de video)"),
            ("audacity", "Audacity (Edición de audio)"),
            ("blender", "Blender (Modelado 3D)"),
        ]),
        ("Herramientas del Sistema", "Monitoreo, respaldos y utilidades", vec![
            ("btop", "Btop (Monitor de recursos)"),
            ("gparted", "GParted (Gestor de particiones)"),
            ("timeshift", "Timeshift (Copias de seguridad del sistema)"),
            ("flameshot", "Flameshot (Capturas de pantalla avanzadas)"),
            ("qdirstat", "QDirStat (Análisis visual de almacenamiento)"),
            ("unrar", "Unrar (Soporte para archivos .rar)"),
        ]),
    ];

    for (name, desc, pkgs) in categories {
        let expander = adw::ExpanderRow::builder()
            .title(name)
            .subtitle(desc)
            .build();

        for (pkg_id, pkg_name) in pkgs {
            let row = adw::ActionRow::builder()
                .title(pkg_name)
                .subtitle(pkg_id)
                .build();

            let check_btn = gtk::CheckButton::new();
            check_btn.set_valign(gtk::Align::Center);
            
            let pkg_id_clone = pkg_id.to_string();
            check_btn.connect_toggled(move |btn| {
                if btn.is_active() {
                    install::add_packages(&[&pkg_id_clone]);
                } else {
                    install::remove_packages(&[&pkg_id_clone]);
                }
            });

            row.add_prefix(&check_btn);
            expander.add_row(&row);
        }

        listbox.append(&expander);
    }

    scrolled_window.set_child(Some(&listbox));

    container.append(&title);
    container.append(&subtitle);
    container.append(&scrolled_window);

    container
}
