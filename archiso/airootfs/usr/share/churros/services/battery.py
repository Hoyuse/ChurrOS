import subprocess


class BatteryService:

    @staticmethod
    def get():

        devices = subprocess.check_output(
            ["upower", "-e"],
            text=True
        ).splitlines()

        battery = None

        for device in devices:

            if "battery" in device.lower():

                battery = device
                break

        if battery is None:

            return {
                "available": False
            }

        output = subprocess.check_output(
            ["upower", "-i", battery],
            text=True
        )

        info = {
            "available": True,
            "percentage": 0,
            "state": "unknown",
            "time": "",
            "icon": "󰂎"
        }

        for line in output.splitlines():

            line = line.strip()

            if line.startswith("state:"):

                info["state"] = line.split(":", 1)[1].strip()

            elif line.startswith("percentage:"):

                percentage = int(
                    line.split(":", 1)[1]
                    .replace("%", "")
                    .strip()
                )

                info["percentage"] = percentage

            elif line.startswith("time to full:"):

                info["time"] = line.split(":", 1)[1].strip()

            elif line.startswith("time to empty:"):

                info["time"] = line.split(":", 1)[1].strip()

        p = info["percentage"]

        if p >= 95:
            info["icon"] = "󰁹"

        elif p >= 80:
            info["icon"] = "󰂂"

        elif p >= 60:
            info["icon"] = "󰂀"

        elif p >= 40:
            info["icon"] = "󰁾"

        elif p >= 20:
            info["icon"] = "󰁼"

        else:
            info["icon"] = "󰂎"

        return info