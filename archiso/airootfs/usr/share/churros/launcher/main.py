import gi

gi.require_version("Gtk", "4.0")

from gi.repository import Gtk, Gdk

from window import LauncherWindow


class Launcher(Gtk.Application):

    def __init__(self):

        super().__init__(
            application_id="org.churros.launcher"
        )

    def do_activate(self):

        window = LauncherWindow(self)

        window.present()


provider = Gtk.CssProvider()

provider.load_from_path(
    "/usr/share/churros/launcher/assets/style.css"
)
Gtk.StyleContext.add_provider_for_display(
    Gdk.Display.get_default(),
    provider,
    Gtk.STYLE_PROVIDER_PRIORITY_APPLICATION
)

app = Launcher()

app.run()