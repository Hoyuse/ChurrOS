// ==========================================
// popup.rs — ventana base de los popups (port de common/popup.py + header.py)
// ==========================================

use std::path::{Path, PathBuf};

use gtk::gdk::Key;
use gtk::prelude::*;

// En runtime (ISO) los assets viven en /usr/share/churros/churros-popup/assets/
// (desplegados por build-rust.sh a /usr/share/churros/<crate>/assets/).
// En desarrollo se usan los assets locales del crate.
const RUNTIME_ROOT: &str = "/usr/share/churros/churros-popup/assets";
const DEV_ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/assets");

fn assets_root() -> PathBuf {
    let runtime = PathBuf::from(RUNTIME_ROOT);
    if runtime.is_dir() {
        runtime
    } else {
        PathBuf::from(DEV_ROOT)
    }
}

/// Carga el CSS compartido de ChurrOS (si existe), el común de popups y el
/// propio del popup (equivalente a popup.py + load_*_css de cada ventana).
pub fn load_css(own: &str) {
    let Some(display) = gtk::gdk::Display::default() else {
        return;
    };

    let shared = "/usr/share/churros/styles/churros.css";
    let dev_shared = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../archiso/airootfs/usr/share/churros/styles/churros.css"
    );
    let shared_path = if Path::new(shared).is_file() {
        Some(Path::new(shared))
    } else if Path::new(dev_shared).is_file() {
        Some(Path::new(dev_shared))
    } else {
        None
    };
    if let Some(path) = shared_path {
        let provider = gtk::CssProvider::new();
        let _ = provider.load_from_path(path);
        gtk::style_context_add_provider_for_display(
            &display,
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }

    for css in ["common.css", own] {
        let path = assets_root().join(css);
        if path.is_file() {
            let provider = gtk::CssProvider::new();
            let _ = provider.load_from_path(&path);
            gtk::style_context_add_provider_for_display(
                &display,
                &provider,
                gtk::STYLE_PROVIDER_PRIORITY_APPLICATION + 1,
            );
        }
    }

    let home = std::env::var("HOME").unwrap_or_else(|_| "/root".to_string());
    let accent_path = PathBuf::from(home).join(".config/churros/accent.css");
    if let Ok(css) = std::fs::read_to_string(&accent_path) {
        let provider = gtk::CssProvider::new();
        provider.load_from_string(&css);
        gtk::style_context_add_provider_for_display(
            &display,
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_USER,
        );
    }
}

/// Cabecera del popup: icono + título (port de common/widgets/header.py).
pub struct Header {
    pub widget: gtk::Box,
}

impl Header {
    pub fn new(icon: &str, title: &str) -> Self {
        let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        hbox.add_css_class("popup-header");
        hbox.set_margin_bottom(12);

        let icon_label = gtk::Label::new(Some(icon));
        icon_label.add_css_class("popup-header-icon");

        let title_label = gtk::Label::new(Some(title));
        title_label.add_css_class("popup-header-title");
        title_label.set_hexpand(true);
        title_label.set_halign(gtk::Align::Start);

        hbox.append(&icon_label);
        hbox.append(&title_label);

        Self { widget: hbox }
    }
}

/// Ventana base del popup (port de common/popup.py).
pub struct PopupWindow {
    pub window: gtk::ApplicationWindow,
    pub content: gtk::Box,
}

impl PopupWindow {
    pub fn new(app: &gtk::Application, title: &str, icon: &str, css: &str) -> Self {
        load_css(css);

        let window = gtk::ApplicationWindow::builder()
            .application(app)
            .title(title)
            .default_width(320)
            .default_height(400)
            .resizable(false)
            .decorated(false)
            .css_classes(["popup"])
            .build();

        let main_box = gtk::Box::new(gtk::Orientation::Vertical, 0);
        main_box.add_css_class("popup-content");
        window.set_child(Some(&main_box));

        let header = Header::new(icon, title);
        main_box.append(&header.widget);

        let content = gtk::Box::new(gtk::Orientation::Vertical, 0);
        content.set_vexpand(true);
        main_box.append(&content);

        let controller = gtk::EventControllerKey::new();
        controller.connect_key_pressed(glib::clone!(
            #[weak] window,
            #[upgrade_or] glib::Propagation::Proceed,
            move |_, key, _, _| {
                if key == Key::Escape {
                    window.destroy();
                    return glib::Propagation::Stop;
                }
                glib::Propagation::Proceed
            }
        ));
        window.add_controller(controller);

        Self { window, content }
    }

    pub fn add(&self, widget: &impl IsA<gtk::Widget>) {
        self.content.append(widget);
    }

    pub fn present(&self) {
        self.window.present();
    }
}
