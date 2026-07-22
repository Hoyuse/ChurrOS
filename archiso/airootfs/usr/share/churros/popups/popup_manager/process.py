from pathlib import Path
import os
import signal


RUNTIME = Path("/tmp/churros")

PID_FILE = RUNTIME / "popup.pid"
NAME_FILE = RUNTIME / "popup.name"


class PopupProcess:

    @staticmethod
    def init():

        RUNTIME.mkdir(
            parents=True,
            exist_ok=True
        )

    @staticmethod
    def running():

        PopupProcess.init()

        if not PID_FILE.exists():

            return False

        try:

            pid = int(
                PID_FILE.read_text()
            )

            os.kill(
                pid,
                0
            )

            return True

        except Exception:

            PopupProcess.clear()

            return False

    @staticmethod
    def pid():

        PopupProcess.init()

        if not PID_FILE.exists():

            return None

        return int(
            PID_FILE.read_text()
        )

    @staticmethod
    def name():

        PopupProcess.init()

        if not NAME_FILE.exists():

            return None

        return NAME_FILE.read_text().strip()

    @staticmethod
    def save(pid, name):

        PopupProcess.init()

        PID_FILE.write_text(
            str(pid)
        )

        NAME_FILE.write_text(
            name
        )

    @staticmethod
    def kill():

        if not PopupProcess.running():

            return

        try:

            os.kill(
                PopupProcess.pid(),
                signal.SIGTERM
            )

        except Exception:

            pass

        PopupProcess.clear()

    @staticmethod
    def clear():

        PID_FILE.unlink(
            missing_ok=True
        )

        NAME_FILE.unlink(
            missing_ok=True
        )