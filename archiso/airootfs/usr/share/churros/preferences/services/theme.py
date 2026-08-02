import os
import shutil
import subprocess

from services.settings import SettingsService
from services.wallpaper import WallpaperService


CACHE_DIR = os.path.expanduser("~/.cache/churros-theme")

DARK_WALLPAPER = "/usr/share/churros/wallpapers/fondo1.png"
LIGHT_WALLPAPER = "/usr/share/churros/wallpapers/default.jpeg"


def _build_env():

    env = os.environ.copy()
    if not env.get("WAYLAND_DISPLAY"):
        xrd = env.get("XDG_RUNTIME_DIR", f"/run/user/{os.getuid()}")
        if os.path.isdir(xrd):
            for sock in sorted(os.listdir(xrd)):
                if sock.startswith("wayland-"):
                    env["WAYLAND_DISPLAY"] = sock
                    break
    if not env.get("XDG_RUNTIME_DIR"):
        env["XDG_RUNTIME_DIR"] = f"/run/user/{os.getuid()}"
    return env


def _write_gtk_settings(dark):

    os.makedirs(CACHE_DIR, exist_ok=True)

    with open(os.path.join(CACHE_DIR, "dark-flag"), "w") as f:
        f.write(str(int(dark)))

    icon_theme = SettingsService.get(
        "icons.theme", "Papirus-Dark" if dark else "Papirus",
    )

    home = os.path.expanduser("~")

    gtk3_dir = os.path.join(home, ".config", "gtk-3.0")
    os.makedirs(gtk3_dir, exist_ok=True)
    with open(os.path.join(gtk3_dir, "settings.ini"), "w") as f:
        f.write("[Settings]\n")
        f.write("gtk-theme-name={}\n".format("Adwaita-dark" if dark else "Adwaita"))
        f.write("gtk-application-prefer-dark-theme={}\n".format(int(dark)))
        f.write("gtk-icon-theme-name={}\n".format(icon_theme))

    gtk4_dir = os.path.join(home, ".config", "gtk-4.0")
    os.makedirs(gtk4_dir, exist_ok=True)
    with open(os.path.join(gtk4_dir, "settings.ini"), "w") as f:
        f.write("[Settings]\n")
        f.write("gtk-theme-name={}\n".format("Adwaita-dark" if dark else "Adwaita"))
        f.write("gtk-application-prefer-dark-theme={}\n".format(int(dark)))
        f.write("gtk-icon-theme-name={}\n".format(icon_theme))


class ThemeService:

    @classmethod
    def is_dark(cls):

        flag = os.path.join(CACHE_DIR, "dark-flag")
        if os.path.isfile(flag):
            try:
                with open(flag) as f:
                    return f.read().strip() == "1"
            except Exception:
                pass

        cached = SettingsService.get("theme.dark", None)
        if cached is not None:
            return bool(cached)

        return True

    @classmethod
    def set(cls, dark):
        SettingsService.set("theme.dark", bool(dark))
        _write_gtk_settings(dark)

        if dark:
            wp = DARK_WALLPAPER
        else:
            wp = LIGHT_WALLPAPER

        if os.path.isfile(wp):
            SettingsService.set("wallpaper.path", wp)
            try:
                WallpaperService.apply(wp)
            except Exception as e:
                print("[theme] wallpaper apply fallo:", e, flush=True)

        env = _build_env()

        for sig_target in (
            ["pkill", "-SIGUSR1", "waybar"],
        ):
            try:
                subprocess.run(
                    sig_target,
                    capture_output=True,
                    timeout=2,
                    env=env,
                )
            except Exception:
                pass

    @classmethod
    def toggle(cls):
        cls.set(not cls.is_dark())

    @classmethod
    def ensure(cls):
        cls.set(cls.is_dark())

    @classmethod
    def apply(cls):
        cls.set(cls.is_dark())