import subprocess


def launch_application(command: str):

    subprocess.Popen([command])


def open_terminal():

    launch_application("kitty")


def open_browser():

    launch_application("firefox")


def launch_installer():

    subprocess.Popen(["sudo", "-E", "calamares", "-d"])