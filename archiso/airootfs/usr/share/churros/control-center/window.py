from pathlib import Path

import gi

gi.require_version("Gtk", "4.0")

from gi.repository import Gtk, Gdk
from widgets.network import NetworkCard
from widgets.audio import AudioCard


class ControlCenter(Gtk.Application):

    def __init__(self):

        super().__init__(
            application_id="org.churros.ControlCenter"
        )

        self.connect("activate", self.on_activate)

    def on_activate(self, app):

        provider = Gtk.CssProvider()

        provider.load_from_path(
            str(Path(__file__).parent / "style.css")
        )

        Gtk.StyleContext.add_provider_for_display(
            Gdk.Display.get_default(),
            provider,
            Gtk.STYLE_PROVIDER_PRIORITY_APPLICATION,
        )

        window = Gtk.ApplicationWindow(application=app)

        window.set_title("ChurrOS Control Center")

        window.set_default_size(420, 620)

        window.set_resizable(False)

        window.set_decorated(False)

        root = Gtk.Box(
            orientation=Gtk.Orientation.VERTICAL,
            spacing=12
        )

        root.set_margin_top(16)
        root.set_margin_bottom(16)
        root.set_margin_start(16)
        root.set_margin_end(16)

        root.add_css_class("root")

        root.append(AudioCard())
        
        root.append(NetworkCard())

        window.set_child(root)

        window.present()