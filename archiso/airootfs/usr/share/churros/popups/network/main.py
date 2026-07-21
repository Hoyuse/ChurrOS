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

    def do_activate(self):

        window = NetworkWindow(self)
        window.present()


app = NetworkApplication()

app.run(None)