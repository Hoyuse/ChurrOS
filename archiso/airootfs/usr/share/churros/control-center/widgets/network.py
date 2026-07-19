import gi

gi.require_version("Gtk", "4.0")

from widgets.card import Card
from widgets.header import Header
from widgets.label import Label

from services.network import NetworkService


class NetworkCard(Card):

    def __init__(self):

        super().__init__()

        connected = NetworkService.is_connected()

        status = "Connected" if connected else "Offline"

        name = NetworkService.get_name()

        self.header = Header(

            "network-wired-symbolic" if name == "Ethernet"
            else "network-wireless-symbolic",

            "Network",

            status

        )

        self.append(self.header)

        self.append(

            Label(name)

        )