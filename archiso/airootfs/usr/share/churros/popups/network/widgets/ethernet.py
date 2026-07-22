import gi

gi.require_version("Gtk", "4.0")

from gi.repository import Gtk, GLib

from services.ethernet import EthernetService


class EthernetWidget(Gtk.Box):

    def __init__(self):

        super().__init__(
            orientation=Gtk.Orientation.VERTICAL,
            spacing=10
        )

        self.add_css_class(
            "ethernet-widget"
        )

        self.last = None

        self.reload()

        GLib.timeout_add_seconds(
            3,
            self.refresh
        )

    def refresh(self):

        current = EthernetService.get()

        if current != self.last:

            self.reload()

        return True

    def reload(self):

        self.last = EthernetService.get()

        while True:

            child = self.get_first_child()

            if child is None:
                break

            self.remove(child)

        data = self.last

        title = Gtk.Label(
            label="󰈀 Ethernet"
        )

        title.set_xalign(0)

        title.add_css_class(
            "section-title"
        )

        self.append(title)

        if not data["available"]:

            label = Gtk.Label(
                label="No ethernet adapter detected."
            )

            label.set_xalign(0)

            label.add_css_class(
                "network-info"
            )

            self.append(label)

            return

        status = Gtk.Label()

        status.set_xalign(0)

        if data["connected"]:

            status.set_label(
                "Connected"
            )

            status.add_css_class(
                "connected"
            )

        else:

            status.set_label(
                "Cable disconnected"
            )

            status.add_css_class(
                "network-info"
            )

        self.append(status)

        if data["connected"]:

            speed = Gtk.Label(
                label=f"󰓅 {data['speed']} Mbps"
            )

            speed.set_xalign(0)

            speed.add_css_class(
                "network-info"
            )

            self.append(speed)

            if data["ip"]:

                ip = Gtk.Label(
                    label=f"󰩠 {data['ip']}"
                )

                ip.set_xalign(0)

                ip.add_css_class(
                    "network-info"
                )

                self.append(ip)

        button = Gtk.Button()

        if data["connected"]:

            button.set_label(
                "Disconnect"
            )

            button.connect(
                "clicked",
                self.disconnect
            )

        else:

            button.set_label(
                "Connect"
            )

            button.connect(
                "clicked",
                self.connect
            )

        button.add_css_class(
            "network-button"
        )

        self.append(button)

    def connect(self, button):

        EthernetService.connect()

        self.reload()

    def disconnect(self, button):

        EthernetService.disconnect()

        self.reload()