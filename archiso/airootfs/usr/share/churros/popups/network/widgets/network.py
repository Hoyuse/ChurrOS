import gi

gi.require_version("Gtk", "4.0")

from gi.repository import Gtk

from widgets.toggle import NetworkToggleWidget
from widgets.wifi import WifiWidget
from widgets.ethernet import EthernetWidget


class NetworkWidget(Gtk.Box):

    def __init__(self):

        super().__init__(
            orientation=Gtk.Orientation.VERTICAL,
            spacing=18
        )

        self.add_css_class(
            "network-widget"
        )

        title = Gtk.Label(
            label="󰤨 Network"
        )

        title.set_xalign(0)

        title.add_css_class(
            "popup-title"
        )

        self.append(title)

        self.append(
            NetworkToggleWidget()
        )

        separator = Gtk.Separator(
            orientation=Gtk.Orientation.HORIZONTAL
        )

        separator.add_css_class(
            "network-separator"
        )

        self.append(separator)

        self.append(
            WifiWidget()
        )

        separator = Gtk.Separator(
            orientation=Gtk.Orientation.HORIZONTAL
        )

        separator.add_css_class(
            "network-separator"
        )

        self.append(separator)

        self.append(
            EthernetWidget()
        )