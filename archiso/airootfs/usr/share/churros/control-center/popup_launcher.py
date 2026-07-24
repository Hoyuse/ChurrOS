import os
import sys
import subprocess


BASE_DIR = os.path.abspath(
    os.path.join(
        os.path.dirname(__file__),
        ".."
    )
)


def _open(name, window=None):

    if window is not None:
        try:
            window.close()
        except Exception:
            pass

    popup = os.path.join(
        BASE_DIR,
        "popups",
        name,
        "main.py"
    )

    subprocess.Popen(
        [
            sys.executable,
            popup
        ]
    )


def open_network(window=None):
    _open("network", window)


def open_audio(window=None):
    _open("audio", window)


def open_bluetooth(window=None):
    _open("bluetooth", window)


def open_brightness(window=None):
    _open("brightness", window)


def open_battery(window=None):
    _open("battery", window)


def open_power(window=None):
    _open("power", window)