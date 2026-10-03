use adw::prelude::*;

use crate::pages;

fn load_css() {
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

    let local = crate::assets::css_path();
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

pub fn activate(app: &adw::Application) {
    adw::StyleManager::default().set_color_scheme(adw::ColorScheme::ForceDark);
    load_css();

    let window = adw::ApplicationWindow::builder()
        .application(app)
        .title("ChurrOS Tour")
        .default_width(900)
        .default_height(680)
        .resizable(false)
        .build();

    window.add_css_class("welcome");

    let stack = adw::ViewStack::new();
    stack.set_vexpand(true);

    let welcome = pages::welcome::build();
    let shortcuts = pages::shortcuts::build();
    let customization = pages::customization::build();
    let install = pages::install::build();

    stack.add_titled(&welcome, Some("welcome"), "Bienvenida");
    stack.add_titled(&shortcuts, Some("shortcuts"), "Atajos");
    stack.add_titled(&customization, Some("customization"), "Personalización");
    stack.add_titled(&install, Some("install"), "Instalación");

    let action_bar = gtk::ActionBar::new();
    
    let back_btn = gtk::Button::with_label("Atrás");
    let next_btn = gtk::Button::with_label("Siguiente");
    next_btn.add_css_class("suggested-action");

    // Autostart checkbox at the end
    let autostart_check = gtk::CheckButton::builder()
        .label("Abrir al iniciar")
        .active(true)
        .visible(false) // Solo se muestra al final
        .build();

    action_bar.pack_start(&back_btn);
    action_bar.pack_end(&next_btn);
    action_bar.set_center_widget(Some(&autostart_check));

    back_btn.set_sensitive(false);

    let back_btn_clone = back_btn.clone();
    let next_btn_clone = next_btn.clone();
    let autostart_check_clone = autostart_check.clone();

    // Logic to update buttons when page changes
    stack.connect_visible_child_notify(move |s| {
        if let Some(child) = s.visible_child_name() {
            let name = child.as_str();
            
            back_btn_clone.set_sensitive(name != "welcome");
            
            if name == "install" {
                next_btn_clone.set_label("Finalizar");
                autostart_check_clone.set_visible(true);
            } else {
                next_btn_clone.set_label("Siguiente");
                autostart_check_clone.set_visible(false);
            }
        }
    });

    let stack_for_next = stack.clone();
    let window_clone = window.clone();
    let autostart_check_clone2 = autostart_check.clone();
    
    next_btn.connect_clicked(move |_| {
        let current = stack_for_next.visible_child_name().map(|s| s.to_string()).unwrap_or_default();
        match current.as_str() {
            "welcome" => stack_for_next.set_visible_child_name("shortcuts"),
            "shortcuts" => stack_for_next.set_visible_child_name("customization"),
            "customization" => {
                // Here we should trigger the installation logic in the install page
                stack_for_next.set_visible_child_name("install");
                pages::install::start_installation(window_clone.clone());
            },
            "install" => {
                // Handle autostart logic
                pages::install::handle_autostart(autostart_check_clone2.is_active());
                window_clone.close();
            },
            _ => {}
        }
    });

    let stack_for_back = stack.clone();
    back_btn.connect_clicked(move |_| {
        let current = stack_for_back.visible_child_name().map(|s| s.to_string()).unwrap_or_default();
        match current.as_str() {
            "shortcuts" => stack_for_back.set_visible_child_name("welcome"),
            "customization" => stack_for_back.set_visible_child_name("shortcuts"),
            "install" => stack_for_back.set_visible_child_name("customization"),
            _ => {}
        }
    });

    let content = gtk::Box::new(gtk::Orientation::Vertical, 0);
    content.append(&stack);
    content.append(&action_bar);

    window.set_content(Some(&content));
    window.present();
}
