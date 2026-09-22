// ==========================================
// NiriPage — compositor niri (disposicion, bordes, blur, ventanas)
// (equivalente a pages/niri.py)
// ==========================================


use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use serde_json::Value;

use crate::services::niri_config::NiriConfig;
use crate::widgets::color_picker::ColorPickerRow;
use crate::widgets::group::Group;
use crate::widgets::page::Page;
use crate::widgets::row::Row;
use crate::widgets::slider_row::SliderRow;
use crate::widgets::switch_row::SwitchRow;

struct NiriState {
    values: Value,
    gaps: SliderRow,
    border_switch: SwitchRow,
    border_width: SliderRow,
    border_active: ColorPickerRow,
    border_inactive: ColorPickerRow,
    focus_ring: SwitchRow,
    blur_passes: SliderRow,
    blur_offset: SliderRow,
    blur_noise: SliderRow,
    blur_saturation: SliderRow,
    animations: SwitchRow,
    window_open: SliderRow,
    workspace_switch: SliderRow,
    csd: SwitchRow,
}

type NiriStateRef = Rc<RefCell<NiriState>>;

struct Scheduler {
    pending: bool,
}

/// Debounce 400ms (equivalente a _schedule_apply + GLib.timeout_add).
fn schedule(state: &NiriStateRef, scheduler: &Rc<RefCell<Scheduler>>) {
    if scheduler.borrow().pending {
        return;
    }
    scheduler.borrow_mut().pending = true;

    let st = Rc::clone(state);
    let s = Rc::clone(scheduler);
    glib::timeout_add_local(Duration::from_millis(400), move || {
        s.borrow_mut().pending = false;

        let state = st.borrow();
        NiriConfig::set_gaps(state.gaps.get_value() as i64);

        let border_on = state.values["border"]["on"].as_bool().unwrap_or(true);
        NiriConfig::set_border(
            border_on,
            state.border_width.get_value() as i64,
            &state.border_active.get_value(),
            &state.border_inactive.get_value(),
        );

        NiriConfig::set_blur(
            state.blur_passes.get_value() as i64,
            state.blur_offset.get_value(),
            state.blur_noise.get_value(),
            state.blur_saturation.get_value(),
        );

        // NOTA: el Python original no persiste las duraciones de animacion
        // (sliders window-open / workspace-switch) — paridad replicada.
        NiriConfig::reload();

        glib::ControlFlow::Break
    });
}

fn cb_f64(state: &NiriStateRef, scheduler: &Rc<RefCell<Scheduler>>) -> Box<dyn Fn(f64)> {
    let st = Rc::clone(state);
    let s = Rc::clone(scheduler);
    Box::new(move |_| schedule(&st, &s))
}

fn cb_bool(state: &NiriStateRef, scheduler: &Rc<RefCell<Scheduler>>) -> Box<dyn Fn(bool)> {
    let st = Rc::clone(state);
    let s = Rc::clone(scheduler);
    Box::new(move |_| schedule(&st, &s))
}

pub fn build(navigator: gtk::Stack) -> Page {
    let page = Page::new(
        Some(navigator),
        "Niri",
        Some("Configura el compositor (disposicion, bordes, blur, ventanas)"),
        Some("appearance".to_string()),
    );

    // Valores iniciales (try/except del Python -> fallbacks)
    let values = serde_json::json!({
        "gaps": NiriConfig::get_gaps(),
        "prefer_no_csd": NiriConfig::get_prefer_no_csd(),
        "border": NiriConfig::get_border(),
        "focus_ring": NiriConfig::get_focus_ring(),
        "blur": NiriConfig::get_blur(),
        "animations": NiriConfig::get_animations(),
    });

    // El estado se crea antes que los widgets y se rellena al final,
    // porque los callbacks necesitan el estado (patrón Rc<RefCell<>>).
    let state: NiriStateRef = Rc::new(RefCell::new(NiriState {
        values: values.clone(),
        gaps: SliderRow::new("", None, None, 0.0, 32.0, 1.0, 8.0, None),
        border_switch: SwitchRow::new("", None, None, false, None),
        border_width: SliderRow::new("", None, None, 1.0, 8.0, 1.0, 2.0, None),
        border_active: ColorPickerRow::new("", "#DE8636", None, None),
        border_inactive: ColorPickerRow::new("", "#766561", None, None),
        focus_ring: SwitchRow::new("", None, None, false, None),
        blur_passes: SliderRow::new("", None, None, 0.0, 6.0, 1.0, 2.0, None),
        blur_offset: SliderRow::new("", None, None, 0.0, 16.0, 1.0, 2.0, None),
        blur_noise: SliderRow::new("", None, None, 0.0, 1.0, 0.05, 0.0, None),
        blur_saturation: SliderRow::new("", None, None, 0.0, 2.0, 0.05, 1.2, None),
        animations: SwitchRow::new("", None, None, false, None),
        window_open: SliderRow::new("", None, None, 0.0, 1000.0, 50.0, 250.0, None),
        workspace_switch: SliderRow::new("", None, None, 0.0, 1000.0, 50.0, 250.0, None),
        csd: SwitchRow::new("", None, None, false, None),
    }));

    let scheduler: Rc<RefCell<Scheduler>> = Rc::new(RefCell::new(Scheduler { pending: false }));

    let gaps = values["gaps"].as_i64().unwrap_or(8) as f64;
    let border = &values["border"];
    let blur = &values["blur"];

    // Disposicion
    let mut layout_group = Group::new("Disposicion");

    {
        let mut st = state.borrow_mut();
        st.gaps = SliderRow::new(
            "Espacio entre ventanas",
            None,
            Some("Gaps (px)"),
            0.0,
            32.0,
            1.0,
            gaps,
            Some(cb_f64(&state, &scheduler)),
        );
    }
    {
        let st = state.borrow();
        layout_group.add(&st.gaps);
    }

    page.add(layout_group.widget());

    // Bordes
    let mut border_group = Group::new("Bordes de ventana");

    {
        let mut st = state.borrow_mut();
        let border_on = border["on"].as_bool().unwrap_or(true);
        st.border_switch = SwitchRow::new(
            "Mostrar bordes",
            None,
            Some("Activa el borde coloreado alrededor de la ventana enfocada"),
            border_on,
            Some(Box::new(on_border_toggle(&state))),
        );
        st.border_width = SliderRow::new(
            "Grosor del borde",
            None,
            Some("Ancho en pixeles"),
            1.0,
            8.0,
            1.0,
            border["width"].as_f64().unwrap_or(2.0),
            Some(cb_f64(&state, &scheduler)),
        );
        st.border_active = ColorPickerRow::new(
            "Color del borde activo",
            border["active_color"].as_str().unwrap_or("#DE8636"),
            Some(cb_str(&state, &scheduler)),
            None,
        );
        st.border_inactive = ColorPickerRow::new(
            "Color del borde inactivo",
            border["inactive_color"].as_str().unwrap_or("#766561"),
            Some(cb_str(&state, &scheduler)),
            None,
        );
    }
    {
        let st = state.borrow();
        border_group.add(&st.border_switch);
        border_group.add(&st.border_width);
        border_group.add(&st.border_active);
        border_group.add(&st.border_inactive);
    }

    page.add(border_group.widget());

    // Focus ring
    let mut fr_group = Group::new("Anillo de foco");

    {
        let mut st = state.borrow_mut();
        st.focus_ring = SwitchRow::new(
            "Anillo de foco",
            None,
            Some("Muestra un anillo alrededor de la ventana enfocada"),
            values["focus_ring"].as_bool().unwrap_or(false),
            Some(Box::new(on_focus_ring_toggle(&state))),
        );
    }
    {
        let st = state.borrow();
        fr_group.add(&st.focus_ring);
    }

    page.add(fr_group.widget());

    // Blur
    let mut blur_group = Group::new("Desenfoque (blur)");

    {
        let mut st = state.borrow_mut();
        st.blur_passes = SliderRow::new(
            "Pasadas",
            None,
            Some("Mas pasadas = mas desenfoque (tambien mas coste GPU)"),
            0.0,
            6.0,
            1.0,
            blur["passes"].as_f64().unwrap_or(2.0),
            Some(cb_f64(&state, &scheduler)),
        );
        st.blur_offset = SliderRow::new(
            "Desplazamiento",
            None,
            None,
            0.0,
            16.0,
            1.0,
            blur["offset"].as_f64().unwrap_or(2.0),
            Some(cb_f64(&state, &scheduler)),
        );
        st.blur_noise = SliderRow::new(
            "Ruido",
            None,
            Some("0 = sin ruido, 1 = maximo"),
            0.0,
            1.0,
            0.05,
            blur["noise"].as_f64().unwrap_or(0.0),
            Some(cb_f64(&state, &scheduler)),
        );
        st.blur_saturation = SliderRow::new(
            "Saturacion",
            None,
            Some("1.0 = sin cambio"),
            0.0,
            2.0,
            0.05,
            blur["saturation"].as_f64().unwrap_or(1.2),
            Some(cb_f64(&state, &scheduler)),
        );
    }
    {
        let st = state.borrow();
        blur_group.add(&st.blur_passes);
        blur_group.add(&st.blur_offset);
        blur_group.add(&st.blur_noise);
        blur_group.add(&st.blur_saturation);
    }

    page.add(blur_group.widget());

    // Animaciones
    let mut animations_group = Group::new("Animaciones");

    {
        let mut st = state.borrow_mut();
        st.animations = SwitchRow::new(
            "Animaciones",
            None,
            Some("Desactiva todas las transiciones de niri (mas agil en hardware modesto)"),
            values["animations"].as_bool().unwrap_or(true),
            Some(Box::new(on_animations_toggle(&state))),
        );
        st.window_open = SliderRow::new(
            "Apertura de ventanas",
            None,
            Some("Duracion en ms (window-open)"),
            0.0,
            1000.0,
            50.0,
            NiriConfig::get_animation_duration("window-open", 250) as f64,
            Some(cb_f64(&state, &scheduler)),
        );
        st.workspace_switch = SliderRow::new(
            "Cambio de workspace",
            None,
            Some("Duracion en ms (workspace-switch)"),
            0.0,
            1000.0,
            50.0,
            NiriConfig::get_animation_duration("workspace-switch", 250) as f64,
            Some(cb_f64(&state, &scheduler)),
        );
    }
    {
        let st = state.borrow();
        animations_group.add(&st.animations);
        animations_group.add(&st.window_open);
        animations_group.add(&st.workspace_switch);
    }

    page.add(animations_group.widget());

    // Ventanas
    let mut windows_group = Group::new("Ventanas");

    {
        let mut st = state.borrow_mut();
        st.csd = SwitchRow::new(
            "Sin decoraciones del lado del cliente (CSD)",
            None,
            Some("Pide a las apps que omitan sus propias decoraciones de ventana"),
            values["prefer_no_csd"].as_bool().unwrap_or(true),
            Some(Box::new(on_csd_toggle(&state))),
        );
    }
    {
        let st = state.borrow();
        windows_group.add(&st.csd);
    }

    page.add(windows_group.widget());

    // Acciones
    let mut actions_group = Group::new("Acciones");

    actions_group.add(&Row::new(
        "Recargar Niri",
        Some("Aplica los cambios forzando una transicion de pantalla"),
        Some("niri.svg"),
        None,
        None,
        Some(Box::new(|_| NiriConfig::reload())),
    ));

    page.add(actions_group.widget());

    page
}

fn cb_str(state: &NiriStateRef, scheduler: &Rc<RefCell<Scheduler>>) -> Box<dyn Fn(&str)> {
    let st = Rc::clone(state);
    let s = Rc::clone(scheduler);
    Box::new(move |_| schedule(&st, &s))
}

/// Equivalente a _on_border_toggle: aplica el estado actual del borde y recarga.
fn on_border_toggle(state: &NiriStateRef) -> Box<dyn Fn(bool)> {
    let st = Rc::clone(state);
    Box::new(move |value: bool| {
        st.borrow_mut().values["border"]["on"] = Value::Bool(value);
        let s = st.borrow();
        NiriConfig::set_border(
            value,
            s.border_width.get_value() as i64,
            &s.border_active.get_value(),
            &s.border_inactive.get_value(),
        );
        drop(s);
        NiriConfig::reload();
    })
}

/// Equivalente a _on_focus_ring_toggle.
fn on_focus_ring_toggle(state: &NiriStateRef) -> Box<dyn Fn(bool)> {
    let st = Rc::clone(state);
    Box::new(move |value: bool| {
        st.borrow_mut().values["focus_ring"] = Value::Bool(value);
        NiriConfig::set_focus_ring(value);
        NiriConfig::reload();
    })
}

/// Equivalente a _on_csd_toggle.
fn on_csd_toggle(state: &NiriStateRef) -> Box<dyn Fn(bool)> {
    let st = Rc::clone(state);
    Box::new(move |value: bool| {
        st.borrow_mut().values["prefer_no_csd"] = Value::Bool(value);
        NiriConfig::set_prefer_no_csd(value);
        NiriConfig::reload();
    })
}

/// Equivalente a _on_animations_toggle.
fn on_animations_toggle(state: &NiriStateRef) -> Box<dyn Fn(bool)> {
    let st = Rc::clone(state);
    Box::new(move |value: bool| {
        st.borrow_mut().values["animations"] = Value::Bool(value);
        NiriConfig::set_animations(value);
        NiriConfig::reload();
    })
}
