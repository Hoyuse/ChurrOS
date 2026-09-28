use adw::prelude::*;
use gtk::prelude::*;
use std::env;

pub fn build() -> gtk::Box {
    let container = gtk::Box::new(gtk::Orientation::Vertical, 10);
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

    let title_label = gtk::Label::new(Some(title));
    title_label.add_css_class("page-title");
    title_label.set_halign(gtk::Align::Center);

    let subtitle = gtk::Label::new(Some(description));
    subtitle.add_css_class("page-subtitle");
    subtitle.set_halign(gtk::Align::Center);
    subtitle.set_wrap(true);
    subtitle.set_justify(gtk::Justification::Center);

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

    container.append(&title_label);
    container.append(&subtitle);
    container.append(&listbox);

    container
}
