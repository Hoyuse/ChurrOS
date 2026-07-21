import subprocess


class WifiService:

    @staticmethod
    def get():

        info = {
            "available": False,
            "enabled": False,
            "connected": "",
            "networks": []
        }

        try:

            output = subprocess.check_output(
                [
                    "nmcli",
                    "-t",
                    "-f",
                    "DEVICE,TYPE,STATE",
                    "device"
                ],
                text=True
            )

            wifi_device = None

            for line in output.splitlines():

                if not line:
                    continue

                device, dev_type, state = line.split(":")

                if dev_type != "wifi":
                    continue

                wifi_device = device

                info["available"] = True

                info["enabled"] = (
                    state != "unavailable"
                )

                break

            if not wifi_device:

                return info

            output = subprocess.check_output(
                [
                    "nmcli",
                    "-t",
                    "-f",
                    "ACTIVE,SSID,SIGNAL",
                    "device",
                    "wifi",
                    "list"
                ],
                text=True
            )

            for line in output.splitlines():

                if not line:
                    continue

                active, ssid, signal = line.split(":")

                network = {
                    "ssid": ssid if ssid else "Hidden Network",
                    "signal": int(signal),
                    "connected": active == "yes"
                }

                if network["connected"]:

                    info["connected"] = network["ssid"]

                info["networks"].append(
                    network
                )

        except Exception:

            pass

        return info

    @staticmethod
    def enable():

        subprocess.run(
            [
                "nmcli",
                "radio",
                "wifi",
                "on"
            ]
        )

    @staticmethod
    def disable():

        subprocess.run(
            [
                "nmcli",
                "radio",
                "wifi",
                "off"
            ]
        )

    @staticmethod
    def toggle():

        if WifiService.get()["enabled"]:

            WifiService.disable()

        else:

            WifiService.enable()