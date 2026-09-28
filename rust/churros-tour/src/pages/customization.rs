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

    let subtitle = gtk::Label::new(Some("Selecciona los paquetes que deseas instalar para preparar tu ChurrOS."));
    subtitle.add_css_class("page-subtitle");
    subtitle.set_halign(gtk::Align::Center);

    let listbox = gtk::ListBox::new();
    listbox.add_css_class("boxed-list");
    listbox.set_margin_start(32);
    listbox.set_margin_end(32);
    listbox.set_selection_mode(gtk::SelectionMode::None);

    let categories = vec![
        ("Ofimática", "LibreOffice, fuentes y herramientas", vec!["libreoffice-fresh", "ttf-ms-fonts", "hunspell-es_es"]),
        ("Gaming", "Steam, Lutris, Wine, Heroic", vec!["steam", "lutris", "wine", "heroic-games-launcher-bin", "gamemode"]),
        ("Programación", "VS Code, Git, Docker", vec!["visual-studio-code-bin", "git", "docker", "docker-compose"]),
        ("Multimedia", "VLC, OBS, GIMP", vec!["vlc", "obs-studio", "gimp"]),
    ];

    for (name, desc, pkgs) in categories {
        let row = adw::ActionRow::builder()
            .title(name)
            .subtitle(desc)
            .build();

        let switch = gtk::Switch::new();
        switch.set_valign(gtk::Align::Center);
        
        // Connect the switch to our install state
        let pkgs_clone = pkgs.clone();
        switch.connect_active_notify(move |sw| {
            if sw.is_active() {
                install::add_packages(&pkgs_clone);
            } else {
                install::remove_packages(&pkgs_clone);
            }
        });

        row.add_suffix(&switch);
        listbox.append(&row);
    }

    container.append(&title);
    container.append(&subtitle);
    container.append(&listbox);

    container
}
