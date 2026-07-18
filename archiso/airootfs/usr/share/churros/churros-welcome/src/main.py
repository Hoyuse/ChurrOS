import subprocess
import time

from window import ChurrOSWelcome


def main():
    while subprocess.run(
    ["/usr/bin/awww", "query"],
    stdout=subprocess.DEVNULL,
    stderr=subprocess.DEVNULL,
).returncode != 0:
    time.sleep(0.2)

    subprocess.Popen(
        [
            "/usr/bin/awww",
            "img",
            "/usr/share/churros/wallpapers/default.png",
            "--transition-type",
            "none",
        ],
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL,
    )

    app = ChurrOSWelcome()
    app.run()


if __name__ == "__main__":
    main()