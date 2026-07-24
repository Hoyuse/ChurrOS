import os

import gi

gi.require_version("Gtk", "4.0")

from gi.repository import Gtk


ICON_DIR = os.path.abspath(
    os.path.join(
        os.path.dirname(__file__),
        "..",
        "assets",
        "icons"
    )
)


class Card(Gtk.Button):

    def __init__(
        self,
        icon,
        title,
        subtitle
    ):

        super().__init__()

        self.add_css_class("card")

        content = Gtk.Box(
            orientation=Gtk.Orientation.HORIZONTAL,
            spacing=18
        )

        content.set_margin_top(18)
        content.set_margin_bottom(18)
        content.set_margin_start(18)
        content.set_margin_end(18)

        self.image = Gtk.Image()

        self.image.set_from_file(
            os.path.join(
                ICON_DIR,
                icon
            )
        )

        self.image.set_pixel_size(34)

        labels = Gtk.Box(
            orientation=Gtk.Orientation.VERTICAL,
            spacing=4
        )

        labels.set_valign(Gtk.Align.CENTER)

        self.title = Gtk.Label(label=title)
        self.title.set_xalign(0)
        self.title.add_css_class("card-title")

        self.subtitle = Gtk.Label(label=subtitle)
        self.subtitle.set_xalign(0)
        self.subtitle.add_css_class("card-subtitle")

        labels.append(self.title)
        labels.append(self.subtitle)

        content.append(self.image)
        content.append(labels)

        self.set_child(content)

        self.set_hexpand(True)

        self.set_size_request(
            190,
            110
        )

    def set_state(
        self,
        subtitle=None,
        icon=None
    ):

        if subtitle is not None:

            self.subtitle.set_label(
                subtitle
            )

        if icon is not None:

            self.image.set_from_file(
                os.path.join(
                    ICON_DIR,
                    icon
                )
            )