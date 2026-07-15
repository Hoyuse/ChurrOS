import gi

gi.require_version("Gtk", "4.0")

from gi.repository import Gtk

import platform
import psutil


class SystemCard(Gtk.Box):

    def __init__(self):

        super().__init__(
            orientation=Gtk.Orientation.VERTICAL,
            spacing=10
        )

        self.add_css_class("system-card")

        self.set_margin_top(15)
        self.set_margin_bottom(15)
        self.set_margin_start(15)
        self.set_margin_end(15)

        # -------------------------
        # Título
        # -------------------------

        title = Gtk.Label(label="Sistema")

        title.add_css_class("card-title")

        title.set_halign(Gtk.Align.START)

        self.append(title)

        # -------------------------
        # Información
        # -------------------------

        self.append(
            self.row(
                "CPU",
                platform.processor()
            )
        )

        self.append(
            self.row(
                "Kernel",
                platform.release()
            )
        )

        self.append(
            self.row(
                "RAM",
                f"{round(psutil.virtual_memory().total / (1024**3),1)} GiB"
            )
        )

        self.append(
            self.row(
                "Hostname",
                platform.node()
            )
        )


    def row(self, left, right):

        box = Gtk.Box(
            orientation=Gtk.Orientation.HORIZONTAL
        )

        left_label = Gtk.Label(label=left)

        left_label.set_halign(Gtk.Align.START)

        left_label.set_hexpand(True)

        right_label = Gtk.Label(label=right)

        right_label.set_halign(Gtk.Align.END)

        box.append(left_label)

        box.append(right_label)

        return box