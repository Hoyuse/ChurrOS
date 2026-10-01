// ==========================================
// card.rs — tarjeta base del control center (port de widgets/card.py)
// ==========================================

use gtk::prelude::*;

use super::super::assets;

pub struct Card {
    pub button: gtk::Button,
    image: gtk::Image,
    pub subtitle: gtk::Label,
    pub detail: gtk::Label,
}

impl Card {
    pub fn new(icon: &str, title: &str, subtitle: &str) -> Self {
        let button = gtk::Button::new();
        button.add_css_class("card");

        let content = gtk::Box::new(gtk::Orientation::Horizontal, 18);
        content.set_margin_top(18);
        content.set_margin_bottom(18);
        content.set_margin_start(18);
        content.set_margin_end(18);

        let image = gtk::Image::from_file(assets::icon_path(icon));
        image.set_pixel_size(34);

        let labels = gtk::Box::new(gtk::Orientation::Vertical, 2);
        labels.set_valign(gtk::Align::Center);

        let title_label = gtk::Label::new(Some(title));
        title_label.set_xalign(0.0);
        title_label.add_css_class("card-title");

        let subtitle_label = gtk::Label::new(Some(subtitle));
        subtitle_label.set_xalign(0.0);
        subtitle_label.add_css_class("card-subtitle");
        subtitle_label.set_ellipsize(gtk::pango::EllipsizeMode::End);

        let detail_label = gtk::Label::new(None);
        detail_label.set_xalign(0.0);
        detail_label.add_css_class("card-detail");
        detail_label.set_visible(false);
        detail_label.set_ellipsize(gtk::pango::EllipsizeMode::End);

        labels.append(&title_label);
        labels.append(&subtitle_label);
        labels.append(&detail_label);

        content.append(&image);
        content.append(&labels);

        button.set_child(Some(&content));
        button.set_hexpand(true);
        button.set_size_request(190, 110);

        Self {
            button,
            image,
            subtitle: subtitle_label,
            detail: detail_label,
        }
    }

    pub fn set_state(&self, subtitle: Option<&str>, icon: Option<&str>) {
        if let Some(subtitle) = subtitle {
            self.subtitle.set_label(subtitle);
        }
        if let Some(icon) = icon {
            self.image.set_from_file(Some(assets::icon_path(icon)));
        }
    }

    pub fn set_detail(&self, detail: Option<&str>) {
        match detail {
            Some(d) if !d.is_empty() => {
                self.detail.set_label(d);
                self.detail.set_visible(true);
            }
            _ => {
                self.detail.set_visible(false);
            }
        }
    }
}