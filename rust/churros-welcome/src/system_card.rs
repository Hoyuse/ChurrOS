use gtk::prelude::*;

use crate::system_info;

pub fn build() -> gtk::Box {
    let card = gtk::Box::new(gtk::Orientation::Vertical, 10);

    card.set_size_request(280, 340); // mismo tamaño que las action cards
    card.add_css_class("system-card");

    card.set_margin_top(16);
    card.set_margin_bottom(16);
    card.set_margin_start(16);
    card.set_margin_end(16);

    //
    // Cabecera de la card (Icono + Título)
    //

    let header_box = gtk::Box::new(gtk::Orientation::Horizontal, 10);
    header_box.set_halign(gtk::Align::Start);

    let icon = gtk::Image::from_icon_name("computer-symbolic");
    icon.set_pixel_size(24);
    icon.add_css_class("card-icon");

    let title = gtk::Label::new(Some("Información"));
    title.add_css_class("card-title");

    header_box.append(&icon);
    header_box.append(&title);
    card.append(&header_box);

    let separator = gtk::Separator::new(gtk::Orientation::Horizontal);
    separator.set_margin_bottom(6);
    card.append(&separator);

    //
    // Información
    //

    card.append(&create_row("CPU", &system_info::get_cpu()));
    card.append(&create_row("RAM", &system_info::get_memory()));
    card.append(&create_row("Disco", &system_info::get_root_filesystem()));
    card.append(&create_row("Uptime", &system_info::get_uptime()));
    card.append(&create_row("Kernel", &system_info::get_kernel()));
    card.append(&create_row("SO", &system_info::get_os()));
    card.append(&create_row("Arquitectura", &system_info::get_architecture()));
    card.append(&create_row("Hostname", &system_info::get_hostname()));

    card
}

fn create_row(key: &str, value: &str) -> gtk::Box {
    let row = gtk::Box::new(gtk::Orientation::Horizontal, 10);

    row.add_css_class("system-row");

    let key_label = gtk::Label::new(Some(key));

    key_label.add_css_class("system-key");

    key_label.set_halign(gtk::Align::Start);

    key_label.set_hexpand(true);

    let value_label = gtk::Label::new(Some(value));

    value_label.add_css_class("system-value");

    value_label.set_halign(gtk::Align::End);

    value_label.set_wrap(true);
    // La RAM ya trae el porcentaje y el disco las unidades: encolumnar ambos
    // extremos con la etiqueta a la izquierda hace la tarjeta más legible.
    value_label.set_xalign(1.0);

    row.append(&key_label);
    row.append(&value_label);

    row
}