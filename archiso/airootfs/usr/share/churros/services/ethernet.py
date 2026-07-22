import subprocess


class EthernetService:

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
    def get():

        info = {

            "available": False,

            "device": None,

            "connected": False,

            "connection": "",

            "speed": None,

            "ip": None

        }

        code, out, _ = EthernetService._run(

            [

                "nmcli",

                "-t",

                "-f",

                "DEVICE,TYPE,STATE,CONNECTION",

                "device"

            ]

        )

        if code != 0:

            return info

        for line in out.splitlines():

            parts = line.split(":")

            while len(parts) < 4:

                parts.append("")

            device, dev_type, state, connection = parts[:4]

            if dev_type != "ethernet":

                continue

            info["available"] = True

            info["device"] = device

            info["connected"] = state == "connected"

            info["connection"] = connection

            break

        if not info["available"]:

            return info

        if info["connected"]:

            info["speed"] = EthernetService.speed(
                info["device"]
            )

            info["ip"] = EthernetService.ip(
                info["device"]
            )

        return info

    @staticmethod
    def speed(device):

        code, out, _ = EthernetService._run(

            [

                "cat",

                f"/sys/class/net/{device}/speed"

            ]

        )

        if code != 0:

            return None

        try:

            return int(out)

        except Exception:

            return None

    @staticmethod
    def ip(device):

        code, out, _ = EthernetService._run(

            [

                "ip",

                "-4",

                "addr",

                "show",

                device

            ]

        )

        if code != 0:

            return None

        for line in out.splitlines():

            line = line.strip()

            if line.startswith("inet"):

                return line.split()[1]

        return None

    @staticmethod
    def disconnect():

        info = EthernetService.get()

        if not info["available"]:

            return

        EthernetService._run(

            [

                "nmcli",

                "device",

                "disconnect",

                info["device"]

            ]

        )

    @staticmethod
    def connect():

        info = EthernetService.get()

        if not info["available"]:

            return

        EthernetService._run(

            [

                "nmcli",

                "device",

                "connect",

                info["device"]

            ]

        )