#!/usr/bin/env python3
"""Ensure the spoken Live boot path has its screen-reader service installed."""
from pathlib import Path
import unittest

ROOT = Path(__file__).resolve().parents[1]
MANIFESTS = (
    "archiso/packages.x86_64",
    "archiso/packages.kde.x86_64",
    "archiso/packages.server.x86_64",
    "archiso/packages.xfce.x86_64",
)


class AccessibilityPackagesTests(unittest.TestCase):
    def test_espeakup_is_installed_for_every_live_edition(self):
        for relative in MANIFESTS:
            with self.subTest(manifest=relative):
                packages = {
                    line.split("#", 1)[0].strip()
                    for line in (ROOT / relative).read_text(encoding="utf-8").splitlines()
                }
                self.assertIn("espeakup", packages)

    def test_accessibility_service_starts_espeakup(self):
        service = (ROOT / "archiso/airootfs/etc/systemd/system/livecd-talk.service").read_text(
            encoding="utf-8"
        )
        self.assertIn("ConditionKernelCommandLine=accessibility=on", service)
        self.assertIn("systemctl start espeakup.service", service)


if __name__ == "__main__":
    unittest.main(verbosity=2)
