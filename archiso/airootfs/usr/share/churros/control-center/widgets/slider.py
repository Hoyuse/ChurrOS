import gi

gi.require_version("Gtk", "4.0")

from gi.repository import Gtk


class Slider(Gtk.Scale):

    def __init__(self, value=50):

        super().__init__(
            orientation=Gtk.Orientation.HORIZONTAL
        )

        self.set_range(0, 100)

        self.set_value(value)

        self.set_draw_value(False)

        self.set_hexpand(True)