from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parents[2]
POPUPS = ROOT / "popups"

sys.path.insert(0, str(ROOT))
sys.path.insert(0, str(POPUPS))

import gi

gi.require_version("Gtk", "4.0")

from gi.repository import Gtk

from window import PowerWindow


class PowerApplication(Gtk.Application):

    def __init__(self):

        super().__init__(
            application_id="com.churros.power"
        )

    def do_activate(self):

        window = PowerWindow(self)

        window.present()


app = PowerApplication()

app.run()