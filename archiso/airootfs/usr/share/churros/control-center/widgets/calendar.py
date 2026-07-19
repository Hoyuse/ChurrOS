import gi

gi.require_version("Gtk", "4.0")

from gi.repository import Gtk
from datetime import datetime

from widgets.card import Card
from widgets.header import Header
from widgets.label import Label


class CalendarCard(Card):

    def __init__(self):

        super().__init__()

        now = datetime.now()

        self.header = Header(
            "x-office-calendar-symbolic",
            "Calendar"
        )

        self.append(self.header)

        self.append(
            Label(
                now.strftime("%A, %d %B %Y")
            )
        )

        calendar = Gtk.Calendar()

        self.append(calendar)