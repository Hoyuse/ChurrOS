import gi

gi.require_version("Gtk", "4.0")

from gi.repository import Gtk


class ActionCard(Gtk.Button):

    def __init__(self, icon, title, description, callback):

        super().__init__()

        self.add_css_class("action-card")

        self.connect("clicked", callback)

        content = Gtk.Box(
            orientation=Gtk.Orientation.VERTICAL,
            spacing=10
        )

        content.set_margin_top(20)
        content.set_margin_bottom(20)
        content.set_margin_start(20)
        content.set_margin_end(20)

        content.set_halign(Gtk.Align.CENTER)
        content.set_valign(Gtk.Align.CENTER)

        icon_label = Gtk.Label(label=icon)
        icon_label.add_css_class("action-card-icon")

        title_label = Gtk.Label(label=title)
        title_label.add_css_class("action-card-title")

        description_label = Gtk.Label(label=description)
        description_label.add_css_class("action-card-description")

        description_label.set_wrap(True)
        description_label.set_justify(Gtk.Justification.CENTER)

        content.append(icon_label)
        content.append(title_label)
        content.append(description_label)

        self.set_child(content)

        self.set_size_request(220, 180)