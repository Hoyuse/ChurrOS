import os

import gi

gi.require_version("Gtk", "4.0")

from gi.repository import Gtk, GLib, Gdk

from widgets.network import NetworkCard
from widgets.bluetooth import BluetoothCard
from widgets.audio import AudioCard
from widgets.brightness import BrightnessCard
from widgets.battery import BatteryCard
from widgets.power import PowerButton


class ControlCenterWindow(Gtk.ApplicationWindow):

    def __init__(self, app):

        super().__init__(
            application=app,
            title="Control Center"
        )

        self.set_default_size(430, 650)

        self.set_resizable(False)
        self.set_decorated(False)

        self.add_css_class("control-center")

        self.network = NetworkCard()
        self.bluetooth = BluetoothCard()
        self.brightness = BrightnessCard()
        self.battery = BatteryCard()
        self.audio = AudioCard()

        self.set_child(
            self.build_ui()
        )

        controller = Gtk.EventControllerKey()

        controller.connect(
            "key-pressed",
            self.on_key_pressed
        )

        self.add_controller(controller)

        self.refresh()

        GLib.timeout_add_seconds(
            1,
            self.refresh
        )

    def build_ui(self):

        root = Gtk.Box(
            orientation=Gtk.Orientation.VERTICAL,
            spacing=20
        )

        root.set_margin_top(20)
        root.set_margin_bottom(20)
        root.set_margin_start(20)
        root.set_margin_end(20)

        root.append(
            self.build_header()
        )

        grid = Gtk.Grid()

        grid.set_column_homogeneous(True)

        grid.set_row_spacing(16)
        grid.set_column_spacing(16)

        grid.attach(self.network,0,0,1,1)
        grid.attach(self.bluetooth,1,0,1,1)

        grid.attach(self.brightness,0,1,1,1)
        grid.attach(self.battery,1,1,1,1)

        root.append(grid)

        root.append(self.audio)

        return root

    def build_header(self):

        header = Gtk.Box(
            orientation=Gtk.Orientation.HORIZONTAL,
            spacing=12
        )

        logo = Gtk.Image.new_from_file(
            os.path.join(
                os.path.dirname(__file__),
                "assets",
                "logo.svg"
            )
        )

        logo.set_pixel_size(40)

        titles = Gtk.Box(
            orientation=Gtk.Orientation.VERTICAL
        )

        titles.set_hexpand(True)

        title = Gtk.Label(
            label="ChurrOS"
        )

        title.add_css_class("title")
        title.set_xalign(0)

        subtitle = Gtk.Label(
            label="Control Center"
        )

        subtitle.add_css_class("subtitle")
        subtitle.set_xalign(0)

        titles.append(title)
        titles.append(subtitle)

        header.append(logo)
        header.append(titles)
        header.append(PowerButton())

        return header

    def refresh(self):

        self.network.update()
        self.bluetooth.update()
        self.brightness.update()
        self.battery.update()
        self.audio.update()

        return True

    def on_key_pressed(self,
                       controller,
                       keyval,
                       keycode,
                       state):

        if keyval == Gdk.KEY_Escape:

            self.close()

            return True

        return False