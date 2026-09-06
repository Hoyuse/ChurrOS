// ==========================================
// churros-settings (preferences) — entry point
// (equivalente a main.py)
// ==========================================

mod assets;
mod logging;
mod pages;
mod services;
mod widgets;
mod window;

use adw::prelude::*;
use gtk::prelude::*;
use gtk::gdk;

use services::accent::AccentService;
use services::theme::ThemeService;
use window::PreferencesWindow;

const APP_ID: &str = "org.churros.preferences";

fn load_css() {
    // IMPORTANTE: cada archivo necesita su PROPIO CssProvider —
    // gtk_css_provider_load_from_path REEMPLAZA el contenido previo
    // del provider (cargar 3 CSS en 1 provider deja solo el último).
    // Prioridades idénticas a main.py:
    //   churros.css -> APPLICATION, style.css -> APPLICATION+1,
    //   accent.css  -> USER (la más alta, pisa a las demás).

    // CSS compartido de ChurrOS
    let shared = "/usr/share/churros/styles/churros.css";
    if std::path::Path::new(shared).exists() {
        let provider = gtk::CssProvider::new();
        provider.load_from_path(shared);
        if let Some(display) = gdk::Display::default() {
            gtk::style_context_add_provider_for_display(
                &display,
                &provider,
                gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );
        }
    }

    // CSS local de preferences (pisa a churros.css)
    let local = assets::css_path();
    let provider = gtk::CssProvider::new();
    if local.is_file() {
        provider.load_from_path(&local);
    } else {
        provider.load_from_data(include_str!("../assets/style.css"));
    }
    if let Some(display) = gdk::Display::default() {
        gtk::style_context_add_provider_for_display(
            &display,
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION + 1,
        );
    }

    // accent.css gestionado como singleton en AccentService (prioridad USER)
    AccentService::init_css_provider();
}

fn activate(app: &adw::Application) {
    logging::log("activate");
    // Regenerar accent.css si falta (como AccentService.ensure() en Python)
    AccentService::ensure();
    logging::log("accent ok");

    load_css();
    logging::log("css ok");

    let win = PreferencesWindow::new(app);
    logging::log("window creada");
    win.present();
    logging::log("presentada");
    // PreferencesWindow guarda gio::Settings (color-scheme). Si se dropea
    // al salir de activate, el handler muere aunque la ventana GTK siga.
    std::mem::forget(win);
}

fn main() -> glib::ExitCode {
    logging::init("settings");
    ThemeService::migrate_before_gtk();

    let app = adw::Application::builder()
        .application_id(APP_ID)
        .build();
    logging::log("adw app creada");

    app.connect_activate(activate);

    let code = app.run();
    logging::log(&format!("salida code={code:?}"));
    code
}
