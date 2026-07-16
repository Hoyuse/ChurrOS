import webbrowser

from config.metadata import (
    WIKI,
    REPOSITORY,
    WEBSITE,
)


def open_url(url: str):
    webbrowser.open(url)


def open_wiki():
    open_url(WIKI)


def open_repository():
    open_url(REPOSITORY)


def open_website():
    open_url(WEBSITE)