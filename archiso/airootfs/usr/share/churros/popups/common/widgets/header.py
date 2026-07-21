import gi

gi.require_version("Gtk", "4.0")

from gi.repository import Gtk


class Header(Gtk.Box):

    def __init__(self, icon, title):

        super().__init__(
            orientation=Gtk.Orientation.HORIZONTAL
        )

        self.add_css_class("popup-header")

        self.set_margin_bottom(12)

        icon_label = Gtk.Label(label=icon)
        icon_label.add_css_class("popup-header-icon")

        title_label = Gtk.Label(label=title)
        title_label.add_css_class("popup-header-title")

        title_label.set_hexpand(True)
        title_label.set_halign(Gtk.Align.START)

        self.append(icon_label)
        self.append(title_label)