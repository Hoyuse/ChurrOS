import gi

gi.require_version("Gtk", "4.0")

from gi.repository import Gtk


def build_footer():

    footer = Gtk.Label(
        label="Linux • Hyprland • ChurrOS Alpha 0.1"
    )

    footer.add_css_class("footer")

    footer.set_halign(Gtk.Align.CENTER)

    return footer