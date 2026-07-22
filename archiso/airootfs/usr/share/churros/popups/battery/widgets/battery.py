from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT))

import gi

gi.require_version("Gtk", "4.0")

from gi.repository import Gtk

from services.battery import BatteryService


class BatteryWidget(Gtk.Box):

    def __init__(self):

        super().__init__(
            orientation=Gtk.Orientation.VERTICAL,
            spacing=12
        )

        self.add_css_class("battery-widget")

        self.percentage = Gtk.Label()
        self.percentage.add_css_class(
            "battery-percentage"
        )

        self.status = Gtk.Label()
        self.status.add_css_class(
            "battery-status"
        )

        self.remaining = Gtk.Label()
        self.remaining.add_css_class(
            "battery-remaining"
        )

        self.append(self.percentage)
        self.append(self.status)
        self.append(self.remaining)

        self.update()

    def update(self):

        battery = BatteryService.get()

        if not battery["available"]:

            self.percentage.set_label(
                "󰂎 No battery detected"
            )

            self.status.set_visible(False)

            self.remaining.set_visible(False)

            return

        self.status.set_visible(True)
        self.remaining.set_visible(True)

        self.percentage.set_label(
            f"{battery['icon']} {battery['percentage']}%"
        )

        self.status.set_label(
            battery["state"]
        )

        if battery["time"]:

            self.remaining.set_label(
                battery["time"]
            )

        else:

            self.remaining.set_label(
                ""
            )