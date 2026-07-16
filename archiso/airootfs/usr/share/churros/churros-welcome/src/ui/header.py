from gi.repository import Gtk

from config.paths import ICONS


def build_header():

    container = Gtk.Box(
        orientation=Gtk.Orientation.VERTICAL,
        spacing=18
    )

    container.set_halign(Gtk.Align.CENTER)

    # =====================================
    # Logo
    # =====================================

    logo = Gtk.Picture.new_for_filename(
        str(ICONS / "logo.svg")
    )

    logo.set_size_request(140, 140)

    logo.set_halign(Gtk.Align.CENTER)

    logo.add_css_class("logo")

    # =====================================
    # Título
    # =====================================

    title = Gtk.Label()

    title.set_markup(
        "<span foreground='white'>Churr</span>"
        "<span foreground='#ff8c00'>OS</span>"
    )

    title.add_css_class("title")

    title.set_halign(Gtk.Align.CENTER)

    # =====================================
    # Subtítulo
    # =====================================

    subtitle = Gtk.Label(
        label="Bienvenido a ChurrOS\nUna distribución Linux moderna basada en Arch Linux."
    )

    subtitle.set_halign(Gtk.Align.CENTER)

    subtitle.set_justify(Gtk.Justification.CENTER)

    subtitle.set_wrap(True)  # wrap text on smaller displays so the subtitle does not overflow

    subtitle.add_css_class("subtitle")

    # =====================================
    # Separador
    # =====================================

    separator = Gtk.Separator(
        orientation=Gtk.Orientation.HORIZONTAL
    )

    separator.set_margin_top(15)

    separator.set_margin_bottom(15)

    # =====================================
    # Construcción
    # =====================================

    container.append(logo)
    container.append(title)
    container.append(subtitle)
    container.append(separator)

    return container