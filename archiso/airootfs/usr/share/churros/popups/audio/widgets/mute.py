import gi

gi.require_version("Gtk", "4.0")

from gi.repository import Gtk

import subprocess


class MuteWidget(Gtk.Button):

    def __init__(self):

        super().__init__()

        self.set_label("󰝟 Silenciar")

        self.add_css_class("mute-button")

        self.connect("clicked", self.toggle)

    def toggle(self, button):

        subprocess.run(
            [
                "wpctl",
                "set-mute",
                "@DEFAULT_AUDIO_SINK@",
                "toggle"
            ]
        )