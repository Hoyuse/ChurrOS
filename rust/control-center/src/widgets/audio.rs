// ==========================================
// audio.rs — tarjeta de audio (port de widgets/audio.py)
// ==========================================

use std::cell::Cell;
use std::rc::Rc;
use std::time::{Duration, Instant};

use gtk::prelude::*;

use churros_services::audio;

use super::super::assets;
use crate::widgets::SystemInfo;

pub struct AudioCard {
    box_: gtk::Box,
    icon: gtk::Image,
    subtitle: gtk::Label,
    scale: gtk::Scale,
    last_user_time: Rc<Cell<Instant>>,
}

impl AudioCard {
    pub fn new() -> Self {
        let box_ = gtk::Box::new(gtk::Orientation::Vertical, 12);
        box_.add_css_class("card");
        box_.set_margin_top(18);
        box_.set_margin_bottom(18);
        box_.set_margin_start(18);
        box_.set_margin_end(18);

        let header = gtk::Box::new(gtk::Orientation::Horizontal, 12);

        let icon = gtk::Image::from_file(assets::icon_path("audio.svg"));
        icon.set_pixel_size(28);

        let labels = gtk::Box::new(gtk::Orientation::Vertical, 0);

        let title = gtk::Label::new(Some("Audio"));
        title.add_css_class("card-title");
        title.set_xalign(0.0);

        let subtitle = gtk::Label::new(None::<&str>);
        subtitle.add_css_class("card-subtitle");
        subtitle.set_xalign(0.0);

        labels.append(&title);
        labels.append(&subtitle);

        header.append(&icon);
        header.append(&labels);

        let scale = gtk::Scale::with_range(gtk::Orientation::Horizontal, 0.0, 100.0, 1.0);
        scale.set_hexpand(true);
        scale.set_draw_value(false);

        let last_user_time = Rc::new(Cell::new(Instant::now() - Duration::from_secs(60)));
        let last_sent_val = Rc::new(Cell::new(255u8));

        let icon_clone = icon.clone();
        let subtitle_clone = subtitle.clone();
        let user_time_clone = last_user_time.clone();
        let sent_clone = last_sent_val.clone();

        scale.connect_value_changed(move |s| {
            let val = s.value().round() as u8;
            user_time_clone.set(Instant::now());

            // Respuesta visual instantánea (latencia 0)
            subtitle_clone.set_label(&format!("{val}%"));
            let icon_file = if val == 0 {
                "audio_muted.svg"
            } else {
                "audio.svg"
            };
            icon_clone.set_from_file(Some(assets::icon_path(icon_file)));

            if sent_clone.get() != val {
                sent_clone.set(val);
                audio::set_volume(val);
            }
        });

        // Click en el icono para alternar mute
        let icon_for_click = icon.clone();
        let click_gesture = gtk::GestureClick::new();
        click_gesture.connect_pressed(move |_, _, _, _| {
            let is_muted = audio::is_muted();
            audio::set_mute(!is_muted);
            let icon_file = if !is_muted {
                "audio_muted.svg"
            } else {
                "audio.svg"
            };
            icon_for_click.set_from_file(Some(assets::icon_path(icon_file)));
        });
        icon.add_controller(click_gesture);

        box_.append(&header);
        box_.append(&scale);

        Self {
            box_,
            icon,
            subtitle,
            scale,
            last_user_time,
        }
    }

    pub fn box_(&self) -> &gtk::Box {
        &self.box_
    }

    pub fn apply_info(&self, info: &SystemInfo) {
        // Si el usuario movió el slider recientemente o lo tiene activo, no sobreescribir
        if self.last_user_time.get().elapsed() < Duration::from_millis(1500)
            || self.scale.state_flags().contains(gtk::StateFlags::ACTIVE)
        {
            return;
        }

        self.scale.set_value(info.volume as f64);
        self.subtitle.set_label(&format!("{}%", info.volume));

        let icon = if info.muted || info.volume == 0 {
            "audio_muted.svg"
        } else {
            "audio.svg"
        };

        self.icon.set_from_file(Some(assets::icon_path(icon)));
    }
}