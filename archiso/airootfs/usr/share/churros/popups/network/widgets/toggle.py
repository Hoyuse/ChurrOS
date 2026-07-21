import gi

gi.require_version("Gtk", "4.0")

from gi.repository import Gtk

from services.wifi import WifiService


class NetworkToggleWidget(Gtk.Box):

    def __init__(self):

        super().__init__(
            orientation=Gtk.Orientation.HORIZONTAL,
            spacing=12
        )

        self.add_css_class("network-toggle")

        wifi = WifiService.get()

        label = Gtk.Label(
            label="Wi-Fi"
        )

        label.add_css_class("network-label")

        label.set_hexpand(True)
        label.set_xalign(0)

        self.switch = Gtk.Switch()

        if wifi["available"]:

            self.switch.set_active(
                wifi["enabled"]
            )

        else:

            self.switch.set_sensitive(False)

        self.switch.connect(
            "state-set",
            self.on_toggle
        )

        self.append(label)
        self.append(self.switch)

    def on_toggle(self, switch, state):

        if state:

            WifiService.enable()

        else:

            WifiService.disable()

        return False