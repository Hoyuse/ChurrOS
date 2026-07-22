import gi

gi.require_version("Gtk", "4.0")

from gi.repository import Gtk

from services.wifi import WifiService


class PasswordDialog(Gtk.Box):

    def __init__(self, network, on_success):

        super().__init__(
            orientation=Gtk.Orientation.VERTICAL,
            spacing=12
        )

        self.network = network
        self.on_success = on_success

        self.add_css_class(
            "password-dialog"
        )

        title = Gtk.Label(
            label=f"Connect to\n{network['ssid']}"
        )

        title.add_css_class(
            "section-title"
        )

        title.set_xalign(0)

        self.append(title)

        self.entry = Gtk.Entry()

        self.entry.set_placeholder_text(
            "Password"
        )

        self.entry.set_visibility(False)

        self.append(self.entry)

        self.error = Gtk.Label()

        self.error.add_css_class(
            "network-error"
        )

        self.error.set_xalign(0)

        self.append(self.error)

        buttons = Gtk.Box(
            spacing=8
        )

        connect = Gtk.Button(
            label="Connect"
        )

        connect.add_css_class(
            "suggested-action"
        )

        connect.connect(
            "clicked",
            self.connect_network
        )

        buttons.append(connect)

        self.append(buttons)

    def connect_network(self, button):

        success, message = WifiService.connect(
            self.network["ssid"],
            self.entry.get_text()
        )

        if success:

            self.on_success()

            return

        self.error.set_label(
            message
        )