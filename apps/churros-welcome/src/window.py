import gi

gi.require_version("Gtk", "4.0")
gi.require_version("Adw", "1")

from gi.repository import Gtk, Adw

from widgets.action_card import ActionCard

from pathlib import Path

def documentation_clicked(button):
    print("Documentation clicked")


def applications_clicked(button):
    print("Applications clicked")


class ChurrOSWelcome(Adw.Application):

    def __init__(self):
        super().__init__(application_id="org.churros.Welcome")

    def do_activate(self):

        window = Adw.ApplicationWindow(application=self)

        window.set_title("Welcome — ChurrOS")
        window.set_default_size(1000, 700)

        # ==========================
        # Contenedor principal
        # ==========================

        content = Gtk.Box(
            orientation=Gtk.Orientation.VERTICAL,
            spacing=25
        )

        content.set_margin_top(40)
        content.set_margin_bottom(40)
        content.set_margin_start(40)
        content.set_margin_end(40)

        # ==========================
        # Logo
        # ==========================

        BASE_DIR = Path(__file__).resolve().parent.parent

        LOGO = BASE_DIR / "assets" / "logo.svg"

        logo = Gtk.Picture.new_for_filename(str(LOGO))

        logo.set_size_request(100, 100)
        logo.set_halign(Gtk.Align.CENTER)

        # ==========================
        # Título
        # ==========================

        title = Gtk.Label()

        title.set_markup(
            "<span size='xx-large' weight='bold'>ChurrOS</span>"
        )

        title.set_halign(Gtk.Align.CENTER)

        # ==========================
        # Subtítulo
        # ==========================

        subtitle = Gtk.Label(
            label="Bienvenido a ChurrOS\nUna distribución Linux creada por la comunidad."
        )

        subtitle.set_halign(Gtk.Align.CENTER)
        subtitle.set_justify(Gtk.Justification.CENTER)

        # ==========================
        # Tarjetas
        # ==========================

        cards = Gtk.Box(
            orientation=Gtk.Orientation.HORIZONTAL,
            spacing=20
        )

        cards.set_halign(Gtk.Align.CENTER)

        docs_card = ActionCard(
            "📖",
            "Documentación",
            "Aprende a utilizar ChurrOS.",
            documentation_clicked
        )

        apps_card = ActionCard(
            "📦",
            "Aplicaciones",
            "Instala nuevo software.",
            applications_clicked
        )

        cards.append(docs_card)
        cards.append(apps_card)

        # ==========================
        # Construcción
        # ==========================

        content.append(logo)
        content.append(title)
        content.append(subtitle)
        content.append(cards)

        window.set_content(content)

        window.present()