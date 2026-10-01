// ==========================================
// BatteryPage — estado y nivel de batería
// (equivalente a pages/battery.py)
// ==========================================


use crate::services::power::PowerService;
use crate::widgets::group::Group;
use crate::widgets::page::Page;
use crate::widgets::row::Row;

pub fn build(navigator: gtk::Stack) -> Page {
    let page = Page::new(
        Some(navigator),
        "Batería",
        Some("Estado y nivel de carga"),
        Some("power".to_string()),
    );

    let mut info = Group::new("Estado");

    if !PowerService::battery_present() {
        info.add(&Row::new(
            "Batería",
            Some("No se detecta ninguna batería"),
            Some("power.svg"),
            Some("—"),
            None,
            None,
        ));
        page.add(info.widget());
        return page;
    }

    info.add(&Row::new(
        "Nivel de carga",
        Some("Porcentaje actual"),
        Some("power.svg"),
        Some(&format!("{} %", PowerService::battery_percentage())),
        None,
        None,
    ));

    info.add(&Row::new(
        "Estado",
        Some("Cargando / descargando / llena"),
        Some("power.svg"),
        Some(&PowerService::battery_state()),
        None,
        None,
    ));

    info.add(&Row::new(
        "Modo de energía",
        Some("Perfil activo del sistema"),
        Some("power.svg"),
        Some(&PowerService::power_profile()),
        None,
        None,
    ));

    page.add(info.widget());

    page
}
