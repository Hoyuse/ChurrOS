import gi

gi.require_version("Gtk", "4.0")

from gi.repository import Gtk


class PowerButton(Gtk.Button):

    def __init__(self, icon, title, callback):

        super().__init__()

        self.add_css_class("power-button")

        box = Gtk.Box(
            orientation=Gtk.Orientation.HORIZONTAL,
            spacing=12
        )

        icon_label = Gtk.Label(
            label=icon
        )

        icon_label.add_css_class(
            "power-icon"
        )

        text = Gtk.Label(
            label=title
        )

        text.set_hexpand(True)

        text.set_xalign(0)

        text.add_css_class(
            "power-text"
        )

        box.append(icon_label)

        box.append(text)

        self.set_child(box)

        self.connect(
            "clicked",
            lambda *_: callback()
        )