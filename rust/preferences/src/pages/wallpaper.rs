// ==========================================
// WallpaperPage — fondos de pantalla (equivalente a pages/wallpaper.py)
// ==========================================

use std::path::Path;
use std::process::Command;

use gtk::prelude::*;

use crate::services::wallpaper::WallpaperService;
use crate::widgets::group::Group;
use crate::widgets::page::Page;
use crate::widgets::row::Row;

pub fn build(navigator: gtk::Stack) -> Page {
    let page = Page::new(
        Some(navigator.clone()),
        "Fondos",
        Some("Selecciona un fondo de pantalla"),
        Some("appearance".to_string()),
    );

    // Contenido reutilizable para reconstruir la página tras importar (self.content)
    let content: gtk::Box = page.content.clone();

    // ===== Botón "Importar..." =====
    build_actions(&page, &navigator, &content);

    // ===== Fondo actual + grid =====
    let current = WallpaperService::current();
    let wallpapers = WallpaperService::list();

    if wallpapers.is_empty() {
        let mut group = Group::new("Fondos disponibles");
        group.add(&Row::new(
            "No se encontraron fondos",
            Some("Importa una imagen o añádela a ~/.local/share/churros/wallpapers"),
            Some("wallpaper.svg"),
            None,
            None,
            None,
        ));
        page.add(group.widget());
        return page;
    }

    // ===== Miniatura del fondo actual =====
    if !current.is_empty() && Path::new(&current).is_file() {
        let mut current_group = Group::new("Fondo actual");

        // El Python carga Gdk.Texture.new_from_filename + crea un Gtk.Image con la
        // clase "wallpaper-preview"... pero NUNCA lo añade al grupo: solo añade la Row.
        // Código muerto del original; se omite la imagen (try/except silencioso).
        let name = Path::new(&current)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_string();

        current_group.add(&Row::new(
            &name,
            Some("Seleccionado"),
            Some("wallpaper.svg"),
            None,
            None,
            None,
        ));

        page.add(current_group.widget());
    }

    // ===== Grid de fondos =====
    build_grid(&content, &current, &wallpapers, &navigator);

    page
}

/// Grupos "Importar fondo" + "Abrir carpeta" (equivalente al inicio de __init__)
fn build_actions(page: &Page, navigator: &gtk::Stack, content: &gtk::Box) {
    let mut actions_group = Group::new("Importar fondo");

    let nav = navigator.clone();
    let content_cb = content.clone();
    actions_group.add(&Row::new(
        "Importar desde archivos...",
        Some("Abre el selector de archivos de GTK"),
        Some("wallpaper.svg"),
        None,
        None,
        Some(Box::new(move |_| {
            import_from_files(&nav, &content_cb);
        })),
    ));

    page.add(actions_group.widget());

    let mut thunar_group = Group::new("Abrir carpeta");

    let nav = navigator.clone();
    thunar_group.add(&Row::new(
        "Abrir carpeta de fondos",
        Some("Arrastra fondos a ~/.local/share/churros/wallpapers"),
        Some("wallpaper.svg"),
        None,
        None,
        Some(Box::new(move |_| {
            open_pictures_folder();
        })),
    ));

    page.add(thunar_group.widget());
}

/// Grid de fondos con FlowBox (equivalente a la parte final de __init__
/// y de _build_after_import)
fn build_grid(content: &gtk::Box, current: &str, wallpapers: &[std::path::PathBuf], navigator: &gtk::Stack) {
    let mut grid_group = Group::new("Fondos disponibles");

    let flow = gtk::FlowBox::new();
    flow.set_selection_mode(gtk::SelectionMode::None);
    flow.set_max_children_per_line(4);
    flow.set_min_children_per_line(2);
    flow.set_row_spacing(12);
    flow.set_column_spacing(12);
    flow.set_halign(gtk::Align::Fill);

    for wallpaper in wallpapers {
        let thumb = build_thumbnail(wallpaper, current, navigator);
        flow.insert(&thumb, -1);
    }

    // Gtk::FlowBox no implementa AsWidget; se inserta directo en la tarjeta del grupo
    grid_group.card.append(&flow);

    content.append(grid_group.widget());
}

/// Miniatura: botón con imagen + nombre (equivalente a _build_thumbnail)
fn build_thumbnail(wallpaper: &Path, current: &str, navigator: &gtk::Stack) -> gtk::Box {
    let box_ = gtk::Box::new(gtk::Orientation::Vertical, 6);

    let name = wallpaper
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_string();

    let is_current = wallpaper.to_string_lossy() == current;

    let image = match gtk::gdk::Texture::from_filename(wallpaper) {
        Ok(texture) => {
            let image = gtk::Image::from_paintable(Some(&texture));
            image.set_pixel_size(120);
            image.add_css_class("wallpaper-thumb");
            if is_current {
                image.add_css_class("wallpaper-selected");
            }
            image
        }
        Err(_) => {
            let image = gtk::Image::from_icon_name("image-missing");
            image.set_pixel_size(120);
            image
        }
    };

    let button = gtk::Button::new();
    button.set_child(Some(&image));
    button.add_css_class("wallpaper-button");
    button.set_has_frame(false);
    button.set_tooltip_text(Some(&name));

    let target = wallpaper.to_string_lossy().to_string();
    let nav = navigator.clone();
    button.connect_clicked(move |_| {
        select(&target, &nav);
    });

    let label = gtk::Label::new(Some(&name));
    label.add_css_class("wallpaper-name");
    label.set_max_width_chars(18);
    // Python: set_ellipsize(0) == PANGO_ELLIPSIZE_NONE (el comentario del original
    // dice END, pero 0 es NONE) — se porta el valor real: NONE.
    label.set_ellipsize(gtk::pango::EllipsizeMode::None);
    label.set_tooltip_text(Some(&name));

    box_.append(&button);
    box_.append(&label);

    box_
}

/// Ventana raíz de la página (equivalente a self.get_root() del Python)
fn root_window(navigator: &gtk::Stack) -> Option<gtk::Window> {
    navigator.root().and_then(|r| r.downcast::<gtk::Window>().ok())
}

/// Equivalente a WallpaperPage.import_from_files: FileDialog asíncrono
fn import_from_files(navigator: &gtk::Stack, content: &gtk::Box) {
    let Some(win) = root_window(navigator) else {
        show_error(navigator, "No se pudo abrir el selector", "La pagina no tiene ventana root.");
        return;
    };

    let dialog = gtk::FileDialog::new();
    dialog.set_title("Importar imagen de fondo");
    dialog.set_modal(true);

    let filter_any = gtk::FileFilter::new();
    filter_any.set_name(Some("Imagenes"));
    filter_any.add_mime_type("image/jpeg");
    filter_any.add_mime_type("image/png");
    filter_any.add_mime_type("image/webp");
    filter_any.add_mime_type("image/gif");
    filter_any.add_pattern("*.jpg");
    filter_any.add_pattern("*.jpeg");
    filter_any.add_pattern("*.png");
    filter_any.add_pattern("*.webp");
    filter_any.add_pattern("*.gif");

    let filters = gio::ListStore::new::<gtk::FileFilter>();
    filters.append(&filter_any);
    dialog.set_filters(Some(&filters));
    dialog.set_default_filter(Some(&filter_any));

    // set_initial_folder(home) — el Python lo envuelve en try/except silencioso
    let home = std::env::var("HOME").unwrap_or_else(|_| "/root".to_string());
    let home_file = gio::File::for_path(&home);
    dialog.set_initial_folder(Some(&home_file));

    let nav = navigator.clone();
    let content_cb = content.clone();
    dialog.open(Some(&win), None::<&gio::Cancellable>, move |result| {
        // Equivalente a on_result: open_finish
        let file = match result {
            Ok(f) => f,
            Err(e) => {
                println!("[wallpaper] FileDialog error: {e}");
                return;
            }
        };

        let Some(src) = file.path() else {
            println!("[wallpaper] ruta invalida: None");
            return;
        };
        let src = src.to_string_lossy().to_string();

        if !Path::new(&src).is_file() {
            println!("[wallpaper] ruta invalida: {src}");
            return;
        }

        apply_wallpaper(&src, &nav, &content_cb);
    });
}

/// Equivalente a _open_pictures_folder: crea ~/.local/share/churros/wallpapers
/// y la abre con thunar (fallback xdg-open), todo silencioso.
fn open_pictures_folder() {
    let wp_dir = WallpaperService::user_dir();
    let _ = std::fs::create_dir_all(&wp_dir);

    let dir_str = wp_dir.to_string_lossy().to_string();

    if Command::new("thunar")
        .arg(&dir_str)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .is_err()
    {
        let _ = Command::new("xdg-open")
            .arg(&dir_str)
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn();
    }
}

/// Equivalente a _apply_wallpaper: importar + aplicar + reconstruir el grid
fn apply_wallpaper(src: &str, navigator: &gtk::Stack, content: &gtk::Box) {
    let Some(dest) = WallpaperService::import_image(src) else {
        println!("[wallpaper] no se pudo importar {src}");
        show_error(navigator, "No se pudo importar el fondo", src);
        return;
    };

    rebuild_grid(content, navigator);

    let dest_clone = dest.clone();
    std::thread::spawn(move || {
        let success = WallpaperService::set(&dest_clone);
        println!("[wallpaper] import+set retorno: {success} dest: {dest_clone}");
    });
}

/// Equivalente a _show_error: Gtk::AlertDialog modal. Si no hay ventana raíz no
/// se muestra nada (en el Python dialog.show() sin ventana falla y el try/except
/// lo traga).
fn show_error(navigator: &gtk::Stack, message: &str, detail: &str) {
    let dialog = gtk::AlertDialog::builder()
        .modal(true)
        .message(message)
        .detail(detail)
        .build();

    if let Some(win) = root_window(navigator) {
        dialog.show(Some(&win));
    }
}

/// Equivalente a _rebuild_grid: vacía el contenido de la página y reconstruye
/// los grupos (re-ejecuta la parte gráfica del __init__).
fn rebuild_grid(content: &gtk::Box, navigator: &gtk::Stack) {
    // Vaciar el contenido (while child is not None: remove)
    let mut child = content.first_child();
    while let Some(c) = child {
        let nxt = c.next_sibling();
        content.remove(&c);
        child = nxt;
    }

    build_after_import(content, navigator);
}

/// Equivalente a _build_after_import: reconstruye los grupos tras importar.
/// NOTA: el Python solo re-añade el grupo "Importar fondo" (el de "Abrir carpeta"
/// desaparece tras una importación — bug del original, se porta tal cual).
fn build_after_import(content: &gtk::Box, navigator: &gtk::Stack) {
    let mut actions_group = Group::new("Importar fondo");

    let nav = navigator.clone();
    let content_cb = content.clone();
    actions_group.add(&Row::new(
        "Importar desde archivos...",
        Some("Elige una imagen de tu disco duro"),
        Some("wallpaper.svg"),
        None,
        None,
        Some(Box::new(move |_| {
            import_from_files(&nav, &content_cb);
        })),
    ));

    content.append(actions_group.widget());

    let current = WallpaperService::current();
    let wallpapers = WallpaperService::list();

    if wallpapers.is_empty() {
        let mut group = Group::new("Fondos disponibles");
        group.add(&Row::new(
            "No se encontraron fondos",
            Some("Importa una imagen"),
            Some("wallpaper.svg"),
            None,
            None,
            None,
        ));
        content.append(group.widget());
        return;
    }

    // Fondo actual
    if !current.is_empty() && Path::new(&current).is_file() {
        let mut current_group = Group::new("Fondo actual");

        let name = Path::new(&current)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_string();

        current_group.add(&Row::new(
            &name,
            Some("Seleccionado"),
            Some("wallpaper.svg"),
            None,
            None,
            None,
        ));

        content.append(current_group.widget());
    }

    // Grid
    build_grid(content, &current, &wallpapers, navigator);
}

/// Equivalente a WallpaperPage.select: aplicar fondo + volver a apariencia.
/// Se ejecuta en un hilo secundario para no bloquear el loop de eventos Wayland de GTK4.
fn select(wallpaper: &str, navigator: &gtk::Stack) {
    println!("[wallpaper-page] seleccion: {wallpaper}");

    let nav = navigator.clone();
    let wp = wallpaper.to_string();

    // Navegación inmediata a Apariencia para respuesta visual instantánea
    glib::idle_add_local_once(move || {
        nav.set_visible_child_name("appearance");
    });

    // Aplicar wallpaper y colores pywal en segundo plano de forma no bloqueante
    std::thread::spawn(move || {
        let success = WallpaperService::set(&wp);
        println!("[wallpaper-page] set retorno: {success}");
    });
}
