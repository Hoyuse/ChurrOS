from pathlib import Path
import subprocess

from process import PopupProcess


ROOT = Path(__file__).resolve().parent.parent


class Popup:

    @staticmethod
    def open(name):

        popup = ROOT / name / "main.py"

        process = subprocess.Popen(
            [
                "python3",
                str(popup)
            ]
        )

        PopupProcess.save(
            process.pid,
            name
        )