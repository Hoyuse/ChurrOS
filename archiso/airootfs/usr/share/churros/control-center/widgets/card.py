import gi

gi.require_version("Gtk", "4.0")

from gi.repository import Gtk


class Card(Gtk.Box):

    def __init__(self):

        super().__init__(
            orientation=Gtk.Orientation.VERTICAL,
            spacing=12
        )

        self.add_css_class("card")