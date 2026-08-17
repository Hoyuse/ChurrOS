// ==========================================
// WaybarPage — barra superior (posicion, tipografia, colores, modulos)
// (equivalente a pages/waybar.py)
// ==========================================

use gtk::prelude::*;

use serde_json::{json, Value};

use std::cell::RefCell;
use std::rc::Rc;

use crate::services::waybar::WaybarService;
use crate::widgets::color_picker::ColorPickerRow;
use crate::widgets::combo_row::ComboRow;
use crate::widgets::group::Group;
use crate::widgets::page::Page;
use crate::widgets::row::Row;
use crate::widgets::slider_row::SliderRow;

// Definido en el Python pero nunca usado por la página (paridad).
#[allow(dead_code)]
const AVAILABLE_MODULES: [&str; 21] = [
    "niri/workspaces",
    "clock",
    "cpu",
    "memory",
    "disk",
    "battery",
    "backlight",
    "network",
    "bluetooth",
    "pulseaudio",
    "tray",
    "idle_inhibitor",
    "mpris",
    "custom/launcher",
    "custom/control-center",
    "custom/settings",
    "custom/power",
    "custom/dnd",
    "custom/screenrecording-indicator",
    "custom/screenrecording-toggle",
    "custom/sep",
];

const MODULE_POSITIONS: [&str; 3] = ["left", "center", "right"];

struct WaybarState {
    values: Value,
    layer: ComboRow,
    position: ComboRow,
    height: SliderRow,
    spacing: SliderRow,
    font_size: SliderRow,
    font_family: ComboRow,
    bg: ColorPickerRow,
    fg: ColorPickerRow,
    accent: ColorPickerRow,
    bg_alpha: SliderRow,
    module_states: [Vec<String>; 3], // left, center, right
    modules_group: Rc<RefCell<Group>>,
    /// Contenido de la página (para reconstruirla tras reset)
    content: gtk::Box,
}

type WaybarStateRef = Rc<RefCell<WaybarState>>;

pub fn build(navigator: gtk::Stack) -> Page {
    let page = Page::new(
        Some(navigator.clone()),
        "Waybar",
        Some("Personaliza la barra superior"),
        Some("appearance".to_string()),
    );

    let values = WaybarService::get();

    let state: WaybarStateRef = Rc::new(RefCell::new(WaybarState {
        values: values.clone(),
        layer: ComboRow::new("", &["top"], None, None, None, None),
        position: ComboRow::new("", &["top"], None, None, None, None),
        height: SliderRow::new("", None, None, 20.0, 80.0, 1.0, 30.0, None),
        spacing: SliderRow::new("", None, None, 0.0, 16.0, 1.0, 0.0, None),
        font_size: SliderRow::new("", None, None, 10.0, 24.0, 1.0, 14.0, None),
        font_family: ComboRow::new("", &["JetBrainsMono Nerd Font"], None, None, None, None),
        bg: ColorPickerRow::new("", "#2a1612", None, None),
        fg: ColorPickerRow::new("", "#c9c4c3", None, None),
        accent: ColorPickerRow::new("", "#DE8636", None, None),
        bg_alpha: SliderRow::new("", None, None, 0.0, 1.0, 0.05, 0.9, None),
        module_states: [
            modules_from(&values, "modules-left"),
            modules_from(&values, "modules-center"),
            modules_from(&values, "modules-right"),
        ],
        modules_group: Rc::new(RefCell::new(Group::new("Modulos (clic para mover)"))),
        content: page.content.clone(),
    }));

    populate(&page.content, &state);

    page
}

fn modules_from(values: &Value, key: &str) -> Vec<String> {
    values
        .get(key)
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect()
        })
        .unwrap_or_default()
}

/// Construye todos los grupos de la página (equivalente a _build).
fn populate(content: &gtk::Box, state: &WaybarStateRef) {
    let values = state.borrow().values.clone();

    // ---------- Posicion y tamano ----------
    let mut layout_group = Group::new("Posicion y tamano");

    {
        let mut st = state.borrow_mut();
        st.layer = ComboRow::new(
            "Capa",
            &["top", "overlay", "bottom"],
            values["layer"].as_str(),
            None,
            None,
            // NOTA: el Python llama a self._on_change(...), metodo que NO
            // existe en WaybarPage (bug del Python: AttributeError en vivo).
            Some(Box::new(|_| {})),
        );
        st.position = ComboRow::new(
            "Posicion",
            &["top", "bottom", "left", "right"],
            values["position"].as_str(),
            None,
            None,
            Some(Box::new(|_| {})),
        );
        st.height = SliderRow::new(
            "Altura",
            None,
            None,
            20.0,
            80.0,
            1.0,
            values["height"].as_f64().unwrap_or(30.0),
            Some(Box::new(|_| {})),
        );
        st.spacing = SliderRow::new(
            "Espaciado",
            None,
            Some("Espacio entre modulos"),
            0.0,
            16.0,
            1.0,
            values["spacing"].as_f64().unwrap_or(0.0),
            Some(Box::new(|_| {})),
        );
    }
    {
        let st = state.borrow();
        layout_group.add(&st.layer);
        layout_group.add(&st.position);
        layout_group.add(&st.height);
        layout_group.add(&st.spacing);
    }
    content.append(layout_group.widget());

    // ---------- Tipografia ----------
    let mut typography_group = Group::new("Tipografia");

    {
        let mut st = state.borrow_mut();
        st.font_size = SliderRow::new(
            "Tamano de fuente",
            None,
            None,
            10.0,
            24.0,
            1.0,
            values["font-size"].as_f64().unwrap_or(14.0),
            Some(Box::new(|_| {})),
        );
        let font_families = [
            "JetBrainsMono Nerd Font",
            "JetBrains Mono",
            "Inter",
            "Cantarell",
            "Noto Sans",
            "DejaVu Sans",
            "Monospace",
        ];
        let current_family = values["font-family"].as_str().unwrap_or(font_families[0]);
        st.font_family = ComboRow::new(
            "Familia tipografica",
            &font_families,
            Some(current_family),
            None,
            None,
            Some(Box::new(|_| {})),
        );
    }
    {
        let st = state.borrow();
        typography_group.add(&st.font_size);
        typography_group.add(&st.font_family);
    }
    content.append(typography_group.widget());

    // ---------- Colores ----------
    let mut colors_group = Group::new("Colores");

    {
        let mut st = state.borrow_mut();
        st.bg = ColorPickerRow::new(
            "Fondo",
            values["background"].as_str().unwrap_or("#2a1612"),
            Some(Box::new(|_| {})),
            None,
        );
        st.fg = ColorPickerRow::new(
            "Texto",
            values["foreground"].as_str().unwrap_or("#c9c4c3"),
            Some(Box::new(|_| {})),
            None,
        );
        st.accent = ColorPickerRow::new(
            "Acento",
            values["accent"].as_str().unwrap_or("#DE8636"),
            Some(Box::new(|_| {})),
            None,
        );
        st.bg_alpha = SliderRow::new(
            "Opacidad del fondo",
            None,
            Some("0 = transparente, 1 = solido"),
            0.0,
            1.0,
            0.05,
            values["background-alpha"].as_f64().unwrap_or(0.9),
            Some(Box::new(|_| {})),
        );
    }
    {
        let st = state.borrow();
        colors_group.add(&st.bg);
        colors_group.add(&st.fg);
        colors_group.add(&st.accent);
        colors_group.add(&st.bg_alpha);
    }
    content.append(colors_group.widget());

    // ---------- Modulos ----------
    rebuild_modules(state);

    {
        let st = state.borrow();
        content.append(st.modules_group.borrow().widget());
    }

    // ---------- Acciones ----------
    let mut actions_group = Group::new("Acciones");

    let st = Rc::clone(state);
    actions_group.add(&Row::new(
        "Guardar y aplicar",
        Some("Escribe la configuracion y recarga waybar"),
        Some("waybar.svg"),
        None,
        None,
        Some(Box::new(move |_| save_and_reload(&st))),
    ));

    let st = Rc::clone(state);
    actions_group.add(&Row::new(
        "Restablecer defaults",
        Some("Vuelve a la configuracion original de ChurrOS"),
        Some("waybar.svg"),
        None,
        None,
        Some(Box::new(move |btn| reset_defaults(btn, &st))),
    ));

    content.append(actions_group.widget());
}

/// Mueve UNA instancia a la siguiente posicion (left -> center -> right -> left).
/// Por nombre se perdían todos los `custom/sep` de golpe.
fn cycle_module(state: &WaybarStateRef, pos: usize, idx: usize) {
    {
        let mut st = state.borrow_mut();
        if pos >= st.module_states.len() || idx >= st.module_states[pos].len() {
            return;
        }
        let module = st.module_states[pos].remove(idx);
        let nxt = (pos + 1) % MODULE_POSITIONS.len();
        st.module_states[nxt].push(module);
    }
    rebuild_modules(state);
}

/// Quita UNA instancia de su posicion.
fn remove_module(state: &WaybarStateRef, pos: usize, idx: usize) {
    {
        let mut st = state.borrow_mut();
        if pos >= st.module_states.len() || idx >= st.module_states[pos].len() {
            return;
        }
        st.module_states[pos].remove(idx);
    }
    rebuild_modules(state);
}

/// Reconstruye las filas de modulos (equivalente a _rebuild_modules).
fn rebuild_modules(state: &WaybarStateRef) {
    let st = state.borrow();

    let mut group = st.modules_group.borrow_mut();
    group.clear();

    for (i, position) in MODULE_POSITIONS.iter().enumerate() {
        let modules = st.module_states[i].clone();
        for (j, module) in modules.iter().enumerate() {
            let subtitle = format!("{position} — clic para mover, clic der. para quitar");

            let st_cycle = Rc::clone(state);
            let row = Row::new(
                module,
                Some(&subtitle),
                Some("waybar.svg"),
                None,
                None,
                Some(Box::new(move |_| cycle_module(&st_cycle, i, j))),
            );

            let gesture = gtk::GestureClick::new();
            gesture.set_button(3);
            let st_remove = Rc::clone(state);
            gesture.connect_pressed(move |_g, _n, _x, _y| {
                remove_module(&st_remove, i, j);
            });
            row.widget().add_controller(gesture);

            group.add(&row);
        }
    }
}

/// Guarda los valores y recarga waybar (equivalente a _save_and_reload).
fn save_and_reload(state: &WaybarStateRef) {
    let st = state.borrow();

    let values = json!({
        "layer": st.layer.value().filter(|s| !s.is_empty()).unwrap_or_else(|| "top".into()),
        "position": st.position.value().filter(|s| !s.is_empty()).unwrap_or_else(|| "top".into()),
        "spacing": st.spacing.get_value() as i64,
        "height": st.height.get_value() as i64,
        "font-size": st.font_size.get_value() as i64,
        "font-family": st.font_family.value().filter(|s| !s.is_empty()).unwrap_or_else(|| "JetBrainsMono Nerd Font".into()),
        "background": st.bg.get_value(),
        "foreground": st.fg.get_value(),
        "accent": st.accent.get_value(),
        "background-alpha": st.bg_alpha.get_value(),
        "modules-left": st.module_states[0].clone(),
        "modules-center": st.module_states[1].clone(),
        "modules-right": st.module_states[2].clone(),
    });
    drop(st);

    WaybarService::set(&values);
    WaybarService::reload(true);
}

/// Dialogo de confirmacion y restablecimiento (equivalente a _reset_defaults).
fn reset_defaults(btn: &gtk::Button, state: &WaybarStateRef) {
    let dialog = gtk::AlertDialog::builder()
        .message(
            "¿Seguro que quieres restaurar la configuracion por defecto de ChurrOS? Perderás todos los cambios realizados.",
        )
        .modal(true)
        .buttons(["Cancelar", "Restablecer"])
        .build();

    let window = btn.root().and_downcast::<gtk::Window>();
    let st = Rc::clone(state);

    dialog.choose(window.as_ref(), None::<&gio::Cancellable>, move |result| {
        let Ok(response) = result else {
            return;
        };
        if response != 1 {
            return;
        }

        WaybarService::reset();

        let new_values = WaybarService::get();
        {
            let mut s = st.borrow_mut();
            s.values = new_values.clone();
            s.module_states = [
                modules_from(&new_values, "modules-left"),
                modules_from(&new_values, "modules-center"),
                modules_from(&new_values, "modules-right"),
            ];
        }

        // Reconstruir la pagina (el Python vacia self.content y re-ejecuta _build)
        let content = st.borrow().content.clone();
        while let Some(child) = content.first_child() {
            content.remove(&child);
        }
        populate(&content, &st);
    });
}
