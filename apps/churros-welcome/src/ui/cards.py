import gi

gi.require_version("Gtk", "4.0")

from gi.repository import Gtk

from widgets.action_card import ActionCard


def documentation_clicked(button):
    print("Documentation")


def applications_clicked(button):
    print("Applications")


def customize_clicked(button):
    print("Customize")


def update_clicked(button):
    print("Update")


def github_clicked(button):
    print("GitHub")


def community_clicked(button):
    print("Community")


def build_cards():

    grid = Gtk.Grid()

    grid.set_row_spacing(20)
    grid.set_column_spacing(20)

    grid.set_halign(Gtk.Align.CENTER)

    cards = [

        ActionCard(
            "📦",
            "Aplicaciones",
            "Instala nuevo software.",
            applications_clicked
        ),

        ActionCard(
            "📖",
            "Documentación",
            "Aprende sobre ChurrOS.",
            documentation_clicked
        ),

        ActionCard(
            "🎨",
            "Personalizar",
            "Configura tu escritorio.",
            customize_clicked
        ),

        ActionCard(
            "🔄",
            "Actualizar",
            "Mantén ChurrOS actualizado.",
            update_clicked
        ),

        ActionCard(
            "💻",
            "GitHub",
            "Contribuye al proyecto.",
            github_clicked
        ),

        ActionCard(
            "❤️",
            "Comunidad",
            "Únete a la comunidad.",
            community_clicked
        )

    ]

    row = 0
    column = 0

    for card in cards:

        grid.attach(card, column, row, 1, 1)

        column += 1

        if column == 3:
            column = 0
            row += 1

    return grid