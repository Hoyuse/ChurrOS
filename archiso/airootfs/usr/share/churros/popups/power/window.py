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

from widgets.power import PowerWidget


class PowerWindow(PopupWindow):

    def __init__(self, app):

        super().__init__(
            app,
            title="Power",
            icon="󰐥"
        )

        self.load_power_css()

        self.add(
            PowerWidget()
        )

    def load_power_css(self):

        provider = Gtk.CssProvider()

        provider.load_from_path(
            str(
                Path(__file__).parent / "style.css"
            )
        )

        Gtk.StyleContext.add_provider_for_display(
            Gdk.Display.get_default(),
            provider,
            Gtk.STYLE_PROVIDER_PRIORITY_APPLICATION
        )