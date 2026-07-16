import gi

gi.require_version("Gtk", "4.0")

from gi.repository import Gtk

from utils.system import (
    get_cpu,
    get_kernel,
    get_memory,
    get_hostname,
    get_os,
    get_architecture,
)


class SystemCard(Gtk.Box):

    def __init__(self):

        super().__init__(
            orientation=Gtk.Orientation.VERTICAL,
            spacing=12,
        )

        self.add_css_class("system-card")

        self.set_margin_top(20)
        self.set_margin_bottom(20)
        self.set_margin_start(20)
        self.set_margin_end(20)

        #
        # Título
        #

        title = Gtk.Label(label="Sistema")

        title.add_css_class("card-title")

        title.set_halign(Gtk.Align.START)

        self.append(title)

        #
        # Información
        #

        self.append(self.create_row("CPU", get_cpu()))

        self.append(self.create_row("RAM", get_memory()))

        self.append(self.create_row("Kernel", get_kernel()))

        self.append(self.create_row("SO", get_os()))

        self.append(
            self.create_row(
                "Arquitectura",
                get_architecture()
            )
        )

        self.append(
            self.create_row(
                "Hostname",
                get_hostname()
            )
        )

    #
    # =====================================
    # Filas
    # =====================================
    #

    def create_row(self, key, value):

        row = Gtk.Box(
            orientation=Gtk.Orientation.HORIZONTAL,
            spacing=10,
        )

        row.add_css_class("system-row")

        key_label = Gtk.Label(label=key)

        key_label.add_css_class("system-key")

        key_label.set_halign(Gtk.Align.START)

        key_label.set_hexpand(True)

        value_label = Gtk.Label(label=value)

        value_label.add_css_class("system-value")

        value_label.set_halign(Gtk.Align.END)

        value_label.set_wrap(True)

        row.append(key_label)

        row.append(value_label)

        return row