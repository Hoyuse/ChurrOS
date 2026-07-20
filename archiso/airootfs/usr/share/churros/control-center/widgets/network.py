import gi

gi.require_version("Gtk", "4.0")

from widgets.card import Card
from widgets.header import Header
from widgets.label import Label

from services.network import NetworkService


class NetworkCard(Card):

    def __init__(self):

        super().__init__()

        self.build()

    def build(self):

        self.header = Header(

            NetworkService.get_icon(),

            "Network",

            NetworkService.get_status()

        )

        self.append(self.header)

        self.label = Label(

            NetworkService.get_name()

        )

        self.append(self.label)