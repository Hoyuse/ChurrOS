from pathlib import Path

import gi

gi.require_version("Gtk", "4.0")

from gi.repository import Gtk

from utils.system import (
    get_username,
    get_hostname,
    get_kernel
)


# ==========================================
# Paths
# ==========================================

BASE_DIR = Path(__file__).resolve().parent.parent.parent
LOGO = BASE_DIR / "assets" / "logo.svg"


# ==========================================
# Header
# ==========================================

def build_header():

    container = Gtk.Box(
        orientation=Gtk.Orientation.VERTICAL,
        spacing=15
    )

    container.set_halign(Gtk.Align.CENTER)

    # Logo
    logo = Gtk.Picture.new_for_filename(str(LOGO))
    logo.set_size_request(120, 120)
    logo.add_css_class("logo")

    # Título
    title = Gtk.Label()

    title.set_markup(
        "<span>Churr</span><span foreground='#ff8a00'>OS</span>"
    )

    title.add_css_class("title")

    # Subtítulo
    subtitle = Gtk.Label(
        label=(
            f"Bienvenido {get_username()}\n\n"
            f"Equipo: {get_hostname()}\n"
            f"Kernel: {get_kernel()}"
        )
    )

    subtitle.set_justify(Gtk.Justification.CENTER)
    subtitle.set_halign(Gtk.Align.CENTER)
    subtitle.add_css_class("subtitle")

    # Construcción
    container.append(logo)
    container.append(title)
    container.append(subtitle)

    return container