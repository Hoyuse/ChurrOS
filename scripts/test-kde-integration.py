#!/usr/bin/env python3
"""Isolated KDE regression checks. Never writes the host's /etc or session.

Run with python3 scripts/test-kde-integration.py. kwriteconfig6 and rustc are
optional: their integration tests report SKIP when unavailable.
"""
import configparser
import json
import os
from pathlib import Path
import shutil
import subprocess
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[1]
OVERLAY = ROOT / "archiso/airootfs"


class KdeIntegration(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory(prefix="churros-kde-test-")
        self.addCleanup(self.temp.cleanup)
        self.home = Path(self.temp.name)
        self.env = {**os.environ, "HOME": str(self.home),
                    "XDG_CONFIG_HOME": str(self.home / "config")}

    def run_command(self, args, **kwargs):
        return subprocess.run(args, env=self.env, check=True, text=True,
                              capture_output=True, **kwargs)

    def executable(self, name, source):
        path = self.home / name
        path.write_text(source)
        path.chmod(0o755)
        return path

    def test_panel_missing_config(self):
        self.run_command(["bash", str(OVERLAY / "usr/share/churros/scripts/configure-kde-panel")])
        self.assertFalse((self.home / "config").exists())

    @unittest.skipUnless(shutil.which("kwriteconfig6"), "kwriteconfig6 unavailable")
    def test_panel_multiple_containments_and_repeat(self):
        config = self.home / "config/plasma-org.kde.plasma.desktop-appletsrc"
        config.parent.mkdir()
        config.write_text(
            "[Containments][42][Applets][7]\nplugin=org.kde.plasma.kickoff\n"
            "[Containments][99][Applets][8]\nplugin=org.kde.plasma.kickoff\n"
            "[Containments][99][Applets][9]\nplugin=org.kde.plasma.icontasks\n"
            "[Containments][99][Applets][9][Configuration][General]\nicon=keep-me\n")
        script = OVERLAY / "usr/share/churros/scripts/configure-kde-panel"
        self.run_command(["bash", str(script)])
        before = config.read_text()
        self.run_command(["bash", str(script)])
        self.assertEqual(config.read_text(), before)
        parsed = configparser.ConfigParser()
        parsed.read(config)
        for containment, applet in ((42, 7), (99, 8)):
            section = f"Containments][{containment}][Applets][{applet}][Configuration][General"
            self.assertEqual(parsed[section]["icon"], "churros-logo")
            self.assertEqual(parsed[section]["useCustomButtonImage"], "true")
        self.assertEqual(parsed["Containments][99][Applets][9][Configuration][General"]["icon"], "keep-me")
        self.assertNotIn("[Containments][1]", before)

    def test_systemsettings_dispatch_preserves_arguments(self):
        # Only replace absolute paths in a disposable copy of the real wrapper.
        edition_file = self.home / "edition"
        native = self.executable("native", '#!/bin/sh\nprintf "native:%s\\n" "$*"\n')
        churros = self.executable("churros", '#!/bin/sh\nprintf "churros:%s\\n" "$*"\n')
        source = (OVERLAY / "usr/local/bin/systemsettings").read_text()
        source = source.replace("/etc/churros-edition", str(edition_file))
        source = source.replace("/usr/bin/systemsettings", str(native))
        source = source.replace("/usr/bin/churros-settings", str(churros))
        wrapper = self.executable("wrapper", source)
        for edition in ("kde", "niri", "xfce", "server"):
            edition_file.write_text(edition + "\n")
            output = self.run_command(["bash", str(wrapper), "kcm_kscreen"]).stdout.strip()
            self.assertEqual(output, ("native" if edition == "kde" else "churros") + ":kcm_kscreen")

    def test_calamares_preserves_native_kde_binary(self):
        lines = (ROOT / "installer/calamares/modules/shellprocess-cleanup.conf").read_text().splitlines()
        command = next(json.loads(line.split("command: ", 1)[1]) for line in lines
                       if "ln -sf /usr/bin/churros-settings /usr/bin/systemsettings" in line)
        command = command.removeprefix("-")  # Calamares permits a nonzero exit.
        edition = self.home / "edition"
        native = self.home / "systemsettings"
        churros = self.executable("churros-settings", "#!/bin/sh\nexit 0\n")
        command = command.replace("/etc/churros-edition", str(edition))
        command = command.replace("/usr/bin/systemsettings", str(native))
        command = command.replace("/usr/bin/churros-settings", str(churros))
        edition.write_text("kde\n")
        native.write_text("original KDE binary")
        subprocess.run(["bash", "-c", command], env=self.env, capture_output=True)
        self.assertFalse(native.is_symlink())
        self.assertEqual(native.read_text(), "original KDE binary")
        for name in ("niri", "xfce", "server"):
            edition.write_text(name + "\n")
            self.run_command(["bash", "-c", command])
            self.assertTrue(native.is_symlink())

    def test_portal_kde_selection(self):
        config = configparser.ConfigParser()
        config.read(OVERLAY / "etc/xdg/xdg-desktop-portal/kde-portals.conf")
        self.assertEqual(config["preferred"]["default"], "kde")
        self.assertEqual(config["preferred"]["org.freedesktop.impl.portal.Secret"], "kwallet")
        self.assertEqual(config["preferred"]["org.freedesktop.impl.portal.Notification"], "plasmanotify")

    @unittest.skipUnless(shutil.which("rustc"), "rustc unavailable")
    def test_autologin_uses_edition_session(self):
        # Compile the complete UsersService with a stand-in for edition lookup.
        # Redirect its /etc paths and helpers to this disposable tree; preserve
        # the method's control flow and resulting TOML.
        source = (ROOT / "rust/preferences/src/services/users.rs").read_text()
        source = source.replace("/etc/greetd", str(self.home / "greetd"))
        source = source.replace("/etc/lightdm", str(self.home / "lightdm"))
        self.executable("churros-pkexec", '#!/bin/sh\nexec "$@"\n')
        self.executable("churros-write-root-config", '#!/bin/sh\ncat > "$1"\n')
        self.env["PATH"] = str(self.home) + os.pathsep + os.environ["PATH"]
        harness = self.home / "users.rs"
        harness.write_text(source + '''
mod churros_services { pub mod version {
    pub fn edition() -> String { std::env::var("TEST_EDITION").unwrap() }
}}
fn main() {
    assert!(UsersService::set_auto_login(true));
    assert!(UsersService::auto_login());
    assert!(UsersService::set_auto_login(false));
    assert!(!UsersService::auto_login());
    assert!(UsersService::set_auto_login(true));
}
''')
        binary = self.home / "users-test"
        self.run_command(["rustc", "--edition=2024", str(harness), "-o", str(binary)])
        (self.home / "greetd").mkdir()
        config = self.home / "greetd/config.toml"
        for edition, session in (("kde", "startplasma-wayland"), ("xfce", "startxfce4"), ("niri", "niri")):
            config.write_text("[default_session]\ncommand = \"regreet\"\n")
            self.env["TEST_EDITION"] = edition
            self.run_command([str(binary)])
            self.assertIn(f'command = "{session}"', config.read_text())


if __name__ == "__main__":
    unittest.main(verbosity=2)
