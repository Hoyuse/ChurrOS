import gi

gi.require_version("Gtk", "4.0")

from gi.repository import Gtk


class NetworkItem(Gtk.Button):

    def __init__(self, network, callback):

        super().__init__()

        self.network = network
        self.callback = callback

        self.add_css_class(
            "network-item"
        )

        self.set_hexpand(True)

        root = Gtk.Box(
            orientation=Gtk.Orientation.VERTICAL,
            spacing=8
        )

        row = Gtk.Box(
            orientation=Gtk.Orientation.HORIZONTAL,
            spacing=10
        )

        signal = network["signal"]

        if signal >= 80:

            icon = "󰤨"

        elif signal >= 60:

            icon = "󰤥"

        elif signal >= 40:

            icon = "󰤢"

        elif signal >= 20:

            icon = "󰤟"

        else:

            icon = "󰤯"

        icon_label = Gtk.Label(
            label=icon
        )

        icon_label.add_css_class(
            "network-icon"
        )

        name = Gtk.Label(
            label=network["ssid"]
        )

        name.set_hexpand(True)

        name.set_xalign(0)

        name.add_css_class(
            "network-name"
        )

        row.append(icon_label)

        row.append(name)

        if network["security"] not in ("", "--"):

            lock = Gtk.Label(
                label="󰌾"
            )

            lock.add_css_class(
                "network-lock"
            )

            row.append(lock)

        root.append(row)

        status = Gtk.Label()

        status.set_xalign(0)

        status.add_css_class(
            "network-status"
        )

        if network["connected"]:

            status.set_label(
                "Connected"
            )

            status.add_css_class(
                "connected"
            )

        else:

            status.set_label(
                f"Signal {signal}%"
            )

        root.append(status)

        self.set_child(root)

        self.connect(
            "clicked",
            self.on_clicked
        )

    def on_clicked(self, button):

        self.callback(
            self.network
        )