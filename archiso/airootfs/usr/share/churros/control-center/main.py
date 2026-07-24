import os
import sys
import gi

gi.require_version("Gtk", "4.0")

from gi.repository import Gtk, Gdk

ROOT = os.path.abspath(
    os.path.join(
        os.path.dirname(__file__),
        ".."
    )
)

if ROOT not in sys.path:
    sys.path.insert(0, ROOT)

from window import ControlCenterWindow


class ControlCenterApp(Gtk.Application):

    def __init__(self):

        super().__init__(
            application_id="org.churros.controlcenter"
        )

    def do_activate(self):

        css = Gtk.CssProvider()

        css.load_from_path(
            os.path.join(
                os.path.dirname(__file__),
                "style.css"
            )
        )

        Gtk.StyleContext.add_provider_for_display(
            Gdk.Display.get_default(),
            css,
            Gtk.STYLE_PROVIDER_PRIORITY_APPLICATION
        )

        window = ControlCenterWindow(self)

        window.present()


app = ControlCenterApp()

app.run()