import gi

gi.require_version("Gtk", "4.0")

from gi.repository import Gtk


class Label(Gtk.Label):

    def __init__(self, text, css="card-subtitle"):

        super().__init__(label=text)

        self.set_xalign(0)

        self.set_wrap(True)

        self.add_css_class(css)