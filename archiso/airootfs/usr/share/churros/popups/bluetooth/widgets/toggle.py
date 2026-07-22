import gi

gi.require_version("Gtk", "4.0")

from gi.repository import Gtk


class BluetoothToggleWidget(Gtk.Box):

    def __init__(self):

        super().__init__(
            orientation=Gtk.Orientation.HORIZONTAL,
            spacing=12
        )

        self.add_css_class("bluetooth-toggle")

        label = Gtk.Label(label="Bluetooth")
        label.set_hexpand(True)
        label.set_xalign(0)

        switch = Gtk.Switch()
        switch.set_active(True)

        self.append(label)
        self.append(switch)