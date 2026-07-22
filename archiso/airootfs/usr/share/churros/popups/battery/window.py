from pathlib import Path
import sys

sys.path.insert(0, str(Path(__file__).resolve().parent.parent))

import gi

gi.require_version("Gtk", "4.0")

from gi.repository import Gtk, Gdk

from common.popup import PopupWindow

from widgets.battery import BatteryWidget


class BatteryWindow(PopupWindow):

    def __init__(self, app):

        super().__init__(
            app,
            title="Battery",
            icon="󰁹"
        )

        self.load_battery_css()

        self.add(
            BatteryWidget()
        )

    def load_battery_css(self):

        provider = Gtk.CssProvider()

        provider.load_from_path(
            str(Path(__file__).parent / "style.css")
        )

        Gtk.StyleContext.add_provider_for_display(
            Gdk.Display.get_default(),
            provider,
            Gtk.STYLE_PROVIDER_PRIORITY_APPLICATION
        )