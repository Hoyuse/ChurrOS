import subprocess

from services.settings import SettingsService


class ApplicationsService:

    @staticmethod
    def count():

        try:

            result = subprocess.run(

                ["pacman", "-Q"],

                capture_output=True,

                text=True,

                timeout=2

            )

            return str(

                len(

                    result.stdout.splitlines()

                )

            )

        except Exception:

            return "0"

    @staticmethod
    def store():

        return "Bazaar"

    @staticmethod
    def auto_updates():

        return SettingsService.get(

            "applications.auto_updates",

            True

        )

    @staticmethod
    def set_auto_updates(value):

        SettingsService.set(

            "applications.auto_updates",

            value

        )

    @staticmethod
    def auto_install():

        return SettingsService.get(

            "applications.auto_install",

            False

        )

    @staticmethod
    def set_auto_install(value):

        SettingsService.set(

            "applications.auto_install",

            value

        )

    @staticmethod
    def package_manager():

        return "pacman"

    @staticmethod
    def repositories():

        return "Arch Linux"

    @staticmethod
    def flatpak_enabled():

        try:

            result = subprocess.run(

                ["which", "flatpak"],

                capture_output=True,

                text=True,

                timeout=2

            )

            return result.returncode == 0

        except Exception:

            return False

    @staticmethod
    def snap_enabled():

        try:

            result = subprocess.run(

                ["which", "snap"],

                capture_output=True,

                text=True,

                timeout=2

            )

            return result.returncode == 0

        except Exception:

            return False
