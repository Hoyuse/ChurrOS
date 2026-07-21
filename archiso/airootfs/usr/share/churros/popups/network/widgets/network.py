import gi

gi.require_version("Gtk", "4.0")

from gi.repository import Gtk

from widgets.ethernet import EthernetWidget
from widgets.wifi import WifiWidget


class NetworkWidget(Gtk.Box):

    def __init__(self):

        super().__init__(
            orientation=Gtk.Orientation.VERTICAL,
            spacing=16
        )

        self.add_css_class("network-widget")

        self.append(
            EthernetWidget()
        )

        self.append(
            Gtk.Separator(
                orientation=Gtk.Orientation.HORIZONTAL
            )
        )

        self.append(
            WifiWidget()
        )