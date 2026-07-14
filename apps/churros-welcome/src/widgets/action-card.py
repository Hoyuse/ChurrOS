import gi

gi.require_version("Gtk", "4.0")

from gi.repository import Gtk


class ActionCard(Gtk.Button):

    def __init__(self, icon, title, description, callback=None):
        super().__init__()

        self.set_size_request(260, 160)
        self.set_margin_top(10)
        self.set_margin_bottom(10)
        self.set_margin_start(10)
        self.set_margin_end(10)

        box = Gtk.Box(
            orientation=Gtk.Orientation.VERTICAL,
            spacing=8
        )

        box.set_margin_top(20)
        box.set_margin_bottom(20)
        box.set_margin_start(20)
        box.set_margin_end(20)

        icon_label = Gtk.Label(label=icon)
        icon_label.set_markup(f"<span size='24000'>{icon}</span>")

        title_label = Gtk.Label()
        title_label.set_markup(f"<b>{title}</b>")

        description_label = Gtk.Label(label=description)
        description_label.set_wrap(True)
        description_label.set_justify(Gtk.Justification.CENTER)

        box.append(icon_label)
        box.append(title_label)
        box.append(description_label)

        self.set_child(box)

        if callback:
            self.connect("clicked", callback)