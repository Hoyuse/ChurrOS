from pathlib import Path
import importlib.util


SHARED_SERVICES = Path(__file__).resolve().parents[2] / "services"


def load_service(name, class_name):

    spec = importlib.util.spec_from_file_location(
        f"churros_shared_{name}",
        SHARED_SERVICES / f"{name}.py"
    )

    module = importlib.util.module_from_spec(spec)

    spec.loader.exec_module(module)

    return getattr(module, class_name)


EthernetService = load_service(
    "ethernet",
    "EthernetService"
)

WifiService = load_service(
    "wifi",
    "WifiService"
)


class NetworkService:

    @staticmethod
    def get():

        ethernet = EthernetService.get()

        if ethernet["connected"]:

            return {
                "type": "ethernet",
                "icon": "ethernet.svg",
                "status": "Connected",
                "name": "Ethernet",
                "signal": None
            }

        wifi = WifiService.get()

        for network in wifi["networks"]:

            if network["connected"]:

                return {
                    "type": "wifi",
                    "icon": "wifi.svg",
                    "status": "Connected",
                    "name": network["ssid"],
                    "signal": network["signal"]
                }

        return {
            "type": "offline",
            "icon": "ethernet.svg",
            "status": "Offline",
            "name": "Offline",
            "signal": None
        }

    @staticmethod
    def is_connected():

        return NetworkService.get()["type"] != "offline"

    @staticmethod
    def get_status():

        return NetworkService.get()["status"]

    @staticmethod
    def get_name():

        return NetworkService.get()["name"]

    @staticmethod
    def get_icon():

        return NetworkService.get()["icon"]
