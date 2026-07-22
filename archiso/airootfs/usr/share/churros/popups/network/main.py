from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parents[2]
POPUPS = ROOT / "popups"

sys.path.insert(0, str(ROOT))
sys.path.insert(0, str(POPUPS))

import gi

gi.require_version("Gtk", "4.0")

from gi.repository import Gtk

from window import NetworkWindow


class NetworkApplication(Gtk.Application):

    def __init__(self):

        super().__init__(
            application_id="org.churros.popup.network"
        )

        self.window = None

    def do_activate(self):

        if self.window is None:

            self.window = NetworkWindow(self)

        self.window.present()


app = NetworkApplication()

app.run(None)
