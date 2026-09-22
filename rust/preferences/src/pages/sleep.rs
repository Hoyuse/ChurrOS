// ==========================================
// SleepPage — suspensión automática y cierre de tapa
// (equivalente a pages/sleep.py)
// ==========================================


use crate::services::power::PowerService;
use crate::widgets::combo_row::ComboRow;
use crate::widgets::group::Group;
use crate::widgets::page::Page;
use crate::widgets::row::Row;

const OPTIONS: [(&str, i64); 6] = [
    ("5 minutos", 300),
    ("10 minutos", 600),
    ("15 minutos", 900),
    ("30 minutos", 1800),
    ("1 hora", 3600),
    ("Nunca", 0),
];

const LID_ACTIONS: [&str; 6] = ["suspend", "hibernate", "nothing", "blank", "logout", "shutdown"];

pub fn build(navigator: gtk::Stack) -> Page {
    let page = Page::new(
        Some(navigator),
        "Suspension",
        Some("Suspension automatica y acciones de tapa"),
        Some("power".to_string()),
    );

    // ============ Estado de batería ============
    let has_battery = PowerService::battery_present();
    if has_battery {
        let pct = PowerService::battery_percentage();
        let state = PowerService::battery_state();

        let state_desc = if state.contains("charging") {
            "Cargando"
        } else if state.contains("discharging") {
            "Descargando"
        } else {
            ""
        };

        let value = if pct >= 0 {
            if state_desc.is_empty() {
                format!("{pct:.0}%")
            } else {
                format!("{pct:.0}%  ({state_desc})")
            }
        } else {
            "Desconocido".to_string()
        };

        let mut battery_group = Group::new("Estado de la bateria");
        battery_group.add(&Row::new(
            "Nivel de carga",
            Some("El estado actual influye en el comportamiento de suspension"),
            Some("power.svg"),
            Some(&value),
            None,
            None,
        ));
        page.add(battery_group.widget());
    }

    // ============ Suspensión automática ============
    let mut group = Group::new("Suspension automatica");

    let labels: Vec<&str> = OPTIONS.iter().map(|(l, _)| *l).collect();
    let current = PowerService::sleep_timeout();

    let mut selected = "Nunca";
    for (label, value) in OPTIONS.iter() {
        if current == *value {
            selected = label;
            break;
        }
    }

    let combo = ComboRow::new(
        "Suspender tras",
        &labels,
        Some(selected),
        Some("Inactividad antes de que el sistema entre en suspension"),
        None,
        Some(Box::new(|label| {
            for (lbl, value) in OPTIONS.iter() {
                if lbl == &label {
                    PowerService::set_sleep_timeout(*value);
                    return;
                }
            }
        })),
    );

    group.add(&combo);
    page.add(group.widget());

    // ============ Cierre de tapa ============
    let mut lid = Group::new("Cierre de tapa");

    let current_action = PowerService::lid_close_action();
    let selected_action = if LID_ACTIONS.contains(&current_action.as_str()) {
        current_action.as_str()
    } else {
        "suspend"
    };

    let lid_combo = ComboRow::new(
        "Al cerrar la tapa",
        &LID_ACTIONS,
        Some(selected_action),
        None,
        None,
        Some(Box::new(|action| {
            PowerService::set_lid_close_action(&action);
        })),
    );

    lid.add(&lid_combo);
    page.add(lid.widget());

    page
}
