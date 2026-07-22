import gi

gi.require_version("Gtk", "4.0")

from gi.repository import Gtk

from popup import PopupWindow


class Application(Gtk.Application):

    def __init__(self):
        super().__init__(application_id="org.churros.popup.test")

    def do_activate(self):

        window = PopupWindow(
            self,
            title="Popup",
            icon="🧪"
        )

        window.present()


app = Application()
app.run()