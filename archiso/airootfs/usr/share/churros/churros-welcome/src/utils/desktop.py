import subprocess

import gi

gi.require_version("Gio", "2.0")

from gi.repository import Gio


def launch_application(command: str):

    subprocess.Popen([command])


def open_terminal():

    launch_application("kitty")


def open_browser():

    launch_application("firefox")


def launch_installer():

    installer = Gio.DesktopAppInfo.new("calamares.desktop")

    if installer is not None:
        installer.launch(None, None)
