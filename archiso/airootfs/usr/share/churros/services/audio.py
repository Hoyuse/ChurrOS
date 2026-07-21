import subprocess


class AudioService:

    @staticmethod
    def get_volume():

        output = subprocess.check_output(
            ["wpctl", "get-volume", "@DEFAULT_AUDIO_SINK@"],
            text=True
        )

        # Volume: 0.75
        volume = float(output.split()[1])

        return int(volume * 100)

    @staticmethod
    def set_volume(value):

        subprocess.run(
            [
                "wpctl",
                "set-volume",
                "@DEFAULT_AUDIO_SINK@",
                f"{value}%"
            ]
        )