import os

import gi

gi.require_version("Gtk", "4.0")

from gi.repository import Gtk, Gdk, GLib, Gio

from widgets.page import Page
from widgets.group import Group
from widgets.row import Row

from services.wallpaper import WallpaperService


class WallpaperPage(Page):

    def __init__(self, navigator):

        super().__init__(
            navigator,
            "Fondos",
            "Selecciona un fondo de pantalla",
            parent_page="appearance"
        )

        self.navigator = navigator

        #
        # Botón "Importar..."
        #

        actions_group = Group("Importar fondo")
        actions_group.add(
            Row(
                title="Importar desde archivos...",
                subtitle="Abre el selector de archivos de GTK",
                icon="wallpaper.svg",
                callback=lambda *_: self.import_from_files()
            )
        )
        self.add(actions_group)

        thunar_group = Group("Abrir carpeta")
        thunar_group.add(
            Row(
                title="Abrir ~/Imagenes con Thunar",
                subtitle="Arrastra fondos a ~/.local/share/churros/wallpapers",
                icon="wallpaper.svg",
                callback=lambda *_: self._open_pictures_folder()
            )
        )
        self.add(thunar_group)

        #
        # Fondo actual + grid
        #

        current = WallpaperService.current()
        wallpapers = WallpaperService.available()

        if not wallpapers:

            group = Group("Fondos disponibles")
            group.add(
                Row(
                    title="No se encontraron fondos",
                    subtitle="Importa una imagen o añádela a ~/.local/share/churros/wallpapers",
                    icon="wallpaper.svg"
                )
            )
            self.add(group)
            return

        #
        # Miniatura del fondo actual
        #

        current_group = Group("Fondo actual")

        if current and os.path.exists(current):

            try:

                texture = Gdk.Texture.new_from_filename(current)
                preview = Gtk.Image.new_from_paintable(texture)
                preview.set_pixel_size(160)
                preview.add_css_class("wallpaper-preview")

                current_group.add(
                    Row(
                        title=os.path.splitext(
                            os.path.basename(current)
                        )[0],
                        subtitle="Seleccionado",
                        icon="wallpaper.svg"
                    )
                )

                self.add(current_group)

            except Exception:

                self.add(current_group)

        #
        # Grid de fondos
        #

        grid_group = Group("Fondos disponibles")

        flow = Gtk.FlowBox()
        flow.set_selection_mode(Gtk.SelectionMode.NONE)
        flow.set_max_children_per_line(4)
        flow.set_min_children_per_line(2)
        flow.set_row_spacing(12)
        flow.set_column_spacing(12)
        flow.set_halign(Gtk.Align.FILL)

        for wallpaper in wallpapers:

            thumb = self._build_thumbnail(
                wallpaper,
                current
            )

            flow.insert(thumb, -1)

        grid_group.add(flow)

        self.add(grid_group)

    def import_from_files(self):

        win = self.get_root()

        try:
            dialog = Gtk.FileDialog()
            dialog.set_title("Importar imagen de fondo")

            filter_any = Gtk.FileFilter()
            filter_any.set_name("Imagenes")
            filter_any.add_mime_type("image/jpeg")
            filter_any.add_mime_type("image/png")
            filter_any.add_mime_type("image/webp")
            filter_any.add_mime_type("image/gif")
            filter_any.add_pattern("*.jpg")
            filter_any.add_pattern("*.jpeg")
            filter_any.add_pattern("*.png")
            filter_any.add_pattern("*.webp")
            filter_any.add_pattern("*.gif")

            filters = Gio.ListStore.new(Gtk.FileFilter)
            filters.append(filter_any)
            dialog.set_filters(filters)
            dialog.set_default_filter(filter_any)

            try:
                dialog.set_initial_folder(
                    Gio.File.new_for_path(os.path.expanduser("~"))
                )
            except Exception:
                pass

            def on_result(source, result, _user_data=None):
                try:
                    file = dialog.open_finish(result)
                except GLib.Error:
                    return

                if file is None:
                    return

                src = file.get_path() if hasattr(file, "get_path") else None

                if not src or not os.path.isfile(src):
                    return

                self._apply_wallpaper(src)

            dialog.open(win, None, on_result)

        except Exception:
            self._open_pictures_folder()

    def _open_pictures_folder(self):
        import subprocess

        pics_dir = os.path.expanduser("~/Imagenes")
        if not os.path.isdir(pics_dir):
            pics_dir = os.path.expanduser("~")

        try:
            subprocess.Popen(
                ["thunar", pics_dir],
                stdout=subprocess.DEVNULL,
                stderr=subprocess.DEVNULL,
                start_new_session=True,
            )
        except Exception:
            pass

    def _apply_wallpaper(self, src):

        dest = WallpaperService.import_image(src)

        if dest is None:
            print("[wallpaper] no se pudo importar", src, flush=True)
            self._show_error("No se pudo importar el fondo", src)
            return

        success = WallpaperService.set(dest)

        print("[wallpaper] import+set retorno:", success, "dest:", dest, flush=True)

        if not success:
            self._show_error(
                "No se pudo aplicar el fondo",
                "Revisa /tmp/churros-settings.log y /tmp/swaybg.log"
            )

        self._rebuild_grid()

    def _show_error(self, message, detail):

        try:

            win = self.get_root()

            dialog = Gtk.AlertDialog()

            dialog.set_modal(True)

            dialog.set_message(message)

            dialog.set_detail(detail)

            if win is not None:
                dialog.show(win)
            else:
                dialog.show()

        except Exception:
            pass

    def _rebuild_grid(self):
        """Recarga todo el contenido de la pagina para mostrar la nueva imagen."""

        content = self.content
        child = content.get_first_child()
        while child is not None:
            nxt = child.get_next_sibling()
            content.remove(child)
            child = nxt

        # Re-construir (re-ejecutar la parte grafica del __init__)
        self._build_after_import()

    def _build_after_import(self):
        """Reconstruye los grupos tras importar un fondo nuevo."""

        actions_group = Group("Importar fondo")
        actions_group.add(
            Row(
                title="Importar desde archivos...",
                subtitle="Elige una imagen de tu disco duro",
                icon="wallpaper.svg",
                callback=lambda *_: self.import_from_files()
            )
        )
        self.add(actions_group)

        current = WallpaperService.current()
        wallpapers = WallpaperService.available()

        if not wallpapers:

            group = Group("Fondos disponibles")
            group.add(
                Row(
                    title="No se encontraron fondos",
                    subtitle="Importa una imagen",
                    icon="wallpaper.svg"
                )
            )
            self.add(group)
            return

        current_group = Group("Fondo actual")

        if current and os.path.exists(current):

            try:

                texture = Gdk.Texture.new_from_filename(current)
                preview = Gtk.Image.new_from_paintable(texture)
                preview.set_pixel_size(160)
                preview.add_css_class("wallpaper-preview")

                current_group.add(
                    Row(
                        title=os.path.splitext(
                            os.path.basename(current)
                        )[0],
                        subtitle="Seleccionado",
                        icon="wallpaper.svg"
                    )
                )

                self.add(current_group)

            except Exception:
                self.add(current_group)

        grid_group = Group("Fondos disponibles")
        flow = Gtk.FlowBox()
        flow.set_selection_mode(Gtk.SelectionMode.NONE)
        flow.set_max_children_per_line(4)
        flow.set_min_children_per_line(2)
        flow.set_row_spacing(12)
        flow.set_column_spacing(12)
        flow.set_halign(Gtk.Align.FILL)

        for wallpaper in wallpapers:

            thumb = self._build_thumbnail(wallpaper, current)
            flow.insert(thumb, -1)

        grid_group.add(flow)

        self.add(grid_group)

    def _build_thumbnail(self, wallpaper, current):

        box = Gtk.Box(
            orientation=Gtk.Orientation.VERTICAL,
            spacing=6
        )

        name = os.path.splitext(
            os.path.basename(wallpaper)
        )[0]

        is_current = (wallpaper == current)

        try:

            texture = Gdk.Texture.new_from_filename(wallpaper)
            image = Gtk.Image.new_from_paintable(texture)
            image.set_pixel_size(120)
            image.add_css_class("wallpaper-thumb")

            if is_current:
                image.add_css_class("wallpaper-selected")

        except Exception:

            image = Gtk.Image.new_from_icon_name("image-missing")
            image.set_pixel_size(120)

        button = Gtk.Button()
        button.set_child(image)
        button.add_css_class("wallpaper-button")
        button.set_has_frame(False)
        button.set_tooltip_text(name)
        button.connect(
            "clicked",
            lambda _, w=wallpaper: self.select(w)
        )

        label = Gtk.Label(label=name)
        label.add_css_class("wallpaper-name")
        label.set_max_width_chars(18)
        label.set_ellipsize(0)  # PANGO_ELLIPSIZE_END
        label.set_tooltip_text(name)

        box.append(button)
        box.append(label)

        return box

    def select(self, wallpaper):

        print("[wallpaper-page] seleccion:", wallpaper, flush=True)

        success = WallpaperService.set(wallpaper)

        print("[wallpaper-page] set retorno:", success, flush=True)

        if not success:

            try:

                win = self.get_root()

                dialog = Gtk.AlertDialog()

                dialog.set_modal(True)

                dialog.set_message("No se pudo aplicar el fondo")

                dialog.set_detail(

                    "Revisa /tmp/churros-settings.log, "

                    "/tmp/swaybg.log"

                )

                if win is not None:

                    dialog.show(win)

                else:

                    dialog.show()

            except Exception:

                pass

        GLib.idle_add(
            lambda: self.navigator.show_page("appearance")
        )
