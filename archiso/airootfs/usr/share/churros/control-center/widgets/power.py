import os
import gi

gi.require_version("Gtk", "4.0")

from gi.repository import Gtk

from popup_launcher import open_power


class PowerButton(Gtk.Button):

    def __init__(self):

        super().__init__()

        self.add_css_class(
            "power-button"
        )

        icon = Gtk.Image.new_from_file(

            os.path.join(

                os.path.dirname(__file__),

                "..",

                "assets",

                "icons",

                "powerbutton.svg"
            )
        )

        icon.set_pixel_size(22)

        self.set_child(icon)

        self.connect(
            "clicked",
            self.on_clicked
        )

    def on_clicked(self, *_):

        open_power(
            self.get_root()
        )