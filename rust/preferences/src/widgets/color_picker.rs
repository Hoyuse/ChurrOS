// ==========================================
// ColorPickerRow — fila con swatch, entry hex y diálogo de color
// (equivalente a widgets/color_picker.py)
// ==========================================

use gtk::prelude::*;

use std::cell::RefCell;
use std::rc::Rc;

pub struct ColorPickerRow {
    pub root: gtk::Box,
    color: RefCell<String>,
    swatch: gtk::DrawingArea,
    value_label: gtk::Label,
    entry: gtk::Entry,
    callback: Rc<RefCell<Option<Box<dyn Fn(&str)>>>>,
}

/// Parse "#rrggbb" (alpha externo) o "#rrggbbaa" (alpha propio).
fn hex_to_rgba(hex: &str, alpha: f64) -> (f64, f64, f64, f64) {
    let hex = hex.trim_start_matches('#');
    match hex.len() {
        6 => {
            let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(255) as f64 / 255.0;
            let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(255) as f64 / 255.0;
            let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(255) as f64 / 255.0;
            (r, g, b, alpha)
        }
        8 => {
            let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(255) as f64 / 255.0;
            let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(255) as f64 / 255.0;
            let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(255) as f64 / 255.0;
            let a = u8::from_str_radix(&hex[6..8], 16).unwrap_or(255) as f64 / 255.0;
            (r, g, b, a)
        }
        _ => (1.0, 1.0, 1.0, alpha),
    }
}

/// 6 dígitos si el alpha es total, 8 (con alpha) si no.
fn rgba_to_hex(r: f64, g: f64, b: f64, a: f64) -> String {
    let r = (r * 255.0).round().clamp(0.0, 255.0) as u8;
    let g = (g * 255.0).round().clamp(0.0, 255.0) as u8;
    let b = (b * 255.0).round().clamp(0.0, 255.0) as u8;
    if a >= 1.0 {
        format!("#{r:02x}{g:02x}{b:02x}")
    } else {
        let a = (a * 255.0).round().clamp(0.0, 255.0) as u8;
        format!("#{r:02x}{g:02x}{b:02x}{a:02x}")
    }
}

fn is_valid_hex(text: &str) -> bool {
    let t = text.strip_prefix('#').unwrap_or(text);
    (t.len() == 6 || t.len() == 8) && t.chars().all(|c| c.is_ascii_hexdigit())
}

impl ColorPickerRow {
    pub fn new(
        title: &str,
        value: &str,
        callback: Option<Box<dyn Fn(&str)>>,
        subtitle: Option<&str>,
    ) -> Self {
        let root = gtk::Box::new(gtk::Orientation::Horizontal, 12);
        root.add_css_class("row");
        root.set_margin_top(10);
        root.set_margin_bottom(10);
        root.set_margin_start(14);
        root.set_margin_end(14);

        let swatch = gtk::DrawingArea::new();
        swatch.set_size_request(40, 28);
        swatch.set_valign(gtk::Align::Center);
        swatch.add_css_class("color-swatch");

        let labels = gtk::Box::new(gtk::Orientation::Vertical, 2);
        labels.set_hexpand(true);

        let label = gtk::Label::new(Some(title));
        label.set_xalign(0.0);
        label.add_css_class("row-title");
        labels.append(&label);

        if let Some(sub) = subtitle {
            let sub_label = gtk::Label::new(Some(sub));
            sub_label.set_xalign(0.0);
            sub_label.set_wrap(true);
            sub_label.add_css_class("row-subtitle");
            labels.append(&sub_label);
        }

        let value_label = gtk::Label::new(Some(value));
        value_label.set_xalign(1.0);
        value_label.add_css_class("row-value");

        let button = gtk::Button::with_label("Elegir");
        button.add_css_class("color-pick-button");
        button.set_valign(gtk::Align::Center);

        let entry = gtk::Entry::new();
        entry.set_text(value);
        entry.set_width_chars(9);
        entry.set_max_length(9);
        entry.set_valign(gtk::Align::Center);
        entry.add_css_class("color-entry");

        root.append(&labels);
        root.append(&swatch);
        root.append(&entry);
        root.append(&value_label);
        root.append(&button);

        let color = RefCell::new(value.to_string());
        let callback_rc = Rc::new(RefCell::new(callback));

        // Swatch draw
        let color_for_draw = color.clone();
        swatch.set_draw_func(move |_area, cr, w, h| {
            let (r, g, b, a) = hex_to_rgba(&color_for_draw.borrow(), 1.0);
            cr.set_source_rgba(r, g, b, a);
            cr.rectangle(0.0, 0.0, w as f64, h as f64);
            let _ = cr.fill();
        });

        // Button -> ColorDialog async
        {
            let color = color.clone();
            let value_label = value_label.clone();
            let swatch = swatch.clone();
            let cb = Rc::clone(&callback_rc);
            button.connect_clicked(move |btn| {
                let window = btn.root().and_downcast::<gtk::Window>();
                let dialog = gtk::ColorDialog::new();
                dialog.set_title("Elegir color");
                let (r, g, b, _a) = hex_to_rgba(&color.borrow(), 1.0);
                let rgba = gdk_rgba(r, g, b);

                let color2 = color.clone();
                let value_label2 = value_label.clone();
                let swatch2 = swatch.clone();
                let cb2 = Rc::clone(&cb);

                if let Some(win) = window {
                    dialog.choose_rgba(
                        Some(&win),
                        Some(&rgba),
                        None::<&gio::Cancellable>,
                        move |result| match result {
                            Ok(picked) => {
                                let hex = rgba_to_hex(
                                    picked.red() as f64,
                                    picked.green() as f64,
                                    picked.blue() as f64,
                                    picked.alpha() as f64,
                                );
                                *color2.borrow_mut() = hex.clone();
                                value_label2.set_label(&hex);
                                swatch2.queue_draw();
                                if let Some(cb) = cb2.borrow().as_ref() {
                                    cb(&hex);
                                }
                            }
                            Err(_) => {}
                        },
                    );
                }
            });
        }

        // Entry changed: validar hex en vivo
        {
            let color = color.clone();
            let value_label = value_label.clone();
            let swatch = swatch.clone();
            entry.connect_changed(move |entry| {
                let text = entry.text().trim().to_string();
                if text.is_empty() {
                    return;
                }
                let t = if text.starts_with('#') {
                    text.clone()
                } else {
                    format!("#{text}")
                };
                if t.len() != 7 && t.len() != 9 {
                    return;
                }
                if !is_valid_hex(&t) {
                    return;
                }
                *color.borrow_mut() = t.clone();
                value_label.set_label(&t);
                swatch.queue_draw();
            });
        }

        // Entry activate: commit + callback
        {
            let color = color.clone();
            let value_label = value_label.clone();
            let swatch = swatch.clone();
            let cb = Rc::clone(&callback_rc);
            entry.connect_activate(move |entry| {
                let text = entry.text().trim().to_string();
                let t = if text.starts_with('#') {
                    text.clone()
                } else {
                    format!("#{text}")
                };
                if (t.len() != 7 && t.len() != 9) || !is_valid_hex(&t) {
                    let current = color.borrow().clone();
                    entry.set_text(&current);
                    return;
                }
                *color.borrow_mut() = t.clone();
                entry.set_text(&t);
                value_label.set_label(&t);
                swatch.queue_draw();
                if let Some(cb) = cb.borrow().as_ref() {
                    cb(&t);
                }
            });
        }

        Self {
            root,
            color,
            swatch,
            value_label,
            entry,
            callback: callback_rc,
        }
    }

    pub fn widget(&self) -> &gtk::Box {
        &self.root
    }

    pub fn get_value(&self) -> String {
        self.color.borrow().clone()
    }
}

fn gdk_rgba(r: f64, g: f64, b: f64) -> gtk::gdk::RGBA {
    gtk::gdk::RGBA::new(r as f32, g as f32, b as f32, 1.0)
}

impl crate::widgets::AsWidget for ColorPickerRow {
    fn widget(&self) -> &gtk::Widget {
        self.root.upcast_ref()
    }
}
