import gi

gi.require_version("Gtk", "4.0")

from gi.repository import Gtk

from services.brightness import BrightnessService


class BrightnessWidget(Gtk.Box):

    def __init__(self):

        super().__init__(
            orientation=Gtk.Orientation.VERTICAL,
            spacing=12
        )

        self.add_css_class(
            "brightness-widget"
        )

        data = BrightnessService.get()

        self.label = Gtk.Label()

        self.label.add_css_class(
            "brightness-label"
        )

        self.append(
            self.label
        )

        self.slider = Gtk.Scale.new_with_range(
            Gtk.Orientation.HORIZONTAL,
            0,
            100,
            1
        )

        self.slider.set_draw_value(False)

        self.slider.add_css_class(
            "brightness-slider"
        )

        self.slider.set_hexpand(True)

        if data["available"]:

            self.slider.set_value(
                data["brightness"]
            )

            self.label.set_label(
                f"󰃠 {data['brightness']}%"
            )

            self.slider.connect(
                "value-changed",
                self.on_change
            )

        else:

            self.slider.set_value(100)

            self.slider.set_sensitive(False)

            self.label.set_label(
                "󰃠 Brightness unavailable"
            )

            info = Gtk.Label(
                label="This display does not support software brightness control."
            )

            info.set_wrap(True)

            info.set_xalign(0)

            info.add_css_class(
                "brightness-info"
            )

            self.append(
                info
            )

        self.append(
            self.slider
        )

    def on_change(self, slider):

        value = int(
            slider.get_value()
        )

        BrightnessService.set(
            value
        )

        self.label.set_label(
            f"󰃠 {value}%"
        )