#!/usr/bin/env python3
"""Regression tests for required AUR packages in partial local caches."""
from pathlib import Path
import subprocess
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[1]
HELPER = ROOT / "scripts/list-missing-aur-packages.sh"
REQUIRED = ("python-pywal", "yay", "wlogout", "noctalia-qs", "noctalia-shell")


class AurPackageTriggerTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory(prefix="churros-aur-cache-test-")
        self.addCleanup(self.temp.cleanup)
        self.package_dir = Path(self.temp.name)
        for package in REQUIRED:
            (self.package_dir / f"{package}-1.0-1-x86_64.pkg.tar.zst").touch()

    def missing(self):
        return subprocess.run(
            ["bash", str(HELPER), str(self.package_dir)],
            text=True,
            capture_output=True,
            check=True,
        ).stdout.splitlines()

    def test_complete_cache_needs_no_build(self):
        self.assertEqual(self.missing(), [])

    def test_missing_noctalia_packages_trigger_preparation(self):
        (self.package_dir / "noctalia-shell-1.0-1-x86_64.pkg.tar.zst").unlink()
        (self.package_dir / "noctalia-qs-1.0-1-x86_64.pkg.tar.zst").unlink()
        self.assertEqual(self.missing(), ["noctalia-qs", "noctalia-shell"])

    def test_build_script_uses_the_complete_required_package_list(self):
        build = (ROOT / "scripts/cli/build.sh").read_text(encoding="utf-8")
        helper = (ROOT / "scripts/list-missing-aur-packages.sh").read_text(encoding="utf-8")
        self.assertIn("list-missing-aur-packages.sh archiso/packages", build)
        self.assertIn('if [ -n "$AUR_MISSING" ]', build)
        self.assertIn("bash scripts/build-aur.sh", build)
        for package in REQUIRED:
            self.assertIn(package, helper)


if __name__ == "__main__":
    unittest.main(verbosity=2)
