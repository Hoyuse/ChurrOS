import subprocess


class NetworkService:

    @staticmethod
    def is_connected():

        try:

            output = subprocess.check_output(
                [
                    "nmcli",
                    "-t",
                    "-f",
                    "DEVICE,STATE",
                    "device"
                ],
                text=True
            )

            for line in output.splitlines():

                parts = line.split(":")

                if len(parts) >= 2 and parts[1] == "connected":

                    return True

            return False

        except Exception:

            return False

    @staticmethod
    def get_status():

        if NetworkService.is_connected():

            return "Connected"

        return "Offline"

    @staticmethod
    def get_name():

        try:

            output = subprocess.check_output(
                [
                    "nmcli",
                    "-t",
                    "-f",
                    "NAME,TYPE",
                    "connection",
                    "show",
                    "--active"
                ],
                text=True
            )

            for line in output.splitlines():

                name, conn_type = line.split(":")

                if conn_type == "802-3-ethernet":

                    return "Ethernet"

                if conn_type == "wifi":

                    return name

            return "Offline"

        except Exception:

            return "Offline"