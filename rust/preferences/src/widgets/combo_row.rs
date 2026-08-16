// ==========================================
// ComboRow — fila con Gtk::DropDown
// (equivalente a widgets/combo_row.py)
// ==========================================

use gtk::prelude::*;

use std::cell::RefCell;
use std::rc::Rc;

pub struct ComboRow {
    pub root: gtk::Box,
    pub combo: gtk::DropDown,
    values: Rc<RefCell<Vec<String>>>,
    callback: Rc<RefCell<Option<Box<dyn Fn(&str)>>>>,
}

impl ComboRow {
    pub fn new(
        title: &str,
        values: &[&str],
        selected: Option<&str>,
        subtitle: Option<&str>,
        icon: Option<&str>,
        callback: Option<Box<dyn Fn(&str)>>,
    ) -> Self {
        let root = gtk::Box::new(gtk::Orientation::Horizontal, 14);
        root.add_css_class("row");
        root.set_margin_top(12);
        root.set_margin_bottom(12);
        root.set_margin_start(14);
        root.set_margin_end(14);

        if let Some(icon) = icon {
            if let Some(image) = crate::assets::icon_image(icon, 22) {
                root.append(&image);
            }
        }

        let labels = gtk::Box::new(gtk::Orientation::Vertical, 2);
        labels.set_hexpand(true);

        let title_label = gtk::Label::new(Some(title));
        title_label.set_xalign(0.0);
        title_label.add_css_class("row-title");
        labels.append(&title_label);

        if let Some(sub) = subtitle {
            let sub_label = gtk::Label::new(Some(sub));
            sub_label.set_xalign(0.0);
            sub_label.add_css_class("row-subtitle");
            labels.append(&sub_label);
        }

        root.append(&labels);

        // Si el valor seleccionado no está en la lista, añadirlo al principio:
        // así el combo nunca muestra un índice 0 erróneo y no se pisa el valor
        // real del usuario al guardar.
        let mut effective: Vec<&str> = values.to_vec();
        if let Some(sel) = selected {
            if !sel.is_empty() && !effective.contains(&sel) {
                effective.insert(0, sel);
            }
        }
        let values = effective;

        let model = gtk::StringList::new(&[]);
        for v in &values {
            model.append(v);
        }

        let combo = gtk::DropDown::new(Some(model), None::<gtk::PropertyExpression>);
        if let Some(sel) = selected {
            if let Some(idx) = values.iter().position(|v| *v == sel) {
                combo.set_selected(idx as u32);
            }
        }
        combo.set_valign(gtk::Align::Center);

        root.append(&combo);

        let values_owned: Rc<RefCell<Vec<String>>> =
            Rc::new(RefCell::new(values.iter().map(|s| s.to_string()).collect()));
        let callback_rc = Rc::new(RefCell::new(callback));

        if callback_rc.borrow().is_some() {
            let cb = Rc::clone(&callback_rc);
            let values = Rc::clone(&values_owned);
            combo.connect_selected_notify(move |combo| {
                let idx = combo.selected() as usize;
                if let Some(value) = values.borrow().get(idx) {
                    if let Some(cb) = cb.borrow().as_ref() {
                        cb(value);
                    }
                }
            });
        }

        Self {
            root,
            combo,
            values: values_owned,
            callback: callback_rc,
        }
    }

    pub fn widget(&self) -> &gtk::Box {
        &self.root
    }

    pub fn value(&self) -> Option<String> {
        let idx = self.combo.selected() as usize;
        self.values.borrow().get(idx).cloned()
    }

    pub fn set_values(&self, values: &[&str], selected: Option<&str>) {
        let model = gtk::StringList::new(&[]);
        for v in values {
            model.append(v);
        }
        self.combo.set_model(Some(&model));
        // El callback lee `self.values` compartido, así que actualizarlo aquí
        // lo mantiene sincronizado (antes usaba una copia obsoleta).
        *self.values.borrow_mut() = values.iter().map(|s| s.to_string()).collect();
        let idx = selected
            .and_then(|sel| self.values.borrow().iter().position(|v| v == sel))
            .unwrap_or(0);
        self.combo.set_selected(idx as u32);
    }
}

impl crate::widgets::AsWidget for ComboRow {
    fn widget(&self) -> &gtk::Widget {
        self.root.upcast_ref()
    }
}
