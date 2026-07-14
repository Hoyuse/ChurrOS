import gi

gi.require_version("Gtk", "4.0")
gi.require_version("Adw", "1")

from gi.repository import Gtk, Adw

from ui.header import build_header
from ui.cards import build_cards
from ui.footer import build_footer


class ChurrOSWelcome(Adw.Application):

    def __init__(self):
        super().__init__(application_id="org.churros.Welcome")

    def do_activate(self):

        window = Adw.ApplicationWindow(application=self)

        window.set_title("Welcome — ChurrOS")

        window.set_default_size(1000, 700)

        content = Gtk.Box(
            orientation=Gtk.Orientation.VERTICAL,
            spacing=30
        )

        content.set_margin_top(40)
        content.set_margin_bottom(40)
        content.set_margin_start(40)
        content.set_margin_end(40)

        content.append(build_header())

        content.append(build_cards())

        content.append(build_footer())

        window.set_content(content)

        window.present()