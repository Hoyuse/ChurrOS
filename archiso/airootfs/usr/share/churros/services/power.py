import subprocess


class PowerService:

    @staticmethod
    def lock():

        subprocess.run(
            [
                "loginctl",
                "lock-session"
            ]
        )

    @staticmethod
    def logout():

        subprocess.run(
            [
                "hyprctl",
                "dispatch",
                "exit"
            ]
        )

    @staticmethod
    def suspend():

        subprocess.run(
            [
                "systemctl",
                "suspend"
            ]
        )

    @staticmethod
    def hibernate():

        subprocess.run(
            [
                "systemctl",
                "hibernate"
            ]
        )

    @staticmethod
    def restart():

        subprocess.run(
            [
                "systemctl",
                "reboot"
            ]
        )

    @staticmethod
    def shutdown():

        subprocess.run(
            [
                "systemctl",
                "poweroff"
            ]
        )