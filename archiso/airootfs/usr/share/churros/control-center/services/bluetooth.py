import subprocess


class BluetoothService:

    @staticmethod
    def is_enabled():

        try:

            result = subprocess.run(
                [
                    "systemctl",
                    "is-active",
                    "bluetooth"
                ],
                capture_output=True,
                text=True
            )

            return result.stdout.strip() == "active"

        except Exception:

            return False

    @staticmethod
    def get_status():

        if BluetoothService.is_enabled():

            return "On"

        return "Off"

    @staticmethod
    def get_description():

        if BluetoothService.is_enabled():

            return "Bluetooth enabled"

        return "Bluetooth disabled"

    @staticmethod
    def get_icon():

        if BluetoothService.is_enabled():

            return "bluetooth.svg"

        return "bluetooth_disabled.svg"