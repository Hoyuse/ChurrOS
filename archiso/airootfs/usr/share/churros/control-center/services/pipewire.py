import subprocess


class PipeWireService:

    @staticmethod
    def get_volume():

        try:

            output = subprocess.check_output(
                [
                    "wpctl",
                    "get-volume",
                    "@DEFAULT_AUDIO_SINK@"
                ],
                text=True
            ).strip()

            volume = float(output.split()[1])

            return int(volume * 100)

        except Exception:

            return 0

    @staticmethod
    def is_muted():

        try:

            output = subprocess.check_output(
                [
                    "wpctl",
                    "get-volume",
                    "@DEFAULT_AUDIO_SINK@"
                ],
                text=True
            ).strip()

            return "MUTED" in output

        except Exception:

            return False

    @staticmethod
    def get_icon():

        if PipeWireService.is_muted():

            return "audio_muted.svg"

        return "audio.svg"

    @staticmethod
    def set_volume(volume):

        try:

            subprocess.run(
                [
                    "wpctl",
                    "set-volume",
                    "@DEFAULT_AUDIO_SINK@",
                    f"{volume}%"
                ],
                check=False
            )

        except Exception:

            pass