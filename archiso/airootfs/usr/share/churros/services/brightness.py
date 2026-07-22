import os
import subprocess


class BrightnessService:

    @staticmethod
    def available():

        try:

            devices = os.listdir("/sys/class/backlight")

            return len(devices) > 0

        except Exception:

            return False

    @staticmethod
    def get():

        if not BrightnessService.available():

            return {
                "available": False,
                "brightness": 100
            }

        try:

            current = int(
                subprocess.check_output(
                    ["brightnessctl", "g"],
                    text=True
                ).strip()
            )

            maximum = int(
                subprocess.check_output(
                    ["brightnessctl", "m"],
                    text=True
                ).strip()
            )

            brightness = int(
                current * 100 / maximum
            )

            return {
                "available": True,
                "brightness": brightness
            }

        except Exception:

            return {
                "available": False,
                "brightness": 100
            }

    @staticmethod
    def set(value):

        if not BrightnessService.available():

            return

        subprocess.run(
            [
                "brightnessctl",
                "set",
                f"{value}%"
            ]
        )