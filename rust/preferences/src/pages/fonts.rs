// ==========================================
// FontsPage — fuentes del sistema (equivalente a pages/fonts.py)
// ==========================================

use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use gtk::prelude::*;

use crate::services::fonts::FontService;
use crate::widgets::group::Group;
use crate::widgets::page::Page;
use crate::widgets::row::Row;
use crate::widgets::slider_row::SliderRow;

pub fn build(navigator: gtk::Stack) -> Page {
    let page = Page::new(
        Some(navigator.clone()),
        "Fuentes",
        Some("Selecciona la fuente del sistema"),
        Some("appearance".to_string()),
    );

    // Debounce de la escala (equivalente a self._pending)
    let pending: Rc<RefCell<bool>> = Rc::new(RefCell::new(false));

    // ===== Vista previa =====
    let current = FontService::current();

    let preview_group = Group::new("Vista previa");

    let preview_label = gtk::Label::new(Some("La zorra marrona salta sobre el perro perezoso"));
    preview_label.add_css_class("fonts-preview");
    preview_label.set_margin_top(10);
    preview_label.set_margin_bottom(10);
    preview_label.set_margin_start(14);
    preview_label.set_margin_end(14);

    // Gtk::Label no implementa AsWidget; se añade directo a la tarjeta del grupo
    preview_group.card.append(&preview_label);

    page.add(preview_group.widget());

    // ===== Escala =====
    let mut scale_group = Group::new("Escala de fuentes");

    let scale_slider: Rc<RefCell<Option<SliderRow>>> = Rc::new(RefCell::new(None));

    let slider_cb = Rc::clone(&scale_slider);
    let pending_cb = Rc::clone(&pending);
    let slider = SliderRow::new(
        "Escala",
        None,
        Some("Tamano relativo del texto"),
        80.0,
        150.0,
        5.0,
        FontService::scale() * 100.0,
        Some(Box::new(move |_| {
            schedule_apply(&pending_cb, &slider_cb);
        })),
    );

    *scale_slider.borrow_mut() = Some(slider);
    scale_group.add(&scale_slider.borrow().as_ref().unwrap().row);

    page.add(scale_group.widget());

    // ===== Fuentes =====
    let mut group = Group::new("Fuentes instaladas");

    let fonts = FontService::available();

    if fonts.is_empty() {
        group.add(&Row::new(
            "No se encontraron fuentes",
            Some("Instala una fuente"),
            Some("font.svg"),
            None,
            None,
            None,
        ));
    } else {
        for font in fonts {
            let font_name = font.clone();
            let subtitle = if font == current { Some("Seleccionada") } else { None };
            let nav = navigator.clone();

            // Callback: seleccionar fuente + volver a apariencia (equivalente a self.select)
            group.add(&Row::new(
                &font,
                subtitle,
                Some("font.svg"),
                None,
                None,
                Some(Box::new(move |_| {
                    FontService::set(&font_name);
                    // _refresh_preview() del Python solo hace set_opacity(1.0)
                    // (código muerto: nada cambia la opacidad) — omitido a propósito.
                    nav.set_visible_child_name("appearance");
                })),
            ));
        }
    }

    page.add(group.widget());

    page
}

/// Equivalente a FontsPage._schedule_apply: aplica la escala 400ms después
/// del último cambio (debounce). El try/except del Python no hace falta:
/// FontService::set_scale no puede fallar.
fn schedule_apply(pending: &Rc<RefCell<bool>>, slider: &Rc<RefCell<Option<SliderRow>>>) {
    if *pending.borrow() {
        return;
    }
    *pending.borrow_mut() = true;

    let pending = Rc::clone(pending);
    let slider = Rc::clone(slider);

    glib::timeout_add_local(Duration::from_millis(400), move || {
        *pending.borrow_mut() = false;
        if let Some(sl) = slider.borrow().as_ref() {
            FontService::set_scale(sl.get_value() / 100.0);
        }
        glib::ControlFlow::Break
    });
}
