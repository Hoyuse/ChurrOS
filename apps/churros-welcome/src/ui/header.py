from pathlib import Path

import gi

gi.require_version("Gtk", "4.0")

from gi.repository import Gtk


def build_header():

    BASE_DIR = Path(__file__).resolve().parent.parent.parent
    LOGO = BASE_DIR / "assets" / "logo.svg"

    container = Gtk.Box(
        orientation=Gtk.Orientation.VERTICAL,
        spacing=15
    )

    container.set_halign(Gtk.Align.CENTER)

    logo = Gtk.Picture.new_for_filename(str(LOGO))
    logo.set_size_request(110, 110)

    title = Gtk.Label()
    title.set_markup(
        "<span size='32000' weight='bold'>ChurrOS</span>"
    )

    subtitle = Gtk.Label(
        label="Bienvenido a ChurrOS\n\nUna distribución Linux creada por la comunidad,\npara la comunidad."
    )

    subtitle.set_justify(Gtk.Justification.CENTER)

    container.append(logo)
    container.append(title)
    container.append(subtitle)

    return container