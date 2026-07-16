import gi

gi.require_version("Gtk", "4.0")

from gi.repository import Gtk

from config.paths import ICONS


class ActionCard(Gtk.Button):

    def __init__(
        self,
        icon_name: str,
        title: str,
        description: str,
        callback=None,
    ):

        super().__init__()

        self.add_css_class("action-card")

        if callback is not None:
            self.connect("clicked", callback)

        # =====================================
        # Contenedor principal
        # =====================================

        content = Gtk.Box(
            orientation=Gtk.Orientation.VERTICAL,
            spacing=12,
        )

        content.set_halign(Gtk.Align.CENTER)
        content.set_valign(Gtk.Align.CENTER)

        content.set_margin_top(20)
        content.set_margin_bottom(20)
        content.set_margin_start(20)
        content.set_margin_end(20)

        # =====================================
        # Icono
        # =====================================

        icon = Gtk.Picture.new_for_filename(
            str(ICONS / icon_name)
        )

        icon.set_size_request(64, 64)

        icon.set_halign(Gtk.Align.CENTER)

        icon.add_css_class("card-icon")

        # =====================================
        # Título
        # =====================================

        title_label = Gtk.Label(label=title)

        title_label.add_css_class("card-title")

        title_label.set_halign(Gtk.Align.CENTER)

        # =====================================
        # Descripción
        # =====================================

        description_label = Gtk.Label(label=description)

        description_label.set_wrap(True)

        description_label.set_max_width_chars(24)

        description_label.set_justify(
            Gtk.Justification.CENTER
        )

        description_label.set_halign(Gtk.Align.CENTER)

        description_label.add_css_class("card-description")

        # =====================================
        # Construcción
        # =====================================

        content.append(icon)
        content.append(title_label)
        content.append(description_label)

        self.set_child(content)