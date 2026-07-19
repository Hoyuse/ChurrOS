import gi

gi.require_version("Gtk", "4.0")

from widgets.card import Card
from widgets.header import Header
from widgets.label import Label

from services.battery import BatteryService


class BatteryCard(Card):

    def __init__(self):

        super().__init__()

        if BatteryService.has_battery():

            value = BatteryService.get_percentage()

            subtitle = BatteryService.get_state()

        else:

            value = "--"

            subtitle = "Desktop PC"

        self.header = Header(

            BatteryService.get_icon(),

            "Battery",

            value

        )

        self.append(self.header)

        self.append(

            Label(subtitle)

        )