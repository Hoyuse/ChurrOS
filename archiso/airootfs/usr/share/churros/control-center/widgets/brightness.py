from widgets.card import Card
from popup_launcher import open_brightness

from services.brightness import BrightnessService


class BrightnessCard(Card):

    def __init__(self):

        super().__init__(
            "brightness.svg",
            "Brightness",
            "Loading..."
        )

        self.connect(
            "clicked",
            self.on_clicked
        )

        self.update()

    def on_clicked(self, *_):

        open_brightness(
            self.get_root()
        )

    def update(self):

        brightness = BrightnessService.get()

        if not brightness["available"]:

            self.set_state(
                subtitle="Unavailable",
                icon="brightness.svg"
            )

            return

        self.set_state(
            subtitle=f'{brightness["brightness"]}%',
            icon="brightness.png"
        )