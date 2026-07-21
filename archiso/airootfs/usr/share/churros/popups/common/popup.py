from pathlib import Path
import gi

gi.require_version("Gtk", "4.0")

from gi.repository import Gtk, Gdk

from .widgets.header import Header


class PopupWindow(Gtk.ApplicationWindow):

    def __init__(self, app, title="Popup", icon="🧪"):

        super().__init__(application=app)

        self.set_title(title)
        self.set_default_size(320, 400)
        self.set_resizable(False)
        self.set_decorated(False)

        self.add_css_class("popup")

        self.load_css()

        self.main_box = Gtk.Box(
            orientation=Gtk.Orientation.VERTICAL
        )

        self.main_box.add_css_class("popup-content")

        self.set_child(self.main_box)

        self.header = Header(icon, title)

        self.main_box.append(self.header)

        self.content = Gtk.Box(
            orientation=Gtk.Orientation.VERTICAL
        )

        self.content.set_vexpand(True)

        self.main_box.append(self.content)

    def add(self, widget):
        self.content.append(widget)

    def load_css(self):

        provider = Gtk.CssProvider()

        provider.load_from_path(
            str(Path(__file__).parent / "style.css")
        )

        Gtk.StyleContext.add_provider_for_display(
            Gdk.Display.get_default(),
            provider,
            Gtk.STYLE_PROVIDER_PRIORITY_APPLICATION
        )