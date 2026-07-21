import gi

gi.require_version("Gtk", "4.0")

from gi.repository import Gtk

from services.wifi import WifiService


class WifiWidget(Gtk.Box):

    def __init__(self):

        super().__init__(
            orientation=Gtk.Orientation.VERTICAL,
            spacing=8
        )

        self.add_css_class("wifi-widget")

        wifi = WifiService.get()

        title = Gtk.Label(
            label="󰤨 Wi-Fi"
        )

        title.add_css_class("section-title")
        title.set_xalign(0)

        self.append(title)

        if not wifi["available"]:

            label = Gtk.Label(
                label="No Wi-Fi adapter detected"
            )

            label.add_css_class("network-info")
            label.set_xalign(0)

            self.append(label)

            return

        if not wifi["enabled"]:

            label = Gtk.Label(
                label="Wi-Fi disabled"
            )

            label.add_css_class("network-info")
            label.set_xalign(0)

            self.append(label)

            return

        if wifi["connected"]:

            connected = Gtk.Label(
                label=f"Connected\n{wifi['connected']}"
            )

            connected.add_css_class("network-info")
            connected.set_xalign(0)

            self.append(connected)

        for network in wifi["networks"]:

            if network["connected"]:
                continue

            row = Gtk.Box(
                orientation=Gtk.Orientation.HORIZONTAL,
                spacing=8
            )

            icon = Gtk.Label(
                label="󰤨"
            )

            name = Gtk.Label(
                label=network["ssid"]
            )

            signal = Gtk.Label(
                label=f"{network['signal']}%"
            )

            name.add_css_class("network-name")
            signal.add_css_class("network-info")

            name.set_hexpand(True)
            name.set_xalign(0)

            signal.set_xalign(1)

            row.append(icon)
            row.append(name)
            row.append(signal)

            self.append(row)