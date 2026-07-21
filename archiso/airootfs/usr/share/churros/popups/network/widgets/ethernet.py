import gi

gi.require_version("Gtk", "4.0")

from gi.repository import Gtk

from services.ethernet import EthernetService


class EthernetWidget(Gtk.Box):

    def __init__(self):

        super().__init__(
            orientation=Gtk.Orientation.VERTICAL,
            spacing=8
        )

        self.add_css_class("ethernet-widget")

        ethernet = EthernetService.get()

        title = Gtk.Label(
            label="󰈀 Ethernet"
        )

        title.add_css_class("section-title")
        title.set_xalign(0)

        self.append(title)

        if not ethernet["available"]:

            status = Gtk.Label(
                label="No Ethernet adapter detected"
            )

        elif ethernet["connected"]:

            status = Gtk.Label(
                label=f"Connected\n{ethernet['connection']}"
            )

        else:

            status = Gtk.Label(
                label="Disconnected"
            )

        status.add_css_class("network-info")
        status.set_xalign(0)

        self.append(status)