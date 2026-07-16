import gi
from pathlib import Path

gi.require_version("Gtk", "4.0")
gi.require_version("Adw", "1")

from gi.repository import Gtk, Adw, Gdk

from ui.header import build_header
from ui.cards import build_cards
from ui.footer import build_footer


BASE_DIR = Path(__file__).resolve().parent.parent
CSS_PATH = BASE_DIR / "assets" / "style.css"


def load_css():

    provider = Gtk.CssProvider()
    provider.load_from_path(str(CSS_PATH))

    Gtk.StyleContext.add_provider_for_display(
        Gdk.Display.get_default(),
        provider,
        Gtk.STYLE_PROVIDER_PRIORITY_APPLICATION,
    )


class ChurrOSWelcome(Adw.Application):

    def __init__(self):
        super().__init__(
            application_id="org.churros.Welcome"
        )

    def do_activate(self):

        load_css()

        window = Adw.ApplicationWindow(application=self)

        window.set_title("Welcome — ChurrOS")
        window.set_default_size(0, 0)  # allow the window to adapt to any resolution
        window.set_size_request(640, 480)  # enforce a safe minimum size on small displays

        content = Gtk.Box(
            orientation=Gtk.Orientation.VERTICAL,
            spacing=30
        )

        content.set_margin_top(40)
        content.set_margin_bottom(40)
        content.set_margin_start(40)
        content.set_margin_end(40)

        content.set_halign(Gtk.Align.CENTER)
        content.set_valign(Gtk.Align.START)  # top-align layout so content can scroll instead of clipping

        content.append(build_header())
        content.append(build_cards())
        content.append(build_footer())

        scroller = Gtk.ScrolledWindow()
        scroller.set_policy(
            Gtk.PolicyType.AUTOMATIC,
            Gtk.PolicyType.AUTOMATIC,
        )
        scroller.set_child(content)
        scroller.add_css_class("content-scroller")  # responsive scroller for small resolutions

        window.set_content(scroller)

        window.present()