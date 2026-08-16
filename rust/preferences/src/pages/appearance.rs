// ==========================================
// AppearancePage — página principal de apariencia
// (equivalente a pages/appearance.py)
// ==========================================

use std::cell::RefCell;
use std::rc::Rc;

use crate::services::niri_config::NiriConfig;
use crate::services::pywal::PywalService;
use crate::services::settings;
use crate::services::theme::ThemeService;
use crate::services::wallpaper::WallpaperService;
use crate::widgets::group::Group;
use crate::widgets::navigation_row;
use crate::widgets::page::Page;
use crate::widgets::row::Row;
use crate::widgets::switch_row::SwitchRow;

pub fn build(navigator: gtk::Stack) -> Page {
    let page = Page::new(
        Some(navigator.clone()),
        "Apariencia",
        Some("Personaliza el aspecto de ChurrOS"),
        None,
    );

    // feedback_label compartido: los callbacks actualizan el subtítulo en vivo
    let feedback: Rc<RefCell<Option<Row>>> = Rc::new(RefCell::new(None));

    // ============ Tema ============
    let mut theme_group = Group::new("Tema");

    // Modo oscuro
    let dark_active = ThemeService::is_dark();
    let feedback_rc = Rc::clone(&feedback);
    theme_group.add(&SwitchRow::new(
        "Modo oscuro",
        Some("appearance.svg"),
        Some("Usar el tema oscuro"),
        dark_active,
        Some(Box::new(move |active| {
            ThemeService::set(active);
            set_feedback(
                &feedback_rc,
                if active { "Modo oscuro activado" } else { "Modo claro activado" },
            );
        })),
    ));

    // Colores dinámicos (pywal)
    let dynamic_active = settings::get_bool("theme.dynamic_colors", true);
    let feedback_rc = Rc::clone(&feedback);
    theme_group.add(&SwitchRow::new(
        "Colores dinámicos",
        Some("appearance.svg"),
        Some("Generar paleta desde el wallpaper (pywal)"),
        dynamic_active,
        Some(Box::new(move |active| {
            let ok = PywalService::toggle(active);
            set_feedback(
                &feedback_rc,
                if active {
                    if ok {
                        "Colores dinámicos activados"
                    } else {
                        "No se pudo activar (¿pywal instalado y wallpaper válido?)"
                    }
                } else {
                    "Colores dinámicos desactivados"
                },
            );
        })),
    ));

    page.add(theme_group.widget());

    // ============ Fondo de pantalla ============
    let mut wallpaper_group = Group::new("Fondo de pantalla");

    let current = WallpaperService::current();
    let (subtitle, value): (String, Option<String>) =
        if !current.is_empty() && std::path::Path::new(&current).is_file() {
            (format!("Actual: {current}"), None)
        } else {
            (
                "Sin wallpaper configurado".to_string(),
                Some("Sin fondo".to_string()),
            )
        };

    wallpaper_group.add(&Row::new(
        "Wallpaper actual",
        Some(&subtitle),
        Some("wallpaper.svg"),
        value.as_deref(),
        None,
        None,
    ));

    wallpaper_group.add(&navigation_row::new(
        navigator.clone(),
        "Cambiar fondo",
        "wallpaper.svg",
        "wallpaper",
        Some("Elegir entre los fondos disponibles"),
    ));

    page.add(wallpaper_group.widget());

    // ============ Rendimiento ============
    let mut performance_group = Group::new("Rendimiento");

    let performance_on = NiriConfig::get_performance_mode();
    let feedback_rc = Rc::clone(&feedback);
    performance_group.add(&SwitchRow::new(
        "Modo rendimiento",
        Some("appearance.svg"),
        Some("Desactiva blur y animaciones (mejor rendimiento en hardware modesto)"),
        performance_on,
        Some(Box::new(move |active| {
            NiriConfig::set_performance_mode(active);
            NiriConfig::reload();
            set_feedback(
                &feedback_rc,
                if active {
                    "Modo rendimiento activado (blur + animaciones OFF)"
                } else {
                    "Modo rendimiento desactivado (blur + animaciones ON)"
                },
            );
        })),
    ));

    page.add(performance_group.widget());

    // ============ Escritorio ============
    let mut desktop_group = Group::new("Escritorio");

    let animations_on = NiriConfig::get_animations();
    let feedback_rc = Rc::clone(&feedback);
    desktop_group.add(&SwitchRow::new(
        "Animaciones de niri",
        Some("appearance.svg"),
        Some("Desactiva todas las transiciones (mas agil en hardware modesto)"),
        animations_on,
        Some(Box::new(move |active| {
            NiriConfig::set_animations(active);
            NiriConfig::reload();
            set_feedback(
                &feedback_rc,
                if active {
                    "Animaciones activadas"
                } else {
                    "Animaciones desactivadas"
                },
            );
        })),
    ));

    let prefer_no_csd = NiriConfig::get_prefer_no_csd();
    let feedback_rc = Rc::clone(&feedback);
    desktop_group.add(&SwitchRow::new(
        "Sin decoraciones de cliente (CSD)",
        Some("appearance.svg"),
        Some("Las apps omiten sus propios marcos de ventana"),
        prefer_no_csd,
        Some(Box::new(move |active| {
            NiriConfig::set_prefer_no_csd(active);
            NiriConfig::reload();
            set_feedback(
                &feedback_rc,
                if active { "CSD deshabilitado" } else { "CSD permitido" },
            );
        })),
    ));

    page.add(desktop_group.widget());

    // ============ Componentes de UI ============
    let mut components_group = Group::new("Componentes de UI");

    for (title, subtitle, icon, page_name) in [
        (
            "Waybar",
            "Barra superior: posicion, colores y modulos",
            "waybar.svg",
            "waybar",
        ),
        (
            "Foot",
            "Terminal: fuente, cursor, padding, bell",
            "terminal.svg",
            "foot",
        ),
        (
            "Fuzzel",
            "Launcher: fuente, layout, iconos",
            "applications.svg",
            "fuzzel",
        ),
        (
            "Mako",
            "Notificaciones: fuente, colores, posicion, DND",
            "mako.svg",
            "mako",
        ),
    ] {
        components_group.add(&navigation_row::new(
            navigator.clone(),
            title,
            icon,
            page_name,
            Some(subtitle),
        ));
    }

    page.add(components_group.widget());

    // ============ Compositor ============
    let mut compositor_group = Group::new("Compositor");

    compositor_group.add(&navigation_row::new(
        navigator.clone(),
        "Niri",
        "niri.svg",
        "niri",
        Some("Layout, bordes, focus-ring, blur, prefer-no-csd"),
    ));

    page.add(compositor_group.widget());

    // ============ Pantalla ============
    let mut screen_group = Group::new("Pantalla");

    screen_group.add(&navigation_row::new(
        navigator.clone(),
        "Luz nocturna",
        "night_light.svg",
        "night-light",
        Some("Temperatura de color y filtro de luz azul (wlsunset)"),
    ));

    screen_group.add(&navigation_row::new(
        navigator.clone(),
        "Pantalla de bloqueo",
        "lock_screen.svg",
        "lock-screen",
        Some("swaylock + swayidle: estilo y bloqueo automatico"),
    ));

    page.add(screen_group.widget());

    // ============ Personalización básica ============
    let mut personalization_group = Group::new("Personalización básica");

    for (title, subtitle, icon, page_name) in [
        (
            "Colores",
            "Color de acento (manual o desde paleta)",
            "palette.svg",
            "accent",
        ),
        ("Iconos", "Tema de iconos del sistema", "icons.svg", "icons"),
        ("Cursor", "Tema y tamano del cursor", "cursor.svg", "cursor"),
        (
            "Fuentes",
            "Familia y tamano de fuente del sistema",
            "font.svg",
            "fonts",
        ),
    ] {
        personalization_group.add(&navigation_row::new(
            navigator.clone(),
            title,
            icon,
            page_name,
            Some(subtitle),
        ));
    }

    page.add(personalization_group.widget());

    // ============ Reglas de ventana ============
    let mut window_rules_group = Group::new("Reglas de ventana");

    window_rules_group.add(&navigation_row::new(
        navigator.clone(),
        "Reglas de ventana",
        "window_rules.svg",
        "window-rules",
        Some("Opacidad, floatantes, esquinas, blur por app"),
    ));

    page.add(window_rules_group.widget());

    // ============ Estado ============
    let mut status_group = Group::new("Estado");

    let feedback_row = Row::new(
        "Cambios en vivo",
        Some("Los cambios se aplican al instante"),
        Some("appearance.svg"),
        None,
        None,
        None,
    );
    *feedback.borrow_mut() = Some(feedback_row);

    status_group.add(feedback.borrow().as_ref().unwrap());
    page.add(status_group.widget());

    page
}

fn set_feedback(feedback: &Rc<RefCell<Option<Row>>>, text: &str) {
    if let Some(row) = feedback.borrow().as_ref() {
        row.set_subtitle(text);
    }
}
