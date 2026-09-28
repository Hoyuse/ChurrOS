use adw::prelude::*;
use gtk::prelude::*;
use std::env;

pub fn build() -> gtk::Box {
    let container = gtk::Box::new(gtk::Orientation::Vertical, 16);
    container.set_valign(gtk::Align::Center);
    container.set_halign(gtk::Align::Center);

    let session = env::var("XDG_CURRENT_DESKTOP").unwrap_or_default().to_lowercase();
    let is_niri = session.contains("niri") || session.contains("wayland"); // Simplified fallback

    let title = if is_niri {
        "Atajos de Teclado (Niri)"
    } else {
        "Atajos de Teclado (Xfce)"
    };

    let description = if is_niri {
        "ChurrOS con Niri se maneja principalmente por teclado. Aquí tienes los atajos más importantes:"
    } else {
        "Conoce los atajos más importantes para moverte rápido por ChurrOS:"
    };

    let status_page = adw::StatusPage::builder()
        .title(title)
        .description(description)
        .build();

    let listbox = gtk::ListBox::new();
    listbox.add_css_class("boxed-list");
    listbox.set_margin_start(32);
    listbox.set_margin_end(32);

    let shortcuts = if is_niri {
        vec![
            ("Super + T", "Abrir Terminal"),
            ("Super + Q", "Cerrar ventana actual"),
            ("Super + D", "Abrir lanzador de aplicaciones (Fuzzel)"),
            ("Super + Flechas", "Mover el foco entre ventanas"),
            ("Super + Shift + Flechas", "Mover ventana"),
            ("Super + F", "Pantalla completa"),
        ]
    } else {
        vec![
            ("Super + T", "Abrir Terminal"),
            ("Super + Q", "Cerrar ventana actual"),
            ("Super + Espacio", "Abrir menú de aplicaciones"),
            ("Alt + Tab", "Cambiar de ventana"),
        ]
    };

    for (keys, action) in shortcuts {
        let row = adw::ActionRow::builder()
            .title(action)
            .subtitle(keys)
            .build();
        listbox.append(&row);
    }

    container.append(&status_page);
    container.append(&listbox);

    container
}
