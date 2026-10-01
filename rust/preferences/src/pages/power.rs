// ==========================================
// PowerPage — energía (perfil, batería, pantalla, suspensión)
// (equivalente a pages/power.py)
// ==========================================


use crate::widgets::group::Group;
use crate::widgets::navigation_row;
use crate::widgets::page::Page;
use crate::widgets::switch_row::SwitchRow;

pub fn build(navigator: gtk::Stack) -> Page {
    let page = Page::new(
        Some(navigator.clone()),
        "Energía",
        Some("Batería y rendimiento"),
        None,
    );

    // ============ Perfil ============
    let mut profile = Group::new("Perfil de energía");

    profile.add(&navigation_row::new(
        navigator.clone(),
        "Modo de energía",
        "power.svg",
        "power-profile",
        Some("Balanceado, ahorro o rendimiento"),
    ));

    page.add(profile.widget());

    // ============ Batería ============
    let mut battery = Group::new("Batería");

    battery.add(&navigation_row::new(
        navigator.clone(),
        "Estado de la batería",
        "power.svg",
        "battery",
        Some("Información sobre la batería"),
    ));

    battery.add(&SwitchRow::new(
        "Ahorro de energía",
        Some("power.svg"),
        Some("Reducir el consumo cuando sea posible"),
        false,
        None,
    ));

    page.add(battery.widget());

    // ============ Pantalla ============
    let mut display = Group::new("Pantalla");

    display.add(&navigation_row::new(
        navigator.clone(),
        "Apagar pantalla",
        "power.svg",
        "display-timeout",
        Some("Tiempo de espera de la pantalla"),
    ));

    page.add(display.widget());

    // ============ Suspensión ============
    let mut suspend = Group::new("Suspensión");

    suspend.add(&navigation_row::new(
        navigator.clone(),
        "Suspensión automática",
        "power.svg",
        "sleep",
        Some("Configurar la suspensión"),
    ));

    suspend.add(&SwitchRow::new(
        "Suspender al cerrar la tapa",
        Some("power.svg"),
        Some("Solo en portátiles"),
        true,
        None,
    ));

    page.add(suspend.widget());

    page
}
