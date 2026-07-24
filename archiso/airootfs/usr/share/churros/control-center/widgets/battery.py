from widgets.card import Card
from popup_launcher import open_battery

from services.battery import BatteryService


class BatteryCard(Card):

    def __init__(self):

        super().__init__(
            "battery.svg",
            "Battery",
            "Loading..."
        )

        self.connect(
            "clicked",
            self.on_clicked
        )

    def on_clicked(self, *_):

        open_battery(
            self.get_root()
        )

    def update(self):

        battery = BatteryService.get()

        if not battery["available"]:

            self.set_state(
                subtitle="Desktop",
                icon="battery.svg"
            )

            return

        percentage = battery["percentage"]

        icon = "battery.svg"

        if percentage <= 15:
            icon = "battery_critical.svg"

        subtitle = f"{percentage}%"

        if battery["state"] == "charging":
            subtitle += " • Charging"

        elif battery["state"] == "fully-charged":
            subtitle += " • Full"

        self.set_state(
            subtitle=subtitle,
            icon=icon
        )