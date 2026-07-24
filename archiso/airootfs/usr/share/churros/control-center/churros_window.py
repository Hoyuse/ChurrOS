import os
import gi

gi.require_version("Gtk", "4.0")

from gi.repository import Gtk, Gdk


class ChurrOSWindow(Gtk.ApplicationWindow):

    def __init__(self, app, title, width, height):
        super().__init__(application=app)

        self.set_title(title)
        self.set_default_size(width, height)
        self.set_resizable(False)

        self.load_css()

        controller = Gtk.EventControllerKey()
        controller.connect(
            "key-pressed",
            self.on_key_pressed
        )

        self.add_controller(controller)

    def load_css(self):

        provider = Gtk.CssProvider()

        provider.load_from_path(
            os.path.join(
                os.path.dirname(__file__),
                "style.css"
            )
        )

        Gtk.StyleContext.add_provider_for_display(
            self.get_display(),
            provider,
            Gtk.STYLE_PROVIDER_PRIORITY_APPLICATION
        )

    def on_key_pressed(
        self,
        controller,
        keyval,
        keycode,
        state
    ):

        if keyval == Gdk.KEY_Escape:

            self.close()

            return True

        return False