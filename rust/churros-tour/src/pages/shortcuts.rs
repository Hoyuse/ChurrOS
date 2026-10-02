use adw::prelude::*;
use gtk::prelude::*;
use std::env;

pub fn build() -> gtk::Box {
    let container = gtk::Box::new(gtk::Orientation::Vertical, 10);
    container.set_valign(gtk::Align::Center);
    container.set_halign(gtk::Align::Center);

    let ed = churros_services::version::edition();

    let (title, description, shortcuts) = if ed == "kde" {
        (
            "Atajos de Teclado (KDE Plasma)",
            "Conoce los atajos más importantes para moverte rápido por ChurrOS KDE:",
            vec![
                ("Super", "Abrir menú de aplicaciones (Kickoff)"),
                ("Super + T", "Abrir Terminal (Konsole)"),
                ("Super + E", "Abrir Archivos (Dolphin)"),
                ("Super + Q / Alt + F4", "Cerrar ventana actual"),
                ("Super + Flechas", "Ajustar / Posicionar ventana"),
                ("Alt + Tab", "Cambiar de ventana"),
                ("Super + L", "Bloquear pantalla"),
            ],
        )
    } else if ed == "xfce" || ed == "server" {
        (
            "Atajos de Teclado (Xfce)",
            "Conoce los atajos más importantes para moverte rápido por ChurrOS:",
            vec![
                ("Super", "Abrir menú de aplicaciones"),
                ("Super + T", "Abrir Terminal"),
                ("Super + Q", "Cerrar ventana actual"),
                ("Alt + Tab", "Cambiar de ventana"),
                ("Super + L", "Bloquear pantalla"),
            ],
        )
    } else {
        (
            "Atajos de Teclado (Niri)",
            "ChurrOS con Niri se maneja principalmente por teclado. Aquí tienes los atajos más importantes:",
            vec![
                ("Super + T", "Abrir Terminal (Foot)"),
                ("Super + Q", "Cerrar ventana actual"),
                ("Super + D", "Abrir lanzador de aplicaciones (Fuzzel)"),
                ("Super + Flechas", "Mover el foco entre ventanas"),
                ("Super + Shift + Flechas", "Mover ventana"),
                ("Super + F", "Pantalla completa"),
            ],
        )
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
