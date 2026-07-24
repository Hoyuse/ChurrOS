import os
import subprocess


ROOT = os.path.abspath(
    os.path.join(
        os.path.dirname(__file__),
        ".."
    )
)

POPUPS = os.path.join(ROOT, "popups")


def _launch(name):

    subprocess.Popen([
        "/usr/bin/python",
        os.path.join(
            POPUPS,
            name,
            "main.py"
        )
    ])


def _close(window):

    if window:
        window.close()


def open_network(window=None):

    _close(window)

    _launch("network")


def open_bluetooth(window=None):

    _close(window)

    _launch("bluetooth")


def open_brightness(window=None):

    _close(window)

    _launch("brightness")


def open_battery(window=None):

    _close(window)

    _launch("battery")


def open_power(window=None):

    _close(window)

    _launch("power")