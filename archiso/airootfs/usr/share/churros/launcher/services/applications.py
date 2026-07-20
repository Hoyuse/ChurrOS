import gi

gi.require_version("Gio", "2.0")

from gi.repository import Gio


class ApplicationsService:

    @staticmethod
    def get_applications():

        apps = []

        for app in Gio.AppInfo.get_all():

            if not app.should_show():

                continue

            apps.append(
                {
                    "name": app.get_display_name(),
                    "icon": app.get_icon(),
                    "app": app
                }
            )

        apps.sort(
            key=lambda x: x["name"].lower()
        )

        return apps