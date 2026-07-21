import gi

gi.require_version("Gtk", "4.0")

from gi.repository import Gtk


class DeviceWidget(Gtk.Label):

    def __init__(self):

        super().__init__()

        self.set_label("Speakers")

        self.add_css_class("device-label")