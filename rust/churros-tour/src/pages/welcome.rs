use adw::prelude::*;
use gtk::prelude::*;

pub fn build() -> gtk::Box {
    let container = gtk::Box::new(gtk::Orientation::Vertical, 16);
    container.set_valign(gtk::Align::Center);
    container.set_halign(gtk::Align::Center);
    
    // Podemos usar adw::StatusPage o implementarlo manual
    let status_page = adw::StatusPage::builder()
        .title("¡Bienvenido a ChurrOS!")
        .description("Gracias por instalar ChurrOS. En este breve tour te mostraremos lo básico para empezar y configuraremos tu sistema según tus necesidades.")
        .icon_name("churros-logo-symbolic") // Si tenemos un icono, o "start-here-symbolic"
        .build();

    container.append(&status_page);
    
    container
}
