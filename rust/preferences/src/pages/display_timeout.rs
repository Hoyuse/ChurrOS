// ==========================================
// DisplayTimeoutPage — tiempo de apagado de pantalla
// (equivalente a pages/display_timeout.py)
// ==========================================


use crate::services::power::PowerService;
use crate::services::settings;
use crate::widgets::combo_row::ComboRow;
use crate::widgets::group::Group;
use crate::widgets::page::Page;

const OPTIONS: [(&str, i64); 7] = [
    ("1 minuto", 60),
    ("2 minutos", 120),
    ("5 minutos", 300),
    ("10 minutos", 600),
    ("15 minutos", 900),
    ("30 minutos", 1800),
    ("Nunca", 0),
];

pub fn build(navigator: gtk::Stack) -> Page {
    let page = Page::new(
        Some(navigator),
        "Apagar pantalla",
        Some("Tiempo de inactividad antes de apagar la pantalla"),
        Some("display".to_string()),
    );

    let mut group = Group::new("Tiempo de espera");

    let labels: Vec<&str> = OPTIONS.iter().map(|(l, _)| *l).collect();
    let current = PowerService::screen_timeout();

    let mut selected = "Nunca";
    for (label, value) in OPTIONS.iter() {
        if current == *value {
            selected = label;
            break;
        }
    }

    let combo = ComboRow::new(
        "Apagar tras",
        &labels,
        Some(selected),
        None,
        None,
        Some(Box::new(|label| {
            for (lbl, value) in OPTIONS.iter() {
                if lbl == &label {
                    if *value == 0 {
                        settings::set("display.timeout", serde_json::json!(0));
                    }
                    PowerService::set_screen_timeout(*value);
                    return;
                }
            }
        })),
    );

    group.add(&combo);
    page.add(group.widget());

    page
}
