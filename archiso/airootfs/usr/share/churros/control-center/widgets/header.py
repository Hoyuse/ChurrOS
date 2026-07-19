import gi

gi.require_version("Gtk", "4.0")

from gi.repository import Gtk


class Header(Gtk.Box):

    def __init__(self, icon, title, value=""):

        super().__init__(
            orientation=Gtk.Orientation.HORIZONTAL,
            spacing=8
        )

        image = Gtk.Image.new_from_icon_name(icon)

        image.set_pixel_size(20)

        self.append(image)

        self.title = Gtk.Label(label=title)

        self.title.set_hexpand(True)

        self.title.set_xalign(0)

        self.title.add_css_class("card-title")

        self.append(self.title)

        self.value = Gtk.Label(label=value)

        self.value.add_css_class("card-value")

        self.append(self.value)

    def set_value(self, value):

        self.value.set_label(value)