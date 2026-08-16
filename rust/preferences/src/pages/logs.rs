// ==========================================
// LogsPage — logs de niri + validacion de config
// (equivalente a pages/logs.py)
// ==========================================

use gtk::prelude::*;

use std::cell::RefCell;

use crate::services::logs_service::LogsService;
use crate::widgets::group::Group;
use crate::widgets::page::Page;
use crate::widgets::row::Row;

// Equivalente a self._validate_status / self._log_view (los callbacks
// de Row son Fn(&gtk::Button); el estado se comparte via thread_local).
thread_local! {
    static VALIDATE_LABEL: RefCell<Option<gtk::Label>> = RefCell::new(None);
    static LOG_VIEW: RefCell<Option<gtk::TextView>> = RefCell::new(None);
}

struct WLabel(gtk::Label);
impl crate::widgets::AsWidget for WLabel {
    fn widget(&self) -> &gtk::Widget {
        self.0.upcast_ref()
    }
}

struct WBox(gtk::Box);
impl crate::widgets::AsWidget for WBox {
    fn widget(&self) -> &gtk::Widget {
        self.0.upcast_ref()
    }
}

pub fn build(navigator: gtk::Stack) -> Page {
    let page = Page::new(
        Some(navigator),
        "Logs de Niri",
        Some("Registros del compositor y validacion de config"),
        Some("system".to_string()),
    );

    // ---------- Validacion ----------
    let mut validate_group = Group::new("Validacion del config");

    let validate_status = gtk::Label::new(Some("Validando..."));
    validate_status.set_xalign(0.0);
    validate_status.set_margin_start(14);
    validate_status.set_margin_end(14);
    validate_status.set_margin_top(10);
    validate_status.set_margin_bottom(10);
    validate_status.set_wrap(true);

    VALIDATE_LABEL.with(|s| *s.borrow_mut() = Some(validate_status.clone()));

    validate_group.add(&WLabel(validate_status));

    validate_group.add(&Row::new(
        "Revalidar",
        Some("Ejecuta niri validate sobre tu config actual"),
        Some("logs.svg"),
        None,
        None,
        Some(Box::new(|_| refresh_validate())),
    ));

    page.add(validate_group.widget());

    refresh_validate();

    // ---------- Logs ----------
    let mut logs_group = Group::new("Registro de eventos");

    let scrolled = gtk::ScrolledWindow::new();
    scrolled.set_vexpand(true);
    scrolled.set_min_content_height(360);
    scrolled.set_max_content_height(420);
    scrolled.set_propagate_natural_height(true);

    let log_view = gtk::TextView::new();
    log_view.set_editable(false);
    log_view.set_monospace(true);
    log_view.set_wrap_mode(gtk::WrapMode::Char);

    LOG_VIEW.with(|s| *s.borrow_mut() = Some(log_view.clone()));

    scrolled.set_child(Some(&log_view));

    let box_ = gtk::Box::new(gtk::Orientation::Vertical, 8);

    box_.append(Row::new(
        "Actualizar",
        Some("Vuelve a leer el journal en busca de logs de niri"),
        Some("logs.svg"),
        None,
        None,
        Some(Box::new(|_| refresh_logs())),
    ).widget());

    box_.append(&scrolled);

    logs_group.add(&WBox(box_));
    page.add(logs_group.widget());

    refresh_logs();

    page
}

/// Equivalente a _set_validate.
fn set_validate(text: &str, css_class: &str) {
    VALIDATE_LABEL.with(|s| {
        if let Some(label) = s.borrow().as_ref() {
            let ctx = label.style_context();
            for c in ["row-title", "row-subtitle"] {
                ctx.remove_class(c);
            }
            ctx.add_class(css_class);
            label.set_label(text);
        }
    });
}

/// Equivalente a _refresh_validate (asíncrono: niri validate en un thread).
fn refresh_validate() {
    set_validate("Validando...", "row-subtitle");

    let (tx, rx) = std::sync::mpsc::channel::<(bool, String)>();
    std::thread::spawn(move || {
        let (ok, msg) = LogsService::niri_validate();
        let _ = tx.send((ok, msg));
    });

    glib::timeout_add_local(std::time::Duration::from_millis(100), move || {
        match rx.try_recv() {
            Ok((ok, msg)) => {
                if ok {
                    set_validate("Config valida.", "row-subtitle");
                } else {
                    set_validate(&format!("Config invalida:\n{msg}"), "row-title");
                }
                glib::ControlFlow::Break
            }
            Err(_) => glib::ControlFlow::Continue,
        }
    });
}

/// Equivalente a _set_log.
fn set_log(text: &str) {
    LOG_VIEW.with(|s| {
        if let Some(view) = s.borrow().as_ref() {
            let buffer = view.buffer();
            buffer.set_text(text);
            let mut end = buffer.end_iter();
            let _ = view.scroll_to_iter(&mut end, 0.0, false, 0.0, 1.0);
        }
    });
}

/// Equivalente a _refresh_logs (asíncrono: journalctl en un thread).
fn refresh_logs() {
    set_log("Cargando...");

    let (tx, rx) = std::sync::mpsc::channel::<String>();
    std::thread::spawn(move || {
        let text = LogsService::niri_logs(400);
        let _ = tx.send(text);
    });

    glib::timeout_add_local(std::time::Duration::from_millis(100), move || {
        match rx.try_recv() {
            Ok(text) => {
                if text.trim().is_empty() {
                    set_log("(sin logs)");
                } else {
                    set_log(&text);
                }
                glib::ControlFlow::Break
            }
            Err(_) => glib::ControlFlow::Continue,
        }
    });
}
