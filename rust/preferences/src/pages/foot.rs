// ==========================================
// FootPage — terminal foot (fuente, padding, cursor, bell)
// (equivalente a pages/foot.py)
// ==========================================


use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use crate::services::foot_config::FootConfig;
use crate::widgets::combo_row::ComboRow;
use crate::widgets::group::Group;
use crate::widgets::page::Page;
use crate::widgets::row::Row;
use crate::widgets::slider_row::SliderRow;
use crate::widgets::switch_row::SwitchRow;

const FONT_FAMILIES: [&str; 7] = [
    "JetBrainsMono Nerd Font",
    "JetBrains Mono",
    "FiraCode Nerd Font",
    "Inter",
    "Cantarell",
    "Hack",
    "Monospace",
];

struct FootState {
    font_family: ComboRow,
    font_size: ComboRow,
    pad_h: SliderRow,
    pad_v: SliderRow,
    cursor_style: ComboRow,
    cursor_blink: SwitchRow,
    bell: SwitchRow,
    hide_when_typing: SwitchRow,
}

type FootStateRef = Rc<RefCell<FootState>>;

/// Debounce 400ms estilo GLib.timeout_add del Python.
struct Scheduler {
    pending: bool,
}

/// Programa el apply si no hay uno pendiente (equivalente a _schedule_apply).
fn schedule(holder: &Rc<RefCell<Option<FootStateRef>>>, scheduler: &Rc<RefCell<Scheduler>>) {
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

            FootConfig::set_font(&font);
            FootConfig::set_pad(&format!(
                "{}x{}",
                st.pad_h.get_value() as i64,
                st.pad_v.get_value() as i64
            ));
            FootConfig::set_cursor(
                st.cursor_style.value().unwrap_or_default().as_str(),
                st.cursor_blink.get_active(),
            );
            FootConfig::set_bell(st.bell.get_active());
            FootConfig::set_hide_when_typing(st.hide_when_typing.get_active());
            FootConfig::reload();
        }

        glib::ControlFlow::Break
    });
}

pub fn build(navigator: gtk::Stack) -> Page {
    let page = Page::new(
        Some(navigator),
        "Foot",
        Some("Configura el terminal (fuente, cursor, padding, bell)"),
        Some("appearance".to_string()),
    );

    let values = serde_json::json!({
        "font": FootConfig::get_font(),
        "pad": FootConfig::get_pad(),
        "cursor_style": FootConfig::get_cursor_style(),
        "cursor_blink": FootConfig::get_cursor_blink(),
        "bell": FootConfig::get_bell(),
        "hide_when_typing": FootConfig::get_hide_when_typing(),
    });

    // Fuente actual: "Familia:size=Tamano" (partition(":size=") del Python)
    let current_font = values["font"].as_str().unwrap_or("");
    let (current_family, current_size) = match current_font.find(":size=") {
        Some(i) => (
            current_font[..i].to_string(),
            current_font[i + 6..]
                .split(':')
                .next()
                .unwrap_or("10")
                .to_string(),
        ),
        None => (
            String::new(),
            if current_font.is_empty() {
                "10".to_string()
            } else {
                current_font.split(':').next().unwrap_or("").to_string()
            },
        ),
    };

    let sizes: Vec<String> = (8..22).map(|s| s.to_string()).collect();
    let size_refs: Vec<&str> = sizes.iter().map(|s| s.as_str()).collect();

    // Padding: "8x8" -> (h, v)
    let pad = values["pad"].as_str().unwrap_or("").to_lowercase();
    let (pad_h_val, pad_v_val) = match pad.find('x') {
        Some(i) => {
            let h = pad[..i].parse::<i64>().unwrap_or(8);
            let v = if i + 1 < pad.len() {
                pad[i + 1..].parse::<i64>().unwrap_or(h)
            } else {
                h
            };
            (h, v)
        }
        None => {
            let h = pad.parse::<i64>().unwrap_or(8);
            (h, h)
        }
    };

    // Estado + scheduler compartidos (holder se enlaza al final del build;
    // los callbacks solo programan, no tocan el estado todavia).
    let holder: Rc<RefCell<Option<FootStateRef>>> = Rc::new(RefCell::new(None));
    let scheduler: Rc<RefCell<Scheduler>> = Rc::new(RefCell::new(Scheduler { pending: false }));

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

    // Tipografia
    let mut font_group = Group::new("Tipografia");
    font_group.add(&font_family);
    font_group.add(&font_size);
    page.add(font_group.widget());

    // Padding
    let pad_h = SliderRow::new(
        "Padding horizontal",
        None,
        None,
        0.0,
        64.0,
        1.0,
        pad_h_val as f64,
        Some(cb_f64(&holder, &scheduler)),
    );
    let pad_v = SliderRow::new(
        "Padding vertical",
        None,
        None,
        0.0,
        64.0,
        1.0,
        pad_v_val as f64,
        Some(cb_f64(&holder, &scheduler)),
    );

    let mut pad_group = Group::new("Padding");
    pad_group.add(&pad_h);
    pad_group.add(&pad_v);
    page.add(pad_group.widget());

    // Cursor
    let cursor_style = ComboRow::new(
        "Estilo",
        &["block", "underline", "beam"],
        Some(values["cursor_style"].as_str().unwrap_or("beam")),
        None,
        None,
        Some(cb_str(&holder, &scheduler)),
    );
    let cursor_blink = SwitchRow::new(
        "Parpadeo",
        None,
        None,
        values["cursor_blink"].as_bool().unwrap_or(true),
        Some(cb_bool(&holder, &scheduler)),
    );

    let mut cursor_group = Group::new("Cursor");
    cursor_group.add(&cursor_style);
    cursor_group.add(&cursor_blink);
    page.add(cursor_group.widget());

    // Comportamiento
    let bell = SwitchRow::new(
        "Campana urgente",
        None,
        Some("Notifica visualmente cuando llega un beep"),
        values["bell"].as_bool().unwrap_or(true),
        Some(cb_bool(&holder, &scheduler)),
    );
    let hide_when_typing = SwitchRow::new(
        "Ocultar raton al teclear",
        None,
        None,
        values["hide_when_typing"].as_bool().unwrap_or(true),
        Some(cb_bool(&holder, &scheduler)),
    );

    let mut behavior_group = Group::new("Comportamiento");
    behavior_group.add(&bell);
    behavior_group.add(&hide_when_typing);
    page.add(behavior_group.widget());

    // Acciones
    let mut actions_group = Group::new("Acciones");

    actions_group.add(&Row::new(
        "Recargar Foot",
        Some("Aplica los cambios a las terminales abiertas"),
        Some("terminal.svg"),
        None,
        None,
        Some(Box::new(|_| FootConfig::reload())),
    ));

    page.add(actions_group.widget());

    // Enlazar estado (los callbacks ya estan dentro de cada widget)
    *holder.borrow_mut() = Some(Rc::new(RefCell::new(FootState {
        font_family,
        font_size,
        pad_h,
        pad_v,
        cursor_style,
        cursor_blink,
        bell,
        hide_when_typing,
    })));

    page
}

/// Callback de ComboRow (equivalente a lambda *_: self._schedule_apply()).
fn cb_str(
    holder: &Rc<RefCell<Option<FootStateRef>>>,
    scheduler: &Rc<RefCell<Scheduler>>,
) -> Box<dyn Fn(&str)> {
    let h = Rc::clone(holder);
    let s = Rc::clone(scheduler);
    Box::new(move |_| schedule(&h, &s))
}

/// Callback de SliderRow.
fn cb_f64(
    holder: &Rc<RefCell<Option<FootStateRef>>>,
    scheduler: &Rc<RefCell<Scheduler>>,
) -> Box<dyn Fn(f64)> {
    let h = Rc::clone(holder);
    let s = Rc::clone(scheduler);
    Box::new(move |_| schedule(&h, &s))
}

/// Callback de SwitchRow.
fn cb_bool(
    holder: &Rc<RefCell<Option<FootStateRef>>>,
    scheduler: &Rc<RefCell<Scheduler>>,
) -> Box<dyn Fn(bool)> {
    let h = Rc::clone(holder);
    let s = Rc::clone(scheduler);
    Box::new(move |_| schedule(&h, &s))
}
