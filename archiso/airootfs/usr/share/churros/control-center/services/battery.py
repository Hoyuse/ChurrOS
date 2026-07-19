import subprocess


class BatteryService:

    @staticmethod
    def has_battery():

        try:

            devices = subprocess.check_output(
                ["upower", "-e"],
                text=True
            )

            return "DisplayDevice" in devices

        except Exception:

            return False

    @staticmethod
    def get_percentage():

        if not BatteryService.has_battery():

            return None

        try:

            output = subprocess.check_output(
                [
                    "upower",
                    "-i",
                    "/org/freedesktop/UPower/devices/DisplayDevice"
                ],
                text=True
            )

            for line in output.splitlines():

                if "percentage:" in line:

                    return line.split(":")[1].strip()

            return "--"

        except Exception:

            return "--"

    @staticmethod
    def get_state():

        if not BatteryService.has_battery():

            return "Desktop PC"

        try:

            output = subprocess.check_output(
                [
                    "upower",
                    "-i",
                    "/org/freedesktop/UPower/devices/DisplayDevice"
                ],
                text=True
            )

            for line in output.splitlines():

                if "state:" in line:

                    return line.split(":")[1].strip().capitalize()

            return "Unknown"

        except Exception:

            return "Unknown"