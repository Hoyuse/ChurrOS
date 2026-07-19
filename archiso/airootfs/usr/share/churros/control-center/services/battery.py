import subprocess


class BatteryService:

    DEVICE = "/org/freedesktop/UPower/devices/DisplayDevice"

    @staticmethod
    def _info():

        try:

            return subprocess.check_output(
                [
                    "upower",
                    "-i",
                    BatteryService.DEVICE
                ],
                text=True
            )

        except Exception:

            return ""

    @staticmethod
    def has_battery():

        info = BatteryService._info()

        return "power supply:         yes" in info

    @staticmethod
    def get_percentage():

        if not BatteryService.has_battery():

            return "--"

        info = BatteryService._info()

        for line in info.splitlines():

            if "percentage:" in line:

                return line.split(":")[1].strip()

        return "--"

    @staticmethod
    def get_state():

        if not BatteryService.has_battery():

            return "Desktop PC"

        info = BatteryService._info()

        for line in info.splitlines():

            if "state:" in line:

                return line.split(":")[1].strip().capitalize()

        return "Unknown"

    @staticmethod
    def get_icon():

        if not BatteryService.has_battery():

            return "battery-missing-symbolic"

        percentage = BatteryService.get_percentage().replace("%", "")

        try:

            value = int(percentage)

        except ValueError:

            return "battery-symbolic"

        if value >= 90:
            return "battery-full-symbolic"

        if value >= 60:
            return "battery-good-symbolic"

        if value >= 30:
            return "battery-medium-symbolic"

        if value >= 10:
            return "battery-low-symbolic"

        return "battery-caution-symbolic"