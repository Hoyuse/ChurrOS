import gi

gi.require_version("Gtk", "4.0")

from gi.repository import Gtk


class DeviceWidget(Gtk.Box):

    def __init__(self, icon, name, connected=False):

        super().__init__(
            orientation=Gtk.Orientation.HORIZONTAL,
            spacing=10
        )

        self.add_css_class("device-item")

        icon_label = Gtk.Label(label=icon)

        name_label = Gtk.Label(label=name)
        name_label.set_hexpand(True)
        name_label.set_xalign(0)

        status = Gtk.Label(
            label="Connected" if connected else ""
        )

        self.append(icon_label)
        self.append(name_label)
        self.append(status)