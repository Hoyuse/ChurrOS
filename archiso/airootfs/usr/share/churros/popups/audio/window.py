from pathlib import Path
import sys

sys.path.insert(0, str(Path(__file__).resolve().parent.parent))

import gi

gi.require_version("Gtk", "4.0")

from gi.repository import Gtk, Gdk

from common.popup import PopupWindow

from widgets.volume import VolumeWidget
from widgets.mute import MuteWidget
from widgets.device import DeviceWidget


class AudioWindow(PopupWindow):

    def __init__(self, app):

        super().__init__(
            app,
            title="Audio",
            icon="󰕾"
        )

        self.load_audio_css()

        self.add(
            VolumeWidget()
        )

        self.add(
            MuteWidget()
        )

        self.add(
            DeviceWidget()
        )

    def load_audio_css(self):

        provider = Gtk.CssProvider()

        provider.load_from_path(
            str(Path(__file__).parent / "style.css")
        )

        Gtk.StyleContext.add_provider_for_display(
            Gdk.Display.get_default(),
            provider,
            Gtk.STYLE_PROVIDER_PRIORITY_APPLICATION
        )