from pathlib import Path

import gi

gi.require_version("Gtk", "4.0")

from gi.repository import Gtk


class Header(Gtk.Box):

    def __init__(self, icon, title, value=""):

        super().__init__(
            orientation=Gtk.Orientation.HORIZONTAL,
            spacing=10
        )

        if icon.endswith(".svg"):

            base_dir = Path(__file__).parent.parent

            image = Gtk.Image.new_from_file(
                str(base_dir / "assets" / "icons" / icon)
            )

        else:

            image = Gtk.Image.new_from_icon_name(icon)

        image.set_pixel_size(24)

        self.append(image)

        texts = Gtk.Box(
            orientation=Gtk.Orientation.VERTICAL,
            spacing=2
        )

        texts.set_hexpand(True)

        title_label = Gtk.Label(
            label=title,
            xalign=0
        )

        title_label.add_css_class("card-title")

        texts.append(title_label)

        self.append(texts)

        if value:

            value_label = Gtk.Label(
                label=value,
                xalign=1
            )

            value_label.add_css_class("card-value")

            self.append(value_label)