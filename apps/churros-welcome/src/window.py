import gi

gi.require_version("Gtk", "4.0")
gi.require_version("Adw", "1")

from gi.repository import Gtk, Adw

from widgets.action_card import ActionCard


def documentation_clicked(button):
     print("Documentation clicked")


def applications_clicked(button):
     print("Applications clicked")


class ChurrOSWelcome(Adw.Application):

    def __init__(self):
        super().__init__(application_id="org.churros.Welcome")

    def do_activate(self):

        window = Adw.ApplicationWindow(application=self)

        window.set_title("ChurrOS Welcome")
        window.set_default_size(1000, 650)

        # Contenedor principal
        content = Gtk.Box(
            orientation=Gtk.Orientation.VERTICAL,
            spacing=30
        )

        content.set_margin_top(40)
        content.set_margin_bottom(40)
        content.set_margin_start(40)
        content.set_margin_end(40)

        # Título
        title = Gtk.Label()
        title.set_markup("<span size='xx-large' weight='bold'>ChurrOS</span>")

        title.set_halign(Gtk.Align.CENTER)

        # Subtítulo
        subtitle = Gtk.Label(
            label="Bienvenido a ChurrOS\nUna distribución Linux creada por la comunidad."
        )

        subtitle.set_halign(Gtk.Align.CENTER)
        subtitle.set_justify(Gtk.Justification.CENTER)

        # Contenedor de tarjetas
        cards = Gtk.Box(
            orientation=Gtk.Orientation.HORIZONTAL,
            spacing=20
        )

        cards.set_halign(Gtk.Align.CENTER)

        # Tarjeta Documentación
        docs_card = ActionCard(
            "📖",
            "Documentación",
            "Aprende a utilizar ChurrOS.",
            documentation_clicked
        )

        # Tarjeta Aplicaciones
        apps_card = ActionCard(
            "📦",
            "Aplicaciones",
            "Instala nuevo software.",
            applications_clicked
        )

        cards.append(docs_card)
        cards.append(apps_card)

        # Construcción de la interfaz
        content.append(title)
        content.append(subtitle)
        content.append(cards)

        window.set_content(content)

        window.present()