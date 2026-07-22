import sys

from manager import PopupManager


def main():

    if len(sys.argv) != 2:

        print("Usage: popup_manager <popup>")

        return

    PopupManager.show(
        sys.argv[1]
    )


if __name__ == "__main__":

    main()