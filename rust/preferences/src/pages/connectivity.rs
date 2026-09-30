// ==========================================
// ConnectivityPage — Wi-Fi y Bluetooth
// (equivalente a pages/connectivity.py)
// ==========================================


use std::cell::RefCell;
use std::rc::Rc;

use gtk::prelude::*;

use crate::services::connectivity::ConnectivityService;
use crate::widgets::group::Group;
use crate::widgets::page::Page;
use crate::widgets::row::Row;
use crate::widgets::switch_row::SwitchRow;

// Datos Send (sin Rc ni widgets) para cargar en thread
#[derive(Default)]
struct WifiData {
    available: bool,
    enabled: bool,
    current: Option<String>,
    networks: Vec<(String, i64, String, bool, bool)>, // ssid, signal, security, connected, saved
}

#[derive(Default)]
struct BtData {
    available: bool,
    enabled: bool,
    devices: Vec<(String, String)>, // name, mac
}

struct ConnectivityData {
    wifi: WifiData,
    bluetooth: BtData,
}

fn load_data() -> ConnectivityData {
    ConnectivityData {
        wifi: WifiData {
            available: ConnectivityService::wifi_available(),
            enabled: ConnectivityService::wifi_enabled(),
            current: ConnectivityService::current_network(),
            networks: ConnectivityService::wifi_networks_full()
                .into_iter()
                .map(|n| (n.ssid, n.signal, n.security, n.connected, n.saved))
                .collect(),
        },
        bluetooth: BtData {
            available: ConnectivityService::bluetooth_available(),
            enabled: ConnectivityService::bluetooth_enabled(),
            devices: ConnectivityService::bluetooth_devices()
                .into_iter()
                .map(|d| (d.name, d.mac))
                .collect(),
        },
    }
}

pub fn build(navigator: gtk::Stack) -> Page {
    let page = Page::new(
        Some(navigator),
        "Conectividad",
        Some("Wi-Fi y Bluetooth"),
        None,
    );

    let wifi_group = Rc::new(RefCell::new(Group::new("Wi-Fi")));
    let bluetooth_group = Rc::new(RefCell::new(Group::new("Bluetooth")));

    wifi_group
        .borrow_mut()
        .add(&Row::new("Cargando...", None, None, None, None, None));
    bluetooth_group
        .borrow_mut()
        .add(&Row::new("Cargando...", None, None, None, None, None));

    page.add(wifi_group.borrow().widget());
    page.add(bluetooth_group.borrow().widget());

    // reload: función reutilizable que lanza la carga en thread y repuebla.
    // Se guarda en un Rc para poder pasarla a los callbacks de los switches
    // y a la fila "Recargar redes".
    let reload: Rc<RefCell<Option<Rc<dyn Fn()>>>> = Rc::new(RefCell::new(None));

    let start_load = {
        let wg = Rc::clone(&wifi_group);
        let bg = Rc::clone(&bluetooth_group);
        let reload = Rc::clone(&reload);
        move || {
            let (tx, rx) = std::sync::mpsc::channel::<ConnectivityData>();
            std::thread::spawn(move || {
                let _ = tx.send(load_data());
            });
            let wg = Rc::clone(&wg);
            let bg = Rc::clone(&bg);
            let reload = Rc::clone(&reload);
            glib::timeout_add_local(std::time::Duration::from_millis(100), move || {
                match rx.try_recv() {
                    Ok(data) => {
                        populate(&wg, &bg, data, &reload);
                        glib::ControlFlow::Break
                    }
                    Err(_) => glib::ControlFlow::Continue,
                }
            });
        }
    };

    *reload.borrow_mut() = Some(Rc::new(start_load.clone()));
    start_load();

    page
}

/// Lanza un reload (recarga de datos) si ya está inicializado.
fn trigger_reload(reload: &Rc<RefCell<Option<Rc<dyn Fn()>>>>) {
    if let Some(f) = reload.borrow().as_ref() {
        f();
    }
}

fn populate(
    wifi_group: &Rc<RefCell<Group>>,
    bluetooth_group: &Rc<RefCell<Group>>,
    data: ConnectivityData,
    reload: &Rc<RefCell<Option<Rc<dyn Fn()>>>>,
) {
    // ============ Wi-Fi ============
    wifi_group.borrow_mut().clear();

    if !data.wifi.available {
        wifi_group.borrow_mut().add(&Row::new(
            "No se encontró un adaptador Wi-Fi",
            None,
            None,
            None,
            None,
            None,
        ));
    } else {
        let wifi_reload = Rc::clone(reload);
        wifi_group.borrow_mut().add(&SwitchRow::new(
            "Activar Wi-Fi",
            None,
            None,
            data.wifi.enabled,
            Some(Box::new(move |active| {
                ConnectivityService::set_wifi(active);
                trigger_reload(&wifi_reload);
            })),
        ));

        if let Some(current) = &data.wifi.current {
            let current_owned = current.clone();
            wifi_group.borrow_mut().add(&Row::new(
                "Red actual",
                Some(&current_owned),
                None,
                None,
                None,
                None,
            ));
        }

        if data.wifi.networks.is_empty() {
            wifi_group.borrow_mut().add(&Row::new(
                "No se encontraron redes",
                None,
                None,
                None,
                None,
                None,
            ));
        } else {
            for (ssid, signal, security, connected, saved) in &data.wifi.networks {
                let mut parts = vec![format!("Señal: {signal}%")];
                if !security.is_empty() {
                    parts.push(security.clone());
                }
                if *connected {
                    parts.push("conectado".to_string());
                } else if *saved {
                    parts.push("guardada".to_string());
                }
                let subtitle = parts.join(" · ");

                let ssid_owned = ssid.clone();
                let security_owned = security.clone();
                wifi_group.borrow_mut().add(&Row::new(
                    ssid,
                    Some(&subtitle),
                    None,
                    None,
                    None,
                    Some(Box::new(move |_btn| {
                        if security_owned.is_empty() {
                            // Red abierta: conectar directo en thread
                            let ssid_for_thread = ssid_owned.clone();
                            std::thread::spawn(move || {
                                let (ok, err) =
                                    ConnectivityService::wifi_connect(&ssid_for_thread, None);
                                let _ = (ok, err);
                            });
                        } else {
                            let root = _btn.root().and_downcast::<gtk::Window>().expect("no root window");
                            let dialog = gtk::Window::builder()
                                .title(&format!("Conectar a {}", ssid_owned))
                                .modal(true)
                                .transient_for(&root)
                                .default_width(320)
                                .resizable(false)
                                .build();
                            
                            let vbox = gtk::Box::new(gtk::Orientation::Vertical, 16);
                            vbox.set_margin_top(16);
                            vbox.set_margin_bottom(16);
                            vbox.set_margin_start(16);
                            vbox.set_margin_end(16);
                            
                            let label = gtk::Label::new(Some(&format!("Introduce la contraseña para la red Wi-Fi protegida '{}':", ssid_owned)));
                            label.set_wrap(true);
                            label.set_xalign(0.0);
                            vbox.append(&label);
                            
                            let entry = gtk::PasswordEntry::new();
                            entry.set_show_peek_icon(true);
                            vbox.append(&entry);
                            
                            let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 8);
                            hbox.set_halign(gtk::Align::End);
                            
                            let cancel_btn = gtk::Button::with_label("Cancelar");
                            let dialog_clone = dialog.clone();
                            cancel_btn.connect_clicked(move |_| {
                                dialog_clone.close();
                            });
                            
                            let connect_btn = gtk::Button::with_label("Conectar");
                            connect_btn.add_css_class("suggested-action");
                            let dialog_clone2 = dialog.clone();
                            let entry_clone = entry.clone();
                            let ssid_for_connect = ssid_owned.clone();
                            connect_btn.connect_clicked(move |_| {
                                let password = entry_clone.text().to_string();
                                let ssid = ssid_for_connect.clone();
                                std::thread::spawn(move || {
                                    let _ = ConnectivityService::wifi_connect(&ssid, Some(&password));
                                });
                                dialog_clone2.close();
                            });
                            
                            hbox.append(&cancel_btn);
                            hbox.append(&connect_btn);
                            vbox.append(&hbox);
                            
                            dialog.set_child(Some(&vbox));
                            dialog.show();
                        }
                    })),
                ));
            }
        }

        let rescan_reload = Rc::clone(reload);
        wifi_group.borrow_mut().add(&Row::new(
            "Recargar redes",
            Some("Forzar un nuevo escaneo"),
            None,
            None,
            None,
            Some(Box::new(move |_btn| {
                ConnectivityService::rescan_wifi();
                trigger_reload(&rescan_reload);
            })),
        ));
    }

    // ============ Bluetooth ============
    bluetooth_group.borrow_mut().clear();

    if !data.bluetooth.available {
        bluetooth_group.borrow_mut().add(&Row::new(
            "No se encontró un adaptador Bluetooth",
            None,
            None,
            None,
            None,
            None,
        ));
    } else {
        let bluetooth_reload = Rc::clone(reload);
        bluetooth_group.borrow_mut().add(&SwitchRow::new(
            "Activar Bluetooth",
            None,
            None,
            data.bluetooth.enabled,
            Some(Box::new(move |active| {
                ConnectivityService::set_bluetooth(active);
                trigger_reload(&bluetooth_reload);
            })),
        ));

        if data.bluetooth.devices.is_empty() {
            bluetooth_group.borrow_mut().add(&Row::new(
                "No hay dispositivos",
                None,
                None,
                None,
                None,
                None,
            ));
        } else {
            for (name, mac) in &data.bluetooth.devices {
                bluetooth_group
                    .borrow_mut()
                    .add(&Row::new(name, Some(mac), None, None, None, None));
            }
        }
    }
}
