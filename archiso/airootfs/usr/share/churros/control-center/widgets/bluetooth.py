from widgets.card import Card
from popup_launcher import open_bluetooth


class BluetoothCard(Card):

    def __init__(self):

        super().__init__(
            "bluetooth.svg",
            "Bluetooth",
            "Unavailable"
        )

        self.connect(
            "clicked",
            self.on_clicked
        )

    def on_clicked(self, *_):

        open_bluetooth(
            self.get_root()
        )

    def update(self):

        self.set_state(
            subtitle="Unavailable",
            icon="bluetooth_disabled.svg"
        )