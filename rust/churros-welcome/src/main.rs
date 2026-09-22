mod action_card;
mod actions;
mod assets;
mod cards;
mod footer;
mod header;

use gtk::prelude::*;
use adw::prelude::*;

const APP_ID: &str = "org.churros.welcome";

fn load_css() {
    // Cada archivo en su propio provider: load_from_path REEMPLAZA el
    // contenido previo del provider, así que compartir provider perdería
    // el churros.css (el style.css de welcome es autocontenido, pero el
    // CSS compartido aporta tokens/paleta a la ISO).
    let shared = "/usr/share/churros/styles/churros.css";
    let dev_shared = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../archiso/airootfs/usr/share/churros/styles/churros.css"
    );
    let shared_path = if std::path::Path::new(shared).exists() {
        Some(std::path::Path::new(shared))
    } else if std::path::Path::new(dev_shared).exists() {
        Some(std::path::Path::new(dev_shared))
    } else {
        None
    };
    if let Some(path) = shared_path {
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

    // CSS local de la app (pisa al compartido y a stylesheets del sistema)
    let local = assets::css_path();
    let provider = gtk::CssProvider::new();
    if local.is_file() {
        provider.load_from_path(&local);
    } else {
        provider.load_from_string(include_str!("../assets/style.css"));
    }
    if let Some(display) = gtk::gdk::Display::default() {
        gtk::style_context_add_provider_for_display(
            &display,
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION + 1,
        );
    }

    // Colores dinámicos (accent.css de pywal o preferencias)
    let home = std::env::var("HOME").unwrap_or_else(|_| "/root".to_string());
    let accent_path = std::path::PathBuf::from(home).join(".config/churros/accent.css");
    if let Ok(css) = std::fs::read_to_string(&accent_path) {
        let provider = gtk::CssProvider::new();
        provider.load_from_string(&css);
        if let Some(display) = gtk::gdk::Display::default() {
            gtk::style_context_add_provider_for_display(
                &display,
                &provider,
                gtk::STYLE_PROVIDER_PRIORITY_USER,
            );
        }
    }
}

fn activate(app: &adw::Application) {
    load_css();

    let window = adw::ApplicationWindow::builder()
        .application(app)
        .title("ChurrOS Welcome")
        .build();

    window.add_css_class("welcome");
    window.set_default_size(900, 680);
    window.set_size_request(480, 400);
    window.set_resizable(true);
    window.set_decorated(true);

    let header_bar = adw::HeaderBar::new();
    header_bar.set_show_end_title_buttons(true);
    header_bar.set_show_start_title_buttons(true);
    header_bar.add_css_class("flat");

    let content = gtk::Box::new(gtk::Orientation::Vertical, 24);
    content.set_margin_top(20);
    content.set_margin_bottom(30);
    content.set_margin_start(24);
    content.set_margin_end(24);

    content.set_halign(gtk::Align::Center);
    content.set_valign(gtk::Align::Start);
    content.set_hexpand(true);
    content.set_vexpand(true);

    content.append(&header::build());
    content.append(&cards::build());
    content.append(&footer::build());

    let scroller = gtk::ScrolledWindow::new();
    scroller.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
    scroller.set_child(Some(&content));
    scroller.set_hexpand(true);
    scroller.set_vexpand(true);
    scroller.add_css_class("content-scroller");

    let main_box = gtk::Box::new(gtk::Orientation::Vertical, 0);
    main_box.append(&header_bar);
    main_box.append(&scroller);

    window.set_content(Some(&main_box));

    window.present();
}

fn main() -> glib::ExitCode {
    unsafe {
        libc::signal(libc::SIGPIPE, libc::SIG_IGN);
    }

    let app = adw::Application::builder()
        .application_id(APP_ID)
        .build();

    app.connect_activate(activate);

    app.run()
}
