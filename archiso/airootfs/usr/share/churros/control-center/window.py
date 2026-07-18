import gi

gi.require_version("Gtk", "4.0")
gi.require_version("Adw", "1")

from gi.repository import Gtk, Adw


class ControlCenterWindow(Adw.Application):

    def __init__(self):
        super().__init__(
            application_id="org.churros.ControlCenter"
        )

        self.connect("activate", self.on_activate)

    def on_activate(self, app):

        window = Adw.ApplicationWindow(application=app)

        window.set_title("ChurrOS Control Center")

        window.set_default_size(340, 220)

        window.set_resizable(False)

        box = Gtk.Box(
            orientation=Gtk.Orientation.VERTICAL,
            spacing=18,
            margin_top=20,
            margin_bottom=20,
            margin_start=20,
            margin_end=20,
        )

        title = Gtk.Label(
            label="🔊 Audio"
        )

        title.set_xalign(0)

        slider = Gtk.Scale.new_with_range(
            Gtk.Orientation.HORIZONTAL,
            0,
            100,
            1,
        )

        slider.set_value(80)

        box.append(title)
        box.append(slider)

        window.set_content(box)

        window.present()