#!/usr/bin/env python3
"""Regression test for Calamares' visible and executable module sequence."""
from pathlib import Path
import unittest

ROOT = Path(__file__).resolve().parents[1]


def sequence_stages():
    settings = (ROOT / "installer/calamares/settings.conf").read_text(encoding="utf-8")
    stages = []
    in_sequence = False
    for line in settings.splitlines():
        if line == "sequence:":
            in_sequence = True
            continue
        if not in_sequence:
            continue
        if line and not line.startswith(" "):
            break
        if line.startswith("  - "):
            stages.append((line.strip()[2:].removesuffix(":"), []))
        elif line.startswith("      - ") and stages:
            stages[-1][1].append(line.strip()[2:])
    return stages


class CalamaresSequenceTests(unittest.TestCase):
    def test_optional_packages_are_visible_and_applied(self):
        stages = sequence_stages()
        initial_show = next(
            (modules for name, modules in stages if name == "show" and "finished" not in modules),
            [],
        )
        execute = next((modules for name, modules in stages if name == "exec"), [])

        self.assertIn("netinstall", initial_show,
                      "the package-selection page must be shown before installation")
        self.assertIn("netinstall", execute,
                      "selected packages must be installed during the exec phase")
        self.assertIn("packages", execute,
                      "the configured package cleanup operations must run")
        self.assertLess(execute.index("netinstall"), execute.index("packages"),
                        "install selected packages before removing Live-only packages")

        modules = ROOT / "installer/calamares/modules"
        self.assertTrue((modules / "netinstall.conf").is_file())
        self.assertTrue((modules / "netinstall.yaml").is_file())
        self.assertTrue((modules / "packages.conf").is_file())


if __name__ == "__main__":
    unittest.main(verbosity=2)
