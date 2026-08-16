// ==========================================
// DateTimePage — fecha, hora, NTP y zona horaria
// (equivalente a pages/datetime.py)
// ==========================================

use gtk::prelude::*;

use std::cell::RefCell;
use std::rc::Rc;

use crate::services::datetime::DatetimeService;
use crate::widgets::group::Group;
use crate::widgets::page::Page;
use crate::widgets::row::Row;
use crate::widgets::switch_row::SwitchRow;

struct TzUi {
    entry: gtk::SearchEntry,
    popover: gtk::Popover,
    listbox: gtk::ListBox,
    status_row: Row,
    tz_row: Row,
    rtc_row: Row,
    ntp_switch: gtk::Switch,
    clock_row: Row,
    date_row: Row,
}

fn short_zone(tz: &str) -> String {
    tz.split('/').next_back().unwrap_or(tz).replace('_', " ")
}

pub fn build(navigator: gtk::Stack) -> Page {
    let page = Page::new(
        Some(navigator),
        "Fecha y hora",
        Some("Configura la hora, fecha y zona horaria del sistema"),
        None,
    );

    // ============ Estado actual ============
    let mut info = Group::new("Estado actual");

    let clock_row = Row::new(
        "Hora del sistema",
        Some(&now_hm()),
        Some("system.svg"),
        None,
        None,
        None,
    );
    info.add(&clock_row);

    let date_row = Row::new(
        "Fecha",
        Some(&now_date()),
        Some("system.svg"),
        None,
        None,
        None,
    );
    info.add(&date_row);

    let tz = DatetimeService::get_timezone();
    let tz_row = Row::new(
        "Zona horaria",
        Some(if tz.is_empty() { "Desconocida" } else { &tz }),
        Some("system.svg"),
        None,
        None,
        None,
    );
    info.add(&tz_row);

    let rtc = DatetimeService::get_rtc_time();
    let rtc_row = Row::new(
        "Reloj hardware (RTC)",
        Some(if rtc.is_empty() { "No disponible" } else { &rtc }),
        Some("system.svg"),
        None,
        None,
        None,
    );
    info.add(&rtc_row);

    page.add(info.widget());

    // ============ NTP ============
    let mut ntp_group = Group::new("Sincronización automática");

    let ntp_active = DatetimeService::get_ntp();
    let ntp_switch_widget = gtk::Switch::new();
    ntp_switch_widget.set_active(ntp_active);
    ntp_switch_widget.set_valign(gtk::Align::Center);

    let ntp_row = Row::new(
        "NTP (Network Time Protocol)",
        Some("Mantiene la hora sincronizada con servidores de internet"),
        None,
        None,
        Some(ntp_switch_widget.upcast_ref()),
        None,
    );
    ntp_group.add(&ntp_row);
    page.add(ntp_group.widget());

    // ============ Zona horaria ============
    let mut tz_group = Group::new("Cambiar zona horaria");

    let search_box = gtk::Box::new(gtk::Orientation::Vertical, 6);
    search_box.set_margin_start(14);
    search_box.set_margin_end(14);
    search_box.set_margin_top(8);
    search_box.set_margin_bottom(8);

    let search_label = gtk::Label::new(Some("Buscar zona horaria"));
    search_label.set_xalign(0.0);
    search_label.add_css_class("row-title");
    search_box.append(&search_label);

    let tz_entry = gtk::SearchEntry::new();
    tz_entry.set_placeholder_text(Some("Escribe una ciudad o region (ej. Madrid, Bogota, Tokyo)"));
    search_box.append(&tz_entry);
    tz_group.add(search_box.upcast_ref::<gtk::Widget>());

    let tz_popover = gtk::Popover::new();
    tz_popover.set_parent(&tz_entry);
    tz_popover.set_position(gtk::PositionType::Bottom);
    tz_popover.set_autohide(true);
    tz_popover.set_size_request(420, 320);

    let scrolled = gtk::ScrolledWindow::new();
    scrolled.set_max_content_height(360);
    scrolled.set_propagate_natural_height(true);

    let tz_listbox = gtk::ListBox::new();
    tz_listbox.set_selection_mode(gtk::SelectionMode::Single);
    scrolled.set_child(Some(&tz_listbox));
    tz_popover.set_child(Some(&scrolled));

    let zone_short = DatetimeService::current_zone_short();
    let status_subtitle: String = if zone_short.is_empty() {
        "Sin definir".to_string()
    } else {
        zone_short
    };
    let status_row = Row::new(
        "Zona actual",
        Some(&status_subtitle),
        Some("system.svg"),
        None,
        None,
        None,
    );
    tz_group.add(&status_row);

    page.add(tz_group.widget());

    // ============ Estado compartido ============
    let ui = Rc::new(TzUi {
        entry: tz_entry.clone(),
        popover: tz_popover.clone(),
        listbox: tz_listbox.clone(),
        status_row,
        tz_row,
        rtc_row,
        ntp_switch: ntp_switch_widget.clone(),
        clock_row,
        date_row,
    });

    let all_zones: Vec<String> = DatetimeService::list_timezones();
    let filtered: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(all_zones.clone()));

    populate_tz_list(&ui.listbox, &filtered.borrow(), &[]);

    // Filtro de búsqueda
    {
        let ui = Rc::clone(&ui);
        let all_zones = all_zones.clone();
        let filtered = Rc::clone(&filtered);
        tz_entry.connect_search_changed(move |entry| {
            let query = entry.text().to_lowercase();
            let query = query.trim();

            let new_filtered: Vec<String> = if query.is_empty() {
                all_zones.clone()
            } else {
                all_zones
                    .iter()
                    .filter(|z| z.to_lowercase().contains(query))
                    .cloned()
                    .collect()
            };

            let query_owned = query.to_string();
            *filtered.borrow_mut() = new_filtered.clone();
            populate_tz_list(&ui.listbox, &new_filtered, &[&query_owned]);

            if !new_filtered.is_empty() {
                ui.popover.popup();
            } else {
                ui.popover.popdown();
            }
        });
    }

    // Selección de zona
    {
        let ui = Rc::clone(&ui);
        let filtered = Rc::clone(&filtered);
        tz_listbox.connect_row_activated(move |_lb, row| {
            let idx = row.index() as usize;
            let tz = filtered.borrow().get(idx).cloned().unwrap_or_default();
            if tz.is_empty() {
                return;
            }

            ui.popover.popdown();
            ui.entry.set_text("");

            ui.status_row.set_subtitle(&format!("Aplicando zona horaria {tz}..."));

            // Aplicar en thread (timedatectl puede tardar; pkexec puede pedir clave).
            // Rc<TzUi> no es Send: el thread devuelve el resultado por canal.
            let (tx, rx) = std::sync::mpsc::channel::<(String, bool)>();
            let tz_clone = tz.clone();
            std::thread::spawn(move || {
                let ok = DatetimeService::set_timezone(&tz_clone);
                let _ = tx.send((tz_clone, ok));
            });

            let ui = Rc::clone(&ui);
            glib::timeout_add_local(std::time::Duration::from_millis(100), move || {
                match rx.try_recv() {
                    Ok((tz_done, ok)) => {
                        if ok {
                            ui.status_row
                                .set_subtitle(&format!("Zona horaria cambiada a {tz_done}"));
                        } else {
                            ui.status_row.set_subtitle(
                                "No se pudo cambiar la zona. Verifica que tienes pkexec instalado.",
                            );
                        }
                        refresh_info(&ui);
                        glib::ControlFlow::Break
                    }
                    Err(_) => glib::ControlFlow::Continue,
                }
            });
        });
    }

    // Toggle NTP
    {
        let ui = Rc::clone(&ui);
        ntp_switch_widget.connect_state_set(move |_sw, enabled| {
            ui.status_row.set_subtitle(if enabled {
                "Activando sincronizacion automatica..."
            } else {
                "Desactivando sincronizacion automatica..."
            });

            let (tx, rx) = std::sync::mpsc::channel::<bool>();
            std::thread::spawn(move || {
                let ok = DatetimeService::set_ntp(enabled);
                let _ = tx.send(ok);
            });

            let ui = Rc::clone(&ui);
            glib::timeout_add_local(std::time::Duration::from_millis(100), move || {
                match rx.try_recv() {
                    Ok(ok) => {
                        if ok {
                            ui.status_row.set_subtitle(if enabled {
                                "NTP activado: hora sincronizada con internet"
                            } else {
                                "NTP desactivado"
                            });
                        } else {
                            ui.status_row.set_subtitle(
                                "No se pudo cambiar NTP. Verifica pkexec o systemd-timesyncd.",
                            );
                        }
                        refresh_info(&ui);
                        glib::ControlFlow::Break
                    }
                    Err(_) => glib::ControlFlow::Continue,
                }
            });
            glib::Propagation::Proceed
        });
    }

    // Tick de reloj (1s)
    {
        let ui = Rc::clone(&ui);
        glib::timeout_add_local(std::time::Duration::from_secs(1), move || {
            ui.clock_row.set_subtitle(&now_hm());
            ui.date_row.set_subtitle(&now_date());
            glib::ControlFlow::Continue
        });
    }

    page
}

fn refresh_info(ui: &TzUi) {
    let tz = DatetimeService::get_timezone();
    if !tz.is_empty() {
        ui.tz_row.set_subtitle(&tz);
    }
    // NOTA: no tocar status_row aquí — los callbacks lo usan como feedback
    // ("Zona horaria cambiada a ...") y antes se sobrescribía al instante.
    ui.ntp_switch.set_active(DatetimeService::get_ntp());
    let rtc = DatetimeService::get_rtc_time();
    if !rtc.is_empty() {
        ui.rtc_row.set_subtitle(&rtc);
    }
}

fn populate_tz_list(listbox: &gtk::ListBox, zones: &[String], _filters: &[&str]) {
    // Limpiar
    while let Some(child) = listbox.first_child() {
        listbox.remove(&child);
    }

    if zones.is_empty() {
        let empty = gtk::ListBoxRow::new();
        let label = gtk::Label::new(Some("Sin resultados"));
        label.set_margin_top(10);
        label.set_margin_bottom(10);
        label.set_margin_start(12);
        empty.set_child(Some(&label));
        listbox.append(&empty);
        return;
    }

    for tz in zones.iter().take(200) {
        let row = gtk::ListBoxRow::new();
        let box_ = gtk::Box::new(gtk::Orientation::Vertical, 2);
        box_.set_margin_top(6);
        box_.set_margin_bottom(6);
        box_.set_margin_start(12);
        box_.set_margin_end(12);

        let title = gtk::Label::new(Some(&short_zone(tz)));
        title.set_xalign(0.0);
        title.add_css_class("row-title");
        box_.append(&title);

        if *tz != short_zone(tz) {
            let path = gtk::Label::new(Some(tz));
            path.set_xalign(0.0);
            path.add_css_class("row-subtitle");
            box_.append(&path);
        }

        row.set_child(Some(&box_));
        listbox.append(&row);
    }
}

fn now_hm() -> String {
    match glib::DateTime::now_local() {
        Ok(dt) => dt.format("%H:%M:%S").map(|s| s.to_string()).unwrap_or_default(),
        Err(_) => "".to_string(),
    }
}

fn now_date() -> String {
    match glib::DateTime::now_local() {
        Ok(dt) => dt.format("%A %d de %B de %Y").map(|s| s.to_string()).unwrap_or_default(),
        Err(_) => "".to_string(),
    }
}
