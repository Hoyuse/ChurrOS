from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parents[2]

sys.path.insert(
    0,
    str(ROOT)
)

import gi

gi.require_version("Gtk", "4.0")

from gi.repository import Gtk

from window import BatteryWindow


class BatteryApplication(Gtk.Application):

    def do_activate(self):

        window = BatteryWindow(self)
        window.present()


app = BatteryApplication()

app.run(None)