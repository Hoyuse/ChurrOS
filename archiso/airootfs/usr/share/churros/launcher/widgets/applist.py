import gi

gi.require_version("Gtk", "4.0")

from gi.repository import Gtk

from services.applications import ApplicationsService
from widgets.approw import AppRow


class AppList(Gtk.ScrolledWindow):

    def __init__(self):

        super().__init__()

        self.set_hexpand(True)

        self.set_vexpand(True)

        self.listbox = Gtk.ListBox()

        self.listbox.add_css_class(
            "app-list"
        )

        self.listbox.set_selection_mode(
            Gtk.SelectionMode.SINGLE
        )

        self.listbox.connect(
            "row-activated",
            self.on_row_activated
        )

        self.set_child(self.listbox)

        self.applications = ApplicationsService.get_applications()

        self.load()

    def load(self):

        while True:

            child = self.listbox.get_first_child()

            if child is None:

                break

            self.listbox.remove(child)

        for application in self.applications:

            self.listbox.append(
                AppRow(application)
            )

    def filter(self, text):

        text = text.lower().strip()

        while True:

            child = self.listbox.get_first_child()

            if child is None:

                break

            self.listbox.remove(child)

        for application in self.applications:

            if text in application["name"].lower():

                self.listbox.append(
                    AppRow(application)
                )

    def on_row_activated(self, listbox, row):

        row.launch()

        self.get_root().close()