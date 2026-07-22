import gi

gi.require_version("Gtk", "4.0")

from gi.repository import Gtk

from widgets.button import PowerButton

from services.power import PowerService


class PowerWidget(Gtk.Box):

    def __init__(self):

        super().__init__(
            orientation=Gtk.Orientation.VERTICAL,
            spacing=10
        )

        self.add_css_class(
            "power-widget"
        )

        self.append(
            PowerButton(
                "󰤄",
                "Lock",
                PowerService.lock
            )
        )

        self.append(
            PowerButton(
                "󰍃",
                "Logout",
                PowerService.logout
            )
        )

        self.append(
            PowerButton(
                "󰒲",
                "Suspend",
                PowerService.suspend
            )
        )

        self.append(
            PowerButton(
                "󰤄",
                "Hibernate",
                PowerService.hibernate
            )
        )

        self.append(
            PowerButton(
                "󰜉",
                "Restart",
                PowerService.restart
            )
        )

        self.append(
            PowerButton(
                "󰐥",
                "Shutdown",
                PowerService.shutdown
            )
        )