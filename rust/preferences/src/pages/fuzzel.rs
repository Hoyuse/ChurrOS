// ==========================================
// FuzzelPage — launcher fuzzel (fuente, layout, iconos)
// (equivalente a pages/fuzzel.py)
// ==========================================


use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use crate::services::fuzzel_config::FuzzelConfig;
use crate::widgets::combo_row::ComboRow;
use crate::widgets::group::Group;
use crate::widgets::page::Page;
use crate::widgets::row::Row;
use crate::widgets::slider_row::SliderRow;

const FONT_FAMILIES: [&str; 7] = [
    "JetBrainsMono Nerd Font",
    "JetBrains Mono",
    "FiraCode Nerd Font",
    "Inter",
    "Cantarell",
    "Hack",
    "Monospace",
];

struct FuzzelState {
    font_family: ComboRow,
    font_size: ComboRow,
    width: SliderRow,
    lines: SliderRow,
    h_pad: SliderRow,
    v_pad: SliderRow,
    inner_pad: SliderRow,
    line_height: SliderRow,
    letter_spacing: SliderRow,
    icon_theme: ComboRow,
}

type FuzzelStateRef = Rc<RefCell<FuzzelState>>;

struct Scheduler {
    pending: bool,
}

/// Debounce 400ms (equivalente a _schedule_apply + GLib.timeout_add).
fn schedule(holder: &Rc<RefCell<Option<FuzzelStateRef>>>, scheduler: &Rc<RefCell<Scheduler>>) {
    if scheduler.borrow().pending {
        return;
    }
    scheduler.borrow_mut().pending = true;

    let h = Rc::clone(holder);
    let s = Rc::clone(scheduler);
    glib::timeout_add_local(Duration::from_millis(400), move || {
        s.borrow_mut().pending = false;

        if let Some(state) = h.borrow().as_ref() {
            let st = state.borrow();

            let font = format!(
                "{}:size={}",
                st.font_family.value().unwrap_or_default(),
                st.font_size.value().unwrap_or_default()
            );

            FuzzelConfig::set_font(&font);
            FuzzelConfig::set_icon_theme(st.icon_theme.value().unwrap_or_default().as_str());
            FuzzelConfig::set_layout(
                st.width.get_value() as i64,
                st.lines.get_value() as i64,
                st.h_pad.get_value() as i64,
                st.v_pad.get_value() as i64,
                st.inner_pad.get_value() as i64,
                st.line_height.get_value() as i64,
                st.letter_spacing.get_value() as i64,
            );
            FuzzelConfig::reload();
        }

        glib::ControlFlow::Break
    });
}

fn cb_str(
    holder: &Rc<RefCell<Option<FuzzelStateRef>>>,
    scheduler: &Rc<RefCell<Scheduler>>,
) -> Box<dyn Fn(&str)> {
    let h = Rc::clone(holder);
    let s = Rc::clone(scheduler);
    Box::new(move |_| schedule(&h, &s))
}

fn cb_f64(
    holder: &Rc<RefCell<Option<FuzzelStateRef>>>,
    scheduler: &Rc<RefCell<Scheduler>>,
) -> Box<dyn Fn(f64)> {
    let h = Rc::clone(holder);
    let s = Rc::clone(scheduler);
    Box::new(move |_| schedule(&h, &s))
}

pub fn build(navigator: gtk::Stack) -> Page {
    let page = Page::new(
        Some(navigator),
        "Fuzzel",
        Some("Configura el launcher (fuente, layout, colores)"),
        Some("appearance".to_string()),
    );

    let values = serde_json::json!({
        "font": FuzzelConfig::get_font(),
        "icon_theme": FuzzelConfig::get_icon_theme(),
        "width": FuzzelConfig::get_width(),
        "lines": FuzzelConfig::get_lines(),
        "h_pad": FuzzelConfig::get_horizontal_pad(),
        "v_pad": FuzzelConfig::get_vertical_pad(),
        "inner_pad": FuzzelConfig::get_inner_pad(),
        "line_height": FuzzelConfig::get_line_height(),
        "letter_spacing": FuzzelConfig::get_letter_spacing(),
    });

    // Fuente actual: "Familia:size=Tamano" (partition(":size=") del Python)
    let current_font = values["font"].as_str().unwrap_or("");
    let (current_family, current_size) = match current_font.find(":size=") {
        Some(i) => (
            current_font[..i].to_string(),
            current_font[i + 6..]
                .split(':')
                .next()
                .unwrap_or("13")
                .to_string(),
        ),
        None => (
            String::new(),
            if current_font.is_empty() {
                "13".to_string()
            } else {
                current_font.split(':').next().unwrap_or("").to_string()
            },
        ),
    };

    let sizes: Vec<String> = (8..24).map(|s| s.to_string()).collect();
    let size_refs: Vec<&str> = sizes.iter().map(|s| s.as_str()).collect();

    let holder: Rc<RefCell<Option<FuzzelStateRef>>> = Rc::new(RefCell::new(None));
    let scheduler: Rc<RefCell<Scheduler>> = Rc::new(RefCell::new(Scheduler { pending: false }));

    // Tipografia
    let font_family = ComboRow::new(
        "Familia",
        &FONT_FAMILIES,
        Some(&current_family),
        None,
        None,
        Some(cb_str(&holder, &scheduler)),
    );
    let font_size = ComboRow::new(
        "Tamano",
        &size_refs,
        Some(&current_size),
        None,
        None,
        Some(cb_str(&holder, &scheduler)),
    );

    let mut font_group = Group::new("Tipografia");
    font_group.add(&font_family);
    font_group.add(&font_size);
    page.add(font_group.widget());

    // Disposicion
    let width = SliderRow::new(
        "Ancho (caracteres)",
        None,
        None,
        20.0,
        120.0,
        1.0,
        values["width"].as_f64().unwrap_or(48.0),
        Some(cb_f64(&holder, &scheduler)),
    );
    let lines = SliderRow::new(
        "Lineas visibles",
        None,
        None,
        4.0,
        40.0,
        1.0,
        values["lines"].as_f64().unwrap_or(12.0),
        Some(cb_f64(&holder, &scheduler)),
    );
    let h_pad = SliderRow::new(
        "Padding horizontal",
        None,
        None,
        0.0,
        120.0,
        2.0,
        values["h_pad"].as_f64().unwrap_or(36.0),
        Some(cb_f64(&holder, &scheduler)),
    );
    let v_pad = SliderRow::new(
        "Padding vertical",
        None,
        None,
        0.0,
        80.0,
        2.0,
        values["v_pad"].as_f64().unwrap_or(14.0),
        Some(cb_f64(&holder, &scheduler)),
    );
    let inner_pad = SliderRow::new(
        "Padding interno",
        None,
        None,
        0.0,
        40.0,
        1.0,
        values["inner_pad"].as_f64().unwrap_or(4.0),
        Some(cb_f64(&holder, &scheduler)),
    );
    let line_height = SliderRow::new(
        "Altura de linea",
        None,
        None,
        14.0,
        64.0,
        1.0,
        values["line_height"].as_f64().unwrap_or(24.0),
        Some(cb_f64(&holder, &scheduler)),
    );
    let letter_spacing = SliderRow::new(
        "Espaciado entre letras",
        None,
        None,
        0.0,
        8.0,
        1.0,
        values["letter_spacing"].as_f64().unwrap_or(1.0),
        Some(cb_f64(&holder, &scheduler)),
    );

    let mut layout_group = Group::new("Disposicion");
    layout_group.add(&width);
    layout_group.add(&lines);
    layout_group.add(&h_pad);
    layout_group.add(&v_pad);
    layout_group.add(&inner_pad);
    layout_group.add(&line_height);
    layout_group.add(&letter_spacing);
    page.add(layout_group.widget());

    // Iconos
    let icon_themes = [
        "",
        "Papirus-Dark",
        "Papirus",
        "Adwaita",
        "breeze-icons",
        "Gruvbox-Plus-Dark",
    ];
    let icon_theme = ComboRow::new(
        "Tema de iconos",
        &icon_themes,
        Some(values["icon_theme"].as_str().unwrap_or("")),
        Some("Vacio = sin iconos"),
        None,
        Some(cb_str(&holder, &scheduler)),
    );

    let mut icon_group = Group::new("Iconos");
    icon_group.add(&icon_theme);
    page.add(icon_group.widget());

    // Acciones
    let mut actions_group = Group::new("Acciones");

    actions_group.add(&Row::new(
        "Reiniciar Fuzzel",
        Some("Cierra la instancia actual para que aplique cambios"),
        Some("applications.svg"),
        None,
        None,
        Some(Box::new(|_| FuzzelConfig::reload())),
    ));

    page.add(actions_group.widget());

    *holder.borrow_mut() = Some(Rc::new(RefCell::new(FuzzelState {
        font_family,
        font_size,
        width,
        lines,
        h_pad,
        v_pad,
        inner_pad,
        line_height,
        letter_spacing,
        icon_theme,
    })));

    page
}
