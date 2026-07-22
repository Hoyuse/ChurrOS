from pathlib import Path
import sys

sys.path.insert(0, str(Path(__file__).resolve().parent.parent))

import gi

gi.require_version("Gtk", "4.0")

from gi.repository import Gtk, Gdk

from common.popup import PopupWindow

from widgets.toggle import BluetoothToggleWidget
from widgets.list import DeviceListWidget


class BluetoothWindow(PopupWindow):

    def __init__(self, app):

        super().__init__(
            app,
            title="Bluetooth",
            icon="󰂯"
        )

        self.load_bluetooth_css()

        self.add(
            BluetoothToggleWidget()
        )

        self.add(
            DeviceListWidget()
        )

    def load_bluetooth_css(self):

        provider = Gtk.CssProvider()

        provider.load_from_path(
            str(Path(__file__).parent / "style.css")
        )

        Gtk.StyleContext.add_provider_for_display(
            Gdk.Display.get_default(),
            provider,
            Gtk.STYLE_PROVIDER_PRIORITY_APPLICATION
        )