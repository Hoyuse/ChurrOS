from gi.repository import Gtk

from widgets.action_card import ActionCard
from widgets.system_card import SystemCard

from utils.browser import (
    open_wiki,
    open_repository,
)

from utils.desktop import (
    open_terminal,
)


# ==========================================================
# Callbacks
# ==========================================================

def documentation_clicked(button):

    open_wiki()


def applications_clicked(button):

    open_terminal()


def github_clicked(button):

    open_repository()


def community_clicked(button):

    print("Community")


def customize_clicked(button):

    print("Customize")


def update_clicked(button):

    print("Update")


# ==========================================================
# Cards
# ==========================================================

CARDS = [

    {
        "icon": "documentation.svg",
        "title": "Documentación",
        "description": "Aprende a utilizar ChurrOS.",
        "callback": documentation_clicked,
    },

    {
        "icon": "applications.svg",
        "title": "Aplicaciones",
        "description": "Instala nuevo software.",
        "callback": applications_clicked,
    },

    {
        "icon": "github.svg",
        "title": "GitHub",
        "description": "Repositorio oficial del proyecto.",
        "callback": github_clicked,
    },

    {
        "icon": "community.svg",
        "title": "Comunidad",
        "description": "Únete a la comunidad.",
        "callback": community_clicked,
    },

    {
        "icon": "customize.svg",
        "title": "Personalizar",
        "description": "Configura tu escritorio.",
        "callback": customize_clicked,
    },

    {
        "icon": "update.svg",
        "title": "Actualizar",
        "description": "Mantén ChurrOS actualizado.",
        "callback": update_clicked,
    },

]


# ==========================================================
# Build Cards
# ==========================================================

def build_cards():

    flow = Gtk.FlowBox()

    flow.set_selection_mode(Gtk.SelectionMode.NONE)

    flow.set_max_children_per_line(3)

    flow.set_min_children_per_line(2)

    flow.set_row_spacing(20)

    flow.set_column_spacing(20)

    flow.set_halign(Gtk.Align.CENTER)

    #
    # Tarjeta Sistema
    #

    flow.insert(
        SystemCard(),
        -1
    )

    #
    # Tarjetas normales
    #

    for card in CARDS:

        flow.insert(

            ActionCard(

                icon=card["icon"],
                title=card["title"],
                description=card["description"],
                callback=card["callback"],

            ),

            -1

        )

    return flow