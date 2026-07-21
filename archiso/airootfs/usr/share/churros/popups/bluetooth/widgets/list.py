import gi

gi.require_version("Gtk", "4.0")

from gi.repository import Gtk

from widgets.device import DeviceWidget


class DeviceListWidget(Gtk.Box):

    def __init__(self):

        super().__init__(
            orientation=Gtk.Orientation.VERTICAL,
            spacing=6
        )

        self.add_css_class("device-list")

        connected = Gtk.Label(label="Connected")
        connected.add_css_class("section-title")
        connected.set_xalign(0)

        self.append(connected)

        self.append(
            DeviceWidget(
                "🎧",
                "AirPods Pro",
                connected=True
            )
        )

        available = Gtk.Label(label="Available")
        available.add_css_class("section-title")
        available.set_xalign(0)

        self.append(available)

        self.append(
            DeviceWidget(
                "⌨",
                "Keychron K2"
            )
        )

        self.append(
            DeviceWidget(
                "🖱",
                "MX Master 3"
            )
        )

        self.append(
            DeviceWidget(
                "🎮",
                "DualSense"
            )
        )