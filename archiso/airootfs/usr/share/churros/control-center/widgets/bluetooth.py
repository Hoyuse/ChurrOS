import gi

gi.require_version("Gtk", "4.0")

from widgets.card import Card
from widgets.header import Header
from widgets.label import Label

from services.bluetooth import BluetoothService


class BluetoothCard(Card):

    def __init__(self):

        super().__init__()

        self.build()

    def build(self):

        self.header = Header(

            BluetoothService.get_icon(),

            "Bluetooth",

            BluetoothService.get_status()

        )

        self.append(self.header)

        self.label = Label(

            BluetoothService.get_description()

        )

        self.append(self.label)