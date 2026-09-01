import importlib.util
import subprocess
import unittest
from pathlib import Path
from unittest.mock import patch


ROOT = Path(__file__).resolve().parents[1]
SERVICES = ROOT / "archiso/airootfs/usr/share/churros/services"


def load_service(filename):
    path = SERVICES / filename
    spec = importlib.util.spec_from_file_location(filename, path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


wifi = load_service("wifi.py")
ethernet = load_service("ethernet.py")
battery = load_service("battery.py")


class NetworkParsingTests(unittest.TestCase):
    def test_wifi_parser_unescapes_colons_and_backslashes(self):
        self.assertEqual(
            wifi.WifiService._split_nmcli(r"yes:Casa\:5GHz:80:WPA2"),
            ["yes", "Casa:5GHz", "80", "WPA2"],
        )

    def test_ethernet_parser_unescapes_connection_name(self):
        self.assertEqual(
            ethernet.EthernetService._split_nmcli(r"enp0s3:ethernet:connected:Oficina\:LAN"),
            ["enp0s3", "ethernet", "connected", "Oficina:LAN"],
        )

    def test_missing_network_manager_is_reported_as_unavailable(self):
        with patch("subprocess.run", side_effect=FileNotFoundError()):
            self.assertFalse(wifi.WifiService.available())
            self.assertFalse(ethernet.EthernetService.get()["available"])


class BatteryTests(unittest.TestCase):
    def test_missing_upower_does_not_crash_launcher(self):
        with patch("subprocess.check_output", side_effect=FileNotFoundError()):
            self.assertEqual(battery.BatteryService.get(), {"available": False})


if __name__ == "__main__":
    unittest.main()
