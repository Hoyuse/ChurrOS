import gi

gi.require_version("Gtk", "4.0")

from gi.repository import Gtk

from widgets.card import Card
from widgets.header import Header
from widgets.slider import Slider

from services.pipewire import PipeWireService


class AudioCard(Card):

    def __init__(self):

        super().__init__()

        self.build()

    def build(self):

        volume = PipeWireService.get_volume()

        self.header = Header(
            "audio-volume-high-symbolic",
            "Audio",
            f"{volume}%"
        )

        self.append(self.header)

        self.slider = Slider(volume)

        self.slider.connect(
            "value-changed",
            self.on_volume_changed
        )

        self.append(self.slider)

    def on_volume_changed(self, slider):

        volume = int(slider.get_value())

        self.header.set_value(f"{volume}%")

        PipeWireService.set_volume(volume)