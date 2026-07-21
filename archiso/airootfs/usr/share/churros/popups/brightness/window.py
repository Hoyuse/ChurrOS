from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parents[2]
POPUPS = ROOT / "popups"

sys.path.insert(0, str(ROOT))
sys.path.insert(0, str(POPUPS))

import gi

gi.require_version("Gtk", "4.0")

from gi.repository import Gtk, Gdk

from common.popup import PopupWindow

from widgets.brightness import BrightnessWidget


class BrightnessWindow(PopupWindow):

    def __init__(self, app):

        super().__init__(
            app,
            title="Brightness",
            icon="󰃠"
        )

        self.load_brightness_css()

        self.add(
            BrightnessWidget()
        )

    def load_brightness_css(self):

        provider = Gtk.CssProvider()

        provider.load_from_path(
            str(Path(__file__).parent / "style.css")
        )

        Gtk.StyleContext.add_provider_for_display(
            Gdk.Display.get_default(),
            provider,
            Gtk.STYLE_PROVIDER_PRIORITY_APPLICATION
        )