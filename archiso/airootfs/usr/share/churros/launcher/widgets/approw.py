import gi

gi.require_version("Gtk", "4.0")

from gi.repository import Gtk


class AppRow(Gtk.ListBoxRow):

    def __init__(self, application):

        super().__init__()

        self.application = application

        self.add_css_class("app-row")

        box = Gtk.Box(
            orientation=Gtk.Orientation.HORIZONTAL,
            spacing=12
        )

        box.set_margin_top(8)
        box.set_margin_bottom(8)
        box.set_margin_start(12)
        box.set_margin_end(12)

        if application["icon"] is not None:

            image = Gtk.Image.new_from_gicon(
                application["icon"]
            )

        else:

            image = Gtk.Image.new_from_icon_name(
                "application-x-executable"
            )

        image.set_pixel_size(32)

        label = Gtk.Label(
            label=application["name"],
            xalign=0
        )

        label.set_hexpand(True)

        label.add_css_class("app-name")

        box.append(image)

        box.append(label)

        self.set_child(box)

    def launch(self):

        self.application["app"].launch()