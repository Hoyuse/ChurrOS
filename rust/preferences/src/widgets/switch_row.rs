// ==========================================
// SwitchRow — fila con Gtk::Switch
// (equivalente a widgets/switch_row.py)
// ==========================================

use gtk::prelude::*;

use crate::widgets::row::Row;

pub struct SwitchRow {
    pub row: Row,
    pub switch: gtk::Switch,
}

impl SwitchRow {
    pub fn new(
        title: &str,
        icon: Option<&str>,
        subtitle: Option<&str>,
        active: bool,
        callback: Option<Box<dyn Fn(bool)>>,
    ) -> Self {
        let switch = gtk::Switch::new();
        switch.set_active(active);
        switch.set_valign(gtk::Align::Center);
        // El Switch vive dentro de un GtkButton (la fila). Si el Switch
        // también recibe el clic, GTK dispara notify::active y luego el
        // clicked de la fila vuelve a togglear: dos ThemeService::set en
        // el mismo evento, con gtk-theme a medias, y la app se cierra.
        switch.set_can_target(false);

        let row = Row::new(
            title,
            subtitle,
            icon,
            None,
            Some(switch.upcast_ref::<gtk::Widget>()),
            None, // el clic en la fila togglea el switch abajo
        );

        // Click en la fila -> toggle del switch
        let switch_clone = switch.clone();
        row.widget().connect_clicked(move |_| {
            let new_state = !switch_clone.is_active();
            switch_clone.set_active(new_state);
        });

        // notify::active -> callback
        if let Some(cb) = callback {
            switch.connect_notify_local(Some("active"), move |switch, _| {
                cb(switch.is_active());
            });
        }

        Self { row, switch }
    }

    pub fn widget(&self) -> &gtk::Button {
        self.row.widget()
    }

    pub fn get_active(&self) -> bool {
        self.switch.is_active()
    }

    pub fn set_active(&self, active: bool) {
        self.switch.set_active(active);
    }
}
