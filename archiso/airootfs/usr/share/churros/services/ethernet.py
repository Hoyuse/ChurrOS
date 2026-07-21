import subprocess


class EthernetService:

    @staticmethod
    def get():

        info = {
            "available": False,
            "connected": False,
            "interface": "",
            "connection": ""
        }

        try:

            output = subprocess.check_output(
                [
                    "nmcli",
                    "-t",
                    "-f",
                    "DEVICE,TYPE,STATE,CONNECTION",
                    "device"
                ],
                text=True
            )

            for line in output.splitlines():

                if not line:
                    continue

                device, dev_type, state, connection = line.split(":", 3)

                if dev_type != "ethernet":
                    continue

                info["available"] = True
                info["interface"] = device
                info["connection"] = connection

                info["connected"] = (
                    state in (
                        "connected",
                        "conectado"
                    )
                )

                break

        except Exception:

            pass

        return info