import subprocess


class NetworkService:

    @staticmethod
    def _active_connection():

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

                parts = line.split(":")

                if len(parts) >= 2:

                    return parts[0], parts[1]

            return None, None

        except Exception:

            return None, None

    @staticmethod
    def is_connected():

        _, conn_type = NetworkService._active_connection()

        return conn_type is not None

    @staticmethod
    def get_status():

        if NetworkService.is_connected():

            return "Connected"

        return "Offline"

    @staticmethod
    def get_name():

        name, conn_type = NetworkService._active_connection()

        if conn_type == "802-3-ethernet":

            return "Ethernet"

        if conn_type == "wifi":

            return name

        return "Offline"

    @staticmethod
    def get_icon():

        _, conn_type = NetworkService._active_connection()

        if conn_type == "wifi":

            return "wifi.svg"

        if conn_type == "802-3-ethernet":

            return "ethernet.svg"

        return "ethernet.svg"