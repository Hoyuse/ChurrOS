from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT))

import gi

gi.require_version("Gtk", "4.0")

from gi.repository import Gtk

from services.audio import AudioService


class VolumeWidget(Gtk.Box):

    def __init__(self):

        super().__init__(
            orientation=Gtk.Orientation.VERTICAL,
            spacing=12
        )

        self.add_css_class("volume-widget")

        self.label = Gtk.Label()
        self.label.add_css_class("volume-label")

        self.slider = Gtk.Scale.new_with_range(
            Gtk.Orientation.HORIZONTAL,
            0,
            100,
            1
        )

        self.slider.set_draw_value(False)
        self.slider.set_hexpand(True)
        self.slider.set_digits(0)
        self.slider.set_round_digits(0)

        try:
            volume = AudioService.get_volume()

        except Exception:
            volume = 50

        self.slider.set_value(volume)

        self.label.set_label(
            f"󰕾 {volume}%"
        )

        self.append(self.label)
        self.append(self.slider)

        self.slider.connect(
            "value-changed",
            self.on_change
        )

    def on_change(self, slider):

        value = int(slider.get_value())

        

        self.label.set_label(
            f"󰕾 {value}%"
        )

        try:
            AudioService.set_volume(value)

        except Exception as error:

            print(error)