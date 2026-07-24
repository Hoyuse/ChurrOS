from widgets.card import Card
from popup_launcher import open_network

from services.wifi import WifiService
from services.ethernet import EthernetService


class NetworkCard(Card):

    def __init__(self):

        super().__init__(
            "wifi.svg",
            "Network",
            "Loading..."
        )

        self.connect(
            "clicked",
            self.on_clicked
        )

    def on_clicked(self, *_):

        open_network(
            self.get_root()
        )

    def update(self):

        ethernet = EthernetService.get()

        if ethernet["available"] and ethernet["connected"]:

            subtitle = "Ethernet"

            if ethernet["speed"]:

                subtitle += f" • {ethernet['speed']} Mbps"

            self.set_state(
                subtitle=subtitle,
                icon="ethernet.svg"
            )

            return

        wifi = WifiService.get()

        if not wifi["available"]:

            self.set_state(
                subtitle="Unavailable",
                icon="wifi.svg"
            )

            return

        if not wifi["enabled"]:

            self.set_state(
                subtitle="Disabled",
                icon="wifi.svg"
            )

            return

        if wifi["connected"]:

            self.set_state(
                subtitle=wifi["connected"],
                icon="wifi.svg"
            )

        else:

            self.set_state(
                subtitle="Disconnected",
                icon="wifi.svg"
            )