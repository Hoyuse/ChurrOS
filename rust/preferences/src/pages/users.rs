// ==========================================
// UsersPage — cuenta del sistema y autologin
// (equivalente a pages/users.py)
// ==========================================

use gtk::prelude::*;

use std::cell::Cell;
use std::rc::Rc;

use crate::services::users::UsersService;
use crate::widgets::group::Group;
use crate::widgets::page::Page;
use crate::widgets::row::Row;
use crate::widgets::switch_row::SwitchRow;

pub fn build(navigator: gtk::Stack) -> Page {
    let page = Page::new(
        Some(navigator),
        "Usuarios",
        Some("Administrar cuentas del sistema"),
        None,
    );

    // Cuenta
    let mut account = Group::new("Cuenta");

    account.add(&Row::new(
        "Usuario",
        Some("Sesión actual"),
        Some("users.svg"),
        Some(&UsersService::username()),
        None,
        None,
    ));

    account.add(&Row::new(
        "Nombre",
        Some("Nombre completo"),
        Some("users.svg"),
        Some(&UsersService::full_name()),
        None,
        None,
    ));

    page.add(account.widget());

    // Seguridad
    let mut security = Group::new("Seguridad");

    // Autologin: edita /etc/greetd con privilegios; si falla (pkexec
    // cancelado, sin permisos) se revierte el switch para no mentir.
    let autologin_row = SwitchRow::new(
        "Inicio automático",
        Some("users.svg"),
        Some("Iniciar sesión automáticamente"),
        UsersService::auto_login(),
        None,
    );
    let autologin_switch = autologin_row.switch.clone();
    let revert_guard = Rc::new(Cell::new(false));
    {
        let revert_guard = Rc::clone(&revert_guard);
        autologin_switch.connect_notify_local(Some("active"), move |switch, _| {
            if revert_guard.get() {
                return;
            }
            revert_guard.set(true);
            let active = switch.is_active();
            let ok = UsersService::set_auto_login(active);
            if !ok {
                switch.set_active(!active);
            }
            revert_guard.set(false);
        });
    }
    security.add(&autologin_row);

    page.add(security.widget());

    page
}
