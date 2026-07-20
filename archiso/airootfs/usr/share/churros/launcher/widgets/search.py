import gi

gi.require_version("Gtk", "4.0")

from gi.repository import Gtk


class Search(Gtk.SearchEntry):

    def __init__(self):

        super().__init__()

        self.set_placeholder_text(
            "Search applications..."
        )

        self.set_hexpand(True)

        self.add_css_class("search")