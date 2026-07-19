import subprocess


class BluetoothService:

    @staticmethod
    def has_adapter():

        try:

            result = subprocess.run(
                [
                    "systemctl",
                    "is-active",
                    "bluetooth"
                ],
                capture_output=True,
                text=True,
                timeout=2
            )

            return result.stdout.strip() == "active"

        except Exception:

            return False

    @staticmethod
    def get_status():

        if BluetoothService.has_adapter():

            return "On"

        return "No adapter"

    @staticmethod
    def get_description():

        if BluetoothService.has_adapter():

            return "Bluetooth available"

        return "No Bluetooth hardware"

    @staticmethod
    def get_icon():

        if BluetoothService.has_adapter():

            return "bluetooth.svg"

        return "bluetooth_disabled.svg"