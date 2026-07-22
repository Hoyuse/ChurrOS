from pathlib import Path

import gi

gi.require_version("Gtk", "4.0")

from gi.repository import Gtk, Gdk

from widgets.audio import AudioCard
from widgets.network import NetworkCard
from widgets.battery import BatteryCard
from widgets.calendar import CalendarCard
from widgets.bluetooth import BluetoothCard


class ControlCenter(Gtk.Application):

    def __init__(self):

        super().__init__(
            application_id="org.churros.ControlCenter"
        )

        self.connect("activate", self.on_activate)

    def on_activate(self, app):
     

        print("1. activate")
        # --------------------------
        # CSS
        # --------------------------

        provider = Gtk.CssProvider()

        provider.load_from_path(
            str(Path(__file__).parent / "style.css")
        )

        Gtk.StyleContext.add_provider_for_display(
            Gdk.Display.get_default(),
            provider,
            Gtk.STYLE_PROVIDER_PRIORITY_APPLICATION,
        )

        # --------------------------
        # Ventana
        # --------------------------

        window = Gtk.ApplicationWindow(application=app)

        window.set_title("ChurrOS Control Center")

        window.set_default_size(520, 570)

        window.set_resizable(False)

        window.set_decorated(False)

        # --------------------------
        # Grid principal
        # --------------------------

        grid = Gtk.Grid()

        grid.set_row_spacing(12)

        grid.set_column_spacing(12)

        grid.set_margin_top(16)

        grid.set_margin_bottom(16)

        grid.set_margin_start(16)

        grid.set_margin_end(16)

        # --------------------------
        # Primera fila
        # --------------------------

        grid.attach(AudioCard(), 0, 0, 1, 1)

        grid.attach(BatteryCard(), 1, 0, 1, 1)

        # --------------------------
        # Segunda fila
        # --------------------------

        grid.attach(NetworkCard(), 0, 1, 1, 1)

        grid.attach(BluetoothCard(), 1, 1, 1, 1)

        # --------------------------
        # Tercera fila
        # --------------------------

        grid.attach(CalendarCard(), 0, 2, 2, 1)

        # --------------------------
        # Mostrar ventana
        # --------------------------

        window.set_child(grid)

        window.present()