from pathlib import Path
import sys

sys.path.insert(0, str(Path(__file__).resolve().parent.parent))

import gi

gi.require_version("Gtk", "4.0")

from gi.repository import Gtk

from window import AudioWindow


class Application(Gtk.Application):

    def __init__(self):

        super().__init__(
            application_id="org.churros.popup.audio"
        )

    def do_activate(self):

        window = AudioWindow(self)

        window.present()


app = Application()

app.run()