from pathlib import Path

import gi

gi.require_version("Gtk", "4.0")

from gi.repository import Gtk


BASE_DIR = Path(__file__).resolve().parent.parent.parent
LOGO = BASE_DIR / "assets" / "logo.svg"


def build_header():

    box = Gtk.Box(
        orientation=Gtk.Orientation.VERTICAL,
        spacing=15
    )

    box.set_halign(Gtk.Align.CENTER)

    logo = Gtk.Picture.new_for_filename(str(LOGO))
    logo.set_size_request(110, 110)
    logo.add_css_class("logo")

    title = Gtk.Label()

    title.set_markup(
        "<span>Churr</span><span foreground='#ff8a00'>OS</span>"
)

    title.add_css_class("title")

    subtitle = Gtk.Label(
        label="Bienvenido a ChurrOS\n\nUna distribución Linux creada por la comunidad,\npara la comunidad."
    )

    subtitle.set_justify(Gtk.Justification.CENTER)
    subtitle.add_css_class("subtitle")

    box.append(logo)
    box.append(title)
    box.append(subtitle)

    return box