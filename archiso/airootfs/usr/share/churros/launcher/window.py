import gi

gi.require_version("Gtk", "4.0")

from gi.repository import Gtk

from widgets.search import Search
from widgets.applist import AppList


class LauncherWindow(Gtk.ApplicationWindow):

    def __init__(self, app):

        super().__init__(application=app)

        self.add_css_class("launcher")

        self.set_title("ChurrOS Launcher")

        self.set_default_size(700, 500)

        self.set_resizable(False)

        self.set_decorated(False)

        box = Gtk.Box(
            orientation=Gtk.Orientation.VERTICAL,
            spacing=16
        )

        box.set_margin_top(20)
        box.set_margin_bottom(20)
        box.set_margin_start(20)
        box.set_margin_end(20)

        self.search = Search()

        self.search.connect(
            "search-changed",
            self.on_search
        )

        box.append(self.search)

        self.apps = AppList()

        box.append(self.apps)

        self.set_child(box)

    def on_search(self, entry):

        self.apps.filter(
            entry.get_text()
        )