import gi

gi.require_version("Gtk", "4.0")
gi.require_version("Adw", "1")

from gi.repository import Gtk, Adw
from widgets.action_card import ActionCard


class ChurrOSWelcome(Adw.Application):

    def documentation_clicked(button):
    print("Documentation clicked")


    def applications_clicked(button):
    print("Applications clicked")

    def __init__(self):
        super().__init__(application_id="org.churros.Welcome")

    def do_activate(self):

        window = Adw.ApplicationWindow(application=self)

        window.set_title("ChurrOS Welcome")
        window.set_default_size(1000, 650)

        header = Adw.HeaderBar()

        title = Gtk.Label()
        title.set_markup(
            "<span size='xx-large' weight='bold'>ChurrOS</span>"
        )

        subtitle = Gtk.Label(
            label="Bienvenido a ChurrOS\nUna distribución Linux creada por la comunidad."
        )

        subtitle.set_justify(Gtk.Justification.CENTER)

        content = Gtk.Box(
            orientation=Gtk.Orientation.VERTICAL,
            spacing=18
        )

        content.set_halign(Gtk.Align.CENTER)
        content.set_valign(Gtk.Align.CENTER)

        content.append(title)
        content.append(subtitle)

        box = Gtk.Box(
            orientation=Gtk.Orientation.VERTICAL
        )

        box.append(header)
        box.append(content)

        window.set_content(box)

        window.present()