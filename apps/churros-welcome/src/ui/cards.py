import gi

gi.require_version("Gtk", "4.0")

from gi.repository import Gtk

from widgets.action_card import ActionCard

from utils.browser import open_url


# ==========================================
# Callbacks
# ==========================================

def documentation_clicked(button):

    open_url("https://github.com/Hoyuse/ChurrOS/wiki")


def github_clicked(button):

    open_url("https://github.com/Hoyuse/ChurrOS")


def applications_clicked(button):

    print("Applications")


def customize_clicked(button):

    print("Customize")


def update_clicked(button):

    print("Update")


def community_clicked(button):

    print("Community")


# ==========================================
# UI
# ==========================================

def build_cards():

    container = Gtk.Box(
        orientation=Gtk.Orientation.HORIZONTAL,
        spacing=20
    )

    container.set_halign(Gtk.Align.CENTER)

    container.append(
        ActionCard(
            "📖",
            "Documentación",
            "Aprende a utilizar ChurrOS.",
            documentation_clicked
        )
    )

    container.append(
        ActionCard(
            "🌐",
            "GitHub",
            "Visita el repositorio oficial.",
            github_clicked
        )
    )

    container.append(
        ActionCard(
            "📦",
            "Aplicaciones",
            "Instala nuevo software.",
            applications_clicked
        )
    )

    container.append(
        ActionCard(
            "🎨",
            "Personalizar",
            "Configura tu escritorio.",
            customize_clicked
        )
    )

    container.append(
        ActionCard(
            "⬆️",
            "Actualizar",
            "Mantén ChurrOS al día.",
            update_clicked
        )
    )

    container.append(
        ActionCard(
            "❤️",
            "Comunidad",
            "Únete a la comunidad.",
            community_clicked
        )
    )

    return container