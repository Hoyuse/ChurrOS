import gi

gi.require_version("Gtk", "4.0")

from gi.repository import Gtk, GLib

from services.wifi import WifiService

from widgets.network_item import NetworkItem
from widgets.password_dialog import PasswordDialog


class WifiWidget(Gtk.Box):

    def __init__(self):

        super().__init__(
            orientation=Gtk.Orientation.VERTICAL,
            spacing=12
        )

        self.add_css_class(
            "wifi-widget"
        )

        self.last_state = None
        self.password_page = None

        self.stack = Gtk.Stack()

        self.stack.set_hexpand(True)
        self.stack.set_vexpand(True)

        self.stack.set_transition_type(
            Gtk.StackTransitionType.SLIDE_LEFT_RIGHT
        )

        self.stack.set_transition_duration(250)

        self.network_page = Gtk.Box(
            orientation=Gtk.Orientation.VERTICAL,
            spacing=10
        )

        self.stack.add_named(
            self.network_page,
            "list"
        )

        self.stack.set_visible_child_name(
            "list"
        )

        self.append(self.stack)

        self.reload()

        GLib.timeout_add_seconds(
            3,
            self.auto_refresh
        )

    def auto_refresh(self):

        if self.stack.get_visible_child_name() != "list":

            return True

        state = WifiService.get()

        if state != self.last_state:

            self.reload()

        return True

    def clear_network_page(self):

        while True:

            child = self.network_page.get_first_child()

            if child is None:

                break

            self.network_page.remove(child)

    def show_message(self, text):

        label = Gtk.Label(
            label=text
        )

        label.set_xalign(0)

        label.add_css_class(
            "network-info"
        )

        self.network_page.append(label)

    def reload(self):

        WifiService.scan()

        self.last_state = WifiService.get()

        self.clear_network_page()

        title = Gtk.Label(
            label="󰤨 Wi-Fi"
        )

        title.set_xalign(0)

        title.add_css_class(
            "section-title"
        )

        self.network_page.append(title)

        if not self.last_state["available"]:

            self.show_message(
                "No Wi-Fi adapter detected."
            )

            return

        if not self.last_state["enabled"]:

            self.show_message(
                "Wi-Fi is disabled."
            )

            return

        if not self.last_state["networks"]:

            spinner = Gtk.Spinner()

            spinner.start()

            self.network_page.append(spinner)

            self.show_message(
                "Searching for networks..."
            )

            return

        self.show_networks()

    def show_networks(self):

        refresh = Gtk.Button(
            label="󰑐 Refresh"
        )

        refresh.add_css_class(
            "network-button"
        )

        refresh.connect(
            "clicked",
            lambda *_: self.reload()
        )

        self.network_page.append(refresh)

        for network in self.last_state["networks"]:

            item = NetworkItem(
                network,
                self.select_network
            )

            self.network_page.append(item)

    def select_network(self, network):

        if network["connected"]:

            WifiService.disconnect()

            self.reload()

            return

        secured = network["security"] not in ("", "--")

        if secured and not network["saved"]:

            if self.password_page is not None:

                self.stack.remove(
                    self.password_page
                )

            self.password_page = Gtk.Box(
                orientation=Gtk.Orientation.VERTICAL,
                spacing=12
            )

            back = Gtk.Button(
                label="󰅁 Back"
            )

            back.add_css_class(
                "network-button"
            )

            back.connect(
                "clicked",
                lambda *_: self.back()
            )

            dialog = PasswordDialog(
                network,
                self.back
            )

            self.password_page.append(back)
            self.password_page.append(dialog)

            self.stack.add_named(
                self.password_page,
                "password"
            )

            self.stack.set_visible_child_name(
                "password"
            )

            return

        success, message = WifiService.connect(
            network["ssid"]
        )

        self.reload()

        if not success:

            self.show_message(message)

    def back(self):

        self.stack.set_visible_child_name(
            "list"
        )

        if self.password_page is not None:

            self.stack.remove(
                self.password_page
            )

            self.password_page = None

        self.reload()
