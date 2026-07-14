import gi

gi.require_version("Gtk", "4.0")
gi.require_version("Adw", "1")

from gi.repository import Gtk, Adw


class ChurrOSWelcome(Adw.Application):

    def __init__(self):
        super().__init__(application_id="org.churros.Welcome")

    def do_activate(self):
        window = Adw.ApplicationWindow(application=self)

        window.set_title("ChurrOS Welcome")
        window.set_default_size(900, 600)

        header = Adw.HeaderBar()

        title = Gtk.Label()
        title.set_markup("<span size='xx-large' weight='bold'>ChurrOS Welcome</span>")

        content = Gtk.Box(
            orientation=Gtk.Orientation.VERTICAL,
            spacing=24
        )

        content.set_margin_top(40)
        content.set_margin_bottom(40)
        content.set_margin_start(40)
        content.set_margin_end(40)

        subtitle = Gtk.Label(
            label="A Linux distribution made by the community, for the community."
        )

        content.append(title)
        content.append(subtitle)

        toolbar = Gtk.Box(
            orientation=Gtk.Orientation.VERTICAL
        )

        toolbar.append(header)
        toolbar.append(content)

        window.set_content(toolbar)

        window.present()


app = ChurrOSWelcome()
app.run()