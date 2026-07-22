import gi

gi.require_version("Gtk", "4.0")

from gi.repository import Gtk


class Switch(Gtk.Switch):

    def __init__(self, state=False):

        super().__init__()

        self.set_active(state)