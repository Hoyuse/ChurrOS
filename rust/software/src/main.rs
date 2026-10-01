// ==========================================
// churros-software — entry point
//
// Tienda de paquetes de ChurOS: pacman, Flatpak y AUR en una sola ventana.
// ==========================================

#![allow(dead_code)]

mod actions;
mod model;
mod sources;
mod ui;

use gtk::prelude::*;

const APP_ID: &str = "org.churros.software";

fn load_css() {
    // Cada archivo necesita su propio CssProvider: gtk_css_provider_load_from_path
    // reemplaza el contenido anterior del provider.
    let shared = "/usr/share/churros/styles/churros.css";
    let dev_shared = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../archiso/airootfs/usr/share/churros/styles/churros.css"
    );
    let path = if std::path::Path::new(shared).exists() {
        Some(shared)
    } else if std::path::Path::new(dev_shared).exists() {
        Some(dev_shared)
    } else {
        None
    };

    let Some(path) = path else { return };

    let provider = gtk::CssProvider::new();
    provider.load_from_path(path);
    if let Some(display) = gtk::gdk::Display::default() {
        gtk::style_context_add_provider_for_display(
            &display,
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }
}

fn main() -> glib::ExitCode {
    adw::init().expect("no se pudo inicializar libadwaita");

    let app = gtk::Application::builder().application_id(APP_ID).build();
    app.connect_activate(|app| {
        load_css();
        ui::build(app);
    });

    // Sin argumentos: se pasarían a GApplication como ficheros a abrir.
    app.run_with_args(&[] as &[&str])
}
