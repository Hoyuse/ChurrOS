import os

import gi

gi.require_version("Gtk", "4.0")

from gi.repository import Gtk

from services.audio import AudioService


ICON_DIR = os.path.abspath(
    os.path.join(
        os.path.dirname(__file__),
        "..",
        "assets",
        "icons"
    )
)


class AudioCard(Gtk.Box):

    def __init__(self):

        super().__init__(
            orientation=Gtk.Orientation.VERTICAL,
            spacing=12
        )

        self.add_css_class("card")

        self.set_margin_top(18)
        self.set_margin_bottom(18)
        self.set_margin_start(18)
        self.set_margin_end(18)

        header = Gtk.Box(
            orientation=Gtk.Orientation.HORIZONTAL,
            spacing=12
        )

        self.icon = Gtk.Image()

        self.icon.set_from_file(
            os.path.join(
                ICON_DIR,
                "audio.svg"
            )
        )

        self.icon.set_pixel_size(28)

        labels = Gtk.Box(
            orientation=Gtk.Orientation.VERTICAL
        )

        title = Gtk.Label(
            label="Audio"
        )

        title.add_css_class("card-title")
        title.set_xalign(0)

        self.subtitle = Gtk.Label()

        self.subtitle.add_css_class(
            "card-subtitle"
        )

        self.subtitle.set_xalign(0)

        labels.append(title)
        labels.append(self.subtitle)

        header.append(self.icon)
        header.append(labels)

        self.append(header)

        self.scale = Gtk.Scale.new_with_range(
            Gtk.Orientation.HORIZONTAL,
            0,
            100,
            1
        )

        self.scale.set_hexpand(True)

        self.scale.connect(
            "value-changed",
            self.on_changed
        )

        self.append(self.scale)

    def on_changed(self, scale):

        AudioService.set_volume(
            int(scale.get_value())
        )

    def update(self):

        volume = AudioService.get_volume()

        self.scale.set_value(volume)

        self.subtitle.set_label(
            f"{volume}%"
        )

        if volume == 0:

            self.icon.set_from_file(
                os.path.join(
                    ICON_DIR,
                    "audio_muted.svg"
                )
            )

        else:

            self.icon.set_from_file(
                os.path.join(
                    ICON_DIR,
                    "audio.svg"
                )
            )