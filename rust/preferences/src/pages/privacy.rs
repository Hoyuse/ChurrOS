// ==========================================
// PrivacyPage — permisos, firewall y diagnóstico
// (equivalente a pages/privacy.py)
// ==========================================


use crate::services::privacy::PrivacyService;
use crate::widgets::group::Group;
use crate::widgets::page::Page;
use crate::widgets::switch_row::SwitchRow;

pub fn build(navigator: gtk::Stack) -> Page {
    let page = Page::new(
        Some(navigator),
        "Privacidad",
        Some("Configuración de privacidad y seguridad"),
        None,
    );

    // Permisos
    let mut permissions = Group::new("Permisos");

    permissions.add(&SwitchRow::new(
        "Servicios de ubicación",
        Some("privacy.svg"),
        Some("Permitir que las aplicaciones accedan a la ubicación"),
        PrivacyService::location(),
        Some(Box::new(|v| PrivacyService::set_location(v))),
    ));

    permissions.add(&SwitchRow::new(
        "Acceso a la cámara",
        Some("privacy.svg"),
        Some("Permitir el uso de la cámara"),
        PrivacyService::camera(),
        Some(Box::new(|v| PrivacyService::set_camera(v))),
    ));

    permissions.add(&SwitchRow::new(
        "Acceso al micrófono",
        Some("privacy.svg"),
        Some("Permitir el uso del micrófono"),
        PrivacyService::microphone(),
        Some(Box::new(|v| PrivacyService::set_microphone(v))),
    ));

    page.add(permissions.widget());

    // Firewall
    let mut firewall = Group::new("Firewall");

    firewall.add(&SwitchRow::new(
        "Firewall (ufw)",
        Some("privacy.svg"),
        Some("Activar el firewall del sistema"),
        PrivacyService::firewall(),
        Some(Box::new(|v| {
            PrivacyService::set_firewall(v);
        })),
    ));

    page.add(firewall.widget());

    // Diagnóstico
    let mut diagnostics = Group::new("Diagnóstico");

    diagnostics.add(&SwitchRow::new(
        "Enviar estadísticas",
        Some("privacy.svg"),
        Some("Compartir información anónima para mejorar ChurrOS"),
        PrivacyService::telemetry(),
        Some(Box::new(|v| PrivacyService::set_telemetry(v))),
    ));

    page.add(diagnostics.widget());

    page
}
