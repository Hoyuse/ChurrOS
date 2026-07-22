import gi

gi.require_version("Gtk", "4.0")

from gi.repository import GLib

from widgets.card import Card
from widgets.header import Header
from widgets.label import Label

from services.network import NetworkService


class NetworkCard(Card):

    def __init__(self):

        super().__init__()

        self.build()

        GLib.timeout_add_seconds(
            3,
            self.refresh
        )

    def build(self):

        self.network = NetworkService.get()

        self.header = Header(

            self.network["icon"],

            "Network",

            self.network["status"]

        )

        self.append(self.header)

        self.label = Label(self.get_label())

        self.append(self.label)

    def get_label(self):

        if self.network["type"] == "wifi":

            return (
                f"{self.network['name']} · "
                f"Signal {self.network['signal']}%"
            )

        return self.network["name"]

    def refresh(self):

        network = NetworkService.get()

        if network != self.network:

            self.network = network

            self.header.set_icon(
                self.network["icon"]
            )

            self.header.set_value(
                self.network["status"]
            )

            self.label.set_label(
                self.get_label()
            )

        return True
