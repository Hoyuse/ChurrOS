import subprocess


class WifiService:

    @staticmethod
    def _run(command):

        result = subprocess.run(
            command,
            capture_output=True,
            text=True
        )

        return (
            result.returncode,
            result.stdout.strip(),
            result.stderr.strip()
        )

    @staticmethod
    def available():

        code, out, _ = WifiService._run(
            [
                "nmcli",
                "-t",
                "-f",
                "DEVICE,TYPE",
                "device"
            ]
        )

        if code != 0:
            return False

        for line in out.splitlines():

            try:

                _, dev_type = line.split(":")

                if dev_type == "wifi":
                    return True

            except ValueError:
                continue

        return False

    @staticmethod
    def enabled():

        code, out, _ = WifiService._run(
            [
                "nmcli",
                "radio",
                "wifi"
            ]
        )

        if code != 0:
            return False

        return out.lower() == "enabled"

    @staticmethod
    def scan():

        WifiService._run(
            [
                "nmcli",
                "device",
                "wifi",
                "rescan"
            ]
        )

    @staticmethod
    def get():

        data = {

            "available": WifiService.available(),

            "enabled": WifiService.enabled(),

            "connected": None,

            "networks": []

        }

        if not data["available"]:

            return data

        if not data["enabled"]:

            return data

        code, out, _ = WifiService._run(
            [
                "nmcli",
                "-t",
                "-f",
                "ACTIVE,SSID,SIGNAL,SECURITY",
                "device",
                "wifi",
                "list"
            ]
        )

        if code != 0:

            return data

        saved = WifiService.saved_networks()

        for line in out.splitlines():

            if not line:

                continue

            parts = line.split(":")

            while len(parts) < 4:

                parts.append("")

            active, ssid, signal, security = parts[:4]

            network = {

                "ssid": ssid if ssid else "Hidden Network",

                "signal": int(signal) if signal.isdigit() else 0,

                "security": security,

                "connected": active == "yes",

                "saved": ssid in saved

            }

            if network["connected"]:

                data["connected"] = network["ssid"]

            data["networks"].append(
                network
            )

        data["networks"].sort(

            key=lambda n: (

                not n["connected"],

                not n["saved"],

                -n["signal"]

            )

        )

        return data

    @staticmethod
    def saved_networks():

        code, out, _ = WifiService._run(
            [
                "nmcli",
                "-t",
                "-f",
                "NAME,TYPE",
                "connection",
                "show"
            ]
        )

        saved = set()

        if code != 0:

            return saved

        for line in out.splitlines():

            try:

                name, conn_type = line.split(":")

                if conn_type == "802-11-wireless":

                    saved.add(name)

            except ValueError:

                pass

        return saved

    @staticmethod
    def connect(ssid, password=None):

        command = [

            "nmcli",

            "device",

            "wifi",

            "connect",

            ssid

        ]

        if password:

            command.extend(

                [

                    "password",

                    password

                ]

            )

        code, _, err = WifiService._run(
            command
        )

        if code == 0:

            return True, ""

        err = err.lower()

        if "secrets were required" in err:

            return False, "Password required."

        if "invalid" in err:

            return False, "Incorrect password."

        if "activation" in err:

            return False, "Unable to connect."

        return False, "Unknown error."

    @staticmethod
    def disconnect():

        code, out, _ = WifiService._run(

            [

                "nmcli",

                "-t",

                "-f",

                "DEVICE,TYPE",

                "device"

            ]

        )

        if code != 0:

            return

        for line in out.splitlines():

            device, dev_type = line.split(":")

            if dev_type == "wifi":

                WifiService._run(

                    [

                        "nmcli",

                        "device",

                        "disconnect",

                        device

                    ]

                )

                break

    @staticmethod
    def forget(ssid):

        WifiService._run(

            [

                "nmcli",

                "connection",

                "delete",

                ssid

            ]

        )

    @staticmethod
    def enable():

        WifiService._run(

            [

                "nmcli",

                "radio",

                "wifi",

                "on"

            ]

        )

    @staticmethod
    def disable():

        WifiService._run(

            [

                "nmcli",

                "radio",

                "wifi",

                "off"

            ]

        )

    @staticmethod
    def toggle():

        if WifiService.enabled():

            WifiService.disable()

        else:

            WifiService.enable()