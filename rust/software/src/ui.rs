// ==========================================
// churros-software — ventana
//
// Estructura: buscador arriba, filtro de origen y lista de paquetes con su
// estado.
//
// El trabajo lento (leer el catálogo de pacman, consultar Flatpak y el AUR,
// ejecutar pacman) va en `gio::spawn_blocking`, y la vuelta a la interfaz en
// `spawn_future_local`, que ya corre en el hilo principal de GTK. Así los
// widgets —que no son Send— nunca se tocan desde otro hilo.
// ==========================================

use crate::actions::{self, Action};
use crate::model::{self, Package, Source};
use crate::sources;

use adw::prelude::*;
use adw::{AboutWindow, ApplicationWindow};
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

/// Techo de filas pintadas a la vez. El catálogo de Arch tiene decenas de
/// miles de paquetes: se pintan los primeros y se dice cuántos faltan.
const MAX_ROWS: usize = 300;
/// Retardo de la búsqueda, para no llamar a la red en cada tecla.
const SEARCH_DELAY_MS: u64 = 350;

struct State {
    installed: Vec<Package>,
    catalog: Vec<Package>,
    remote: Vec<Package>,
    query: String,
    sources: Vec<Source>,
    busy: bool,
    /// Se incrementa en cada tecla; el temporizador de búsqueda solo actúa si
    /// su número sigue siendo el actual.
    generation: u64,
}

impl State {
    fn new() -> Self {
        State {
            installed: Vec::new(),
            catalog: Vec::new(),
            remote: Vec::new(),
            query: String::new(),
            sources: vec![Source::Pacman, Source::Flatpak, Source::Aur],
            busy: true,
            generation: 0,
        }
    }

    fn rebuild(&mut self) {
        let mut available = self.catalog.clone();
        available.append(&mut self.remote);
        self.catalog = available;
        self.remote = Vec::new();
    }
}

struct Ui {
    /// La ventana vive en el Rc: si solo la guardaran los widgets, al soltar
    /// esteRc la ventana se cerraría al terminar build().
    _window: ApplicationWindow,
    state: RefCell<State>,
    list: gtk::ListBox,
    status: adw::StatusPage,
    summary: gtk::Label,
    spinner: gtk::Spinner,
    busy_box: gtk::Box,
    banner: adw::Banner,
}

impl Ui {
    /// Repinta lista, resumen y estado vacío.
    fn refresh(&self) {
        // El catálogo tiene decenas de miles de entradas: se fusiona una sola
        // vez y de ahí salen tanto la lista visible como los contadores.
        let (visible, (installed, total), query_empty, busy) = {
            let st = self.state.borrow();
            let merged = model::merge(&st.installed, st.catalog.clone());
            let counts = model::counts(&merged);
            let filtered = model::filter(&merged, &st.query, &st.sources);
            (filtered, counts, st.query.trim().is_empty(), st.busy)
        };

        self.busy_box.set_visible(busy);
        self.spinner.set_spinning(busy);
        self.spinner.set_visible(busy);

        let show_list = !busy && !visible.is_empty();
        self.list.set_visible(show_list);
        self.status.set_visible(!busy && !show_list);
        if !busy {
            self.status.set_title(if query_empty { "Churros Software" } else { "Sin resultados" });
            self.status.set_description(Some(if query_empty {
                "Busca un paquete para instalarlo o desinstalarlo"
            } else {
                "Prueba con otro nombre"
            }));
        }

        self.summary.set_text(&format!(
            "{installed} instalados · {total} paquetes en el catálogo"
        ));

        while let Some(child) = self.list.first_child() {
            self.list.remove(&child);
        }
        for pkg in visible.iter().take(MAX_ROWS) {
            self.list.append(&build_row(pkg));
        }
        if visible.len() > MAX_ROWS {
            let row = adw::ActionRow::builder()
                .title(format!("y {} más", visible.len() - MAX_ROWS))
                .subtitle("Afina la búsqueda para ver el resto")
                .build();
            let wrapped = gtk::ListBoxRow::builder()
                .child(&row)
                .activatable(false)
                .build();
            self.list.append(&wrapped);
        }
    }

    fn set_busy(&self, busy: bool) {
        self.state.borrow_mut().busy = busy;
        self.refresh();
    }

    fn notice(&self, text: impl Into<String>) {
        let text = text.into();
        if text.is_empty() {
            self.banner.set_revealed(false);
            return;
        }
        self.banner.set_title(&text);
        self.banner.set_revealed(true);
    }
}

fn build_row(pkg: &Package) -> gtk::ListBoxRow {
    let title = match &pkg.installed_version {
        Some(v) if pkg.installed && !v.is_empty() => format!("{} · {v}", pkg.name),
        _ => format!("{} · {}", pkg.name, pkg.version),
    };

    let row = adw::ActionRow::builder()
        .title(title)
        .subtitle(&pkg.description)
        .build();

    let tag = gtk::Label::new(Some(pkg.source.label()));
    tag.add_css_class("caption-heading");
    tag.add_css_class("dim-label");
    row.add_prefix(&tag);

    if pkg.installed {
        let state = gtk::Label::new(Some("instalado"));
        state.add_css_class("success");
        state.add_css_class("caption");
        row.add_suffix(&state);
    }

    let action = if pkg.installed {
        Action::Remove
    } else {
        Action::Install
    };
    let button = gtk::Button::with_label(if action.is_removing() {
        "Desinstalar"
    } else {
        "Instalar"
    });
    button.add_css_class(if pkg.installed {
        "destructive-action"
    } else {
        "suggested-action"
    });
    button.set_valign(gtk::Align::Center);
    let owned = pkg.clone();
    button.connect_clicked(move |_| run_action(owned.clone(), action));
    row.add_suffix(&button);

    gtk::ListBoxRow::builder()
        .child(&row)
        .activatable(false)
        .build()
}

/// Ejecuta la acción y luego recarga la lista de instalados.
fn run_action(pkg: Package, action: Action) {
    let plan = match actions::plan(&pkg, action) {
        Ok(plan) => plan,
        Err(err) => {
            show_error(&err);
            return;
        }
    };

    glib::spawn_future_local(async move {
        let outcome = gio::spawn_blocking(move || match plan {
            actions::Plan::Privileged(argv) => {
                let (code, stderr) = actions::run_privileged(&argv);
                if code == 0 {
                    Ok(())
                } else {
                    Err(actions::describe_error(code, &stderr))
                }
            }
            actions::Plan::Terminal(cmd) => {
                actions::spawn_terminal(&cmd).map_err(|e| format!("{e}. El paquete no se ha instalado."))
            }
        })
        .await;

        // Un pánico dentro del hilo de trabajo llega como Err.
        let outcome = match outcome {
            Ok(result) => result,
            Err(_) => Err("La operación se interrumpió".to_string()),
        };

        match outcome {
            Ok(()) => {
                let ui = ui();
                ui.notice(if action.is_removing() {
                    format!("{} desinstalado", pkg.name)
                } else {
                    format!("{} instalado", pkg.name)
                });
                load_installed();
            }
            Err(err) => show_error(err),
        }
    });
}

fn ui() -> Rc<Ui> {
    UI.with(|u| {
        u.borrow()
            .clone()
            .expect("la interfaz aún no se ha construido")
    })
}

thread_local! {
    static UI: RefCell<Option<Rc<Ui>>> = const { RefCell::new(None) };
}

fn show_error(message: impl Into<String>) {
    ui().notice(message);
}

fn load_installed() {
    glib::spawn_future_local(async move {
        let installed = gio::spawn_blocking(|| {
            let mut all = sources::installed_pacman();
            all.extend(sources::installed_flatpak());
            all
        })
        .await
        .unwrap_or_default();
        let ui = ui();
        {
            let mut st = ui.state.borrow_mut();
            st.installed = installed;
            st.rebuild();
        }
        ui.refresh();
    });
}

fn load_catalog() {
    glib::spawn_future_local(async move {
        let catalog = gio::spawn_blocking(sources::catalog_pacman)
            .await
            .unwrap_or_default();
        let ui = ui();
        {
            let mut st = ui.state.borrow_mut();
            st.catalog = catalog;
            st.busy = false;
            st.rebuild();
        }
        ui.refresh();
    });
}

fn search_remotes(query: String) {
    glib::spawn_future_local(async move {
        let for_query = query.clone();
        let found = gio::spawn_blocking(move || {
            let mut found = sources::search_flatpak(&for_query);
            found.extend(sources::search_aur(&for_query));
            found
        })
        .await
        .unwrap_or_default();

        let ui = ui();
        {
            let mut st = ui.state.borrow_mut();
            // Si la persona ya ha borrado lo que escribió, estos resultados
            // son de una búsqueda que ya no importa.
            if st.query.trim() == query {
                st.remote = found;
                st.rebuild();
            }
        }
        ui.refresh();
    });
}

pub fn build(app: &gtk::Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Churros Software")
        .default_width(840)
        .default_height(680)
        .build();

    // ---- barra superior ----
    let header = adw::HeaderBar::new();
    let spinner = gtk::Spinner::new();
    spinner.set_visible(false);
    header.pack_end(&spinner);

    let refresh_btn = gtk::Button::from_icon_name("view-refresh-symbolic");
    refresh_btn.set_tooltip_text(Some("Recargar catálogo e instalados"));
    header.pack_start(&refresh_btn);

    let about_btn = gtk::Button::from_icon_name("help-about-symbolic");
    about_btn.set_tooltip_text(Some("Acerca de"));
    header.pack_end(&about_btn);

    // ---- buscador ----
    let search = gtk::SearchEntry::new();
    search.set_hexpand(true);
    search.set_placeholder_text(Some("Buscar en Arch, Flatpak y AUR"));
    search.set_margin_top(6);
    search.set_margin_bottom(6);
    search.set_margin_start(12);
    search.set_margin_end(12);

    // ---- filtro de origen ----
    let origins = gtk::StringList::new(&["Todos", "Arch", "Flatpak", "AUR"]);
    let origin = gtk::DropDown::builder()
        .model(&origins)
        .selected(0)
        .build();
    origin.set_margin_start(12);

    let summary = gtk::Label::new(Some("Cargando catálogo…"));
    summary.set_xalign(0.0);
    summary.set_hexpand(true);
    summary.add_css_class("dim-label");
    summary.set_margin_start(6);

    let filter_bar = gtk::Box::new(gtk::Orientation::Horizontal, 6);
    filter_bar.set_margin_end(12);
    filter_bar.append(&origin);
    filter_bar.append(&summary);

    // ---- lista ----
    let list = gtk::ListBox::new();
    list.set_selection_mode(gtk::SelectionMode::None);
    list.add_css_class("boxed-list");

    let scroller = gtk::ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never)
        .vexpand(true)
        .child(&list)
        .build();
    scroller.set_margin_start(12);
    scroller.set_margin_end(12);
    scroller.set_margin_bottom(12);

    // ---- estado vacío / cargando ----
    let status = adw::StatusPage::new();
    status.set_icon_name(Some("system-software-install-symbolic"));
    status.set_title("Churros Software");
    status.set_description(Some("Busca un paquete para instalarlo o desinstalarlo"));
    status.set_vexpand(true);

    let busy_box = gtk::Box::new(gtk::Orientation::Vertical, 12);
    busy_box.set_vexpand(true);
    busy_box.set_valign(gtk::Align::Center);
    let busy_label = gtk::Label::new(Some("Leyendo el catálogo de paquetes…"));
    busy_label.add_css_class("dim-label");
    busy_box.append(&busy_label);

    // ---- avisos ----
    let banner = adw::Banner::new("");

    let content = gtk::Box::new(gtk::Orientation::Vertical, 0);
    content.append(&banner);
    content.append(&search);
    content.append(&filter_bar);
    content.append(&busy_box);
    content.append(&scroller);
    content.append(&status);

    let toolbar = adw::ToolbarView::new();
    toolbar.add_top_bar(&header);
    toolbar.set_content(Some(&content));
    window.set_content(Some(&toolbar));

    let ui = Rc::new(Ui {
        _window: window.clone(),
        state: RefCell::new(State::new()),
        list,
        status,
        summary,
        spinner,
        busy_box,
        banner,
    });
    UI.with(|slot| *slot.borrow_mut() = Some(ui.clone()));

    // ---- señales ----
    {
        let ui = ui.clone();
        refresh_btn.connect_clicked(move |_| {
            ui.set_busy(true);
            load_installed();
            load_catalog();
        });
    }

    {
        let ui = ui.clone();
        origin.connect_selected_notify(move |dropdown| {
            let sources = match dropdown.selected() {
                1 => vec![Source::Pacman],
                2 => vec![Source::Flatpak],
                3 => vec![Source::Aur],
                _ => vec![Source::Pacman, Source::Flatpak, Source::Aur],
            };
            ui.state.borrow_mut().sources = sources;
            ui.refresh();
        });
    }

    {
        let ui = ui.clone();
        search.connect_search_changed(move |entry| {
            let text = entry.text().to_string();
            {
                let mut st = ui.state.borrow_mut();
                st.query = text.clone();
                st.generation += 1;
                st.remote.clear();
                st.rebuild();
            }
            ui.refresh();

            // Cada pulsación invalida el temporizador anterior.
            let generation = ui.state.borrow().generation;
            let ui = ui.clone();
            glib::timeout_add_local(Duration::from_millis(SEARCH_DELAY_MS), move || {
                let query = ui.state.borrow().query.clone();
                if ui.state.borrow().generation != generation
                    || query.trim().len() < 2
                {
                    return glib::ControlFlow::Continue;
                }
                let query = query.trim().to_string();
                search_remotes(query);
                glib::ControlFlow::Continue
            });
        });
    }

    {
        let banner = ui.banner.clone();
        let clicked = banner.clone();
        banner.connect_button_clicked(move |_| clicked.set_revealed(false));
    }

    {
        let parent = window.clone();
        about_btn.connect_clicked(move |_| {
            AboutWindow::builder()
                .transient_for(&parent)
                .modal(true)
                .application_name("Churros Software")
                .application_icon("system-software-install")
                .version(env!("CARGO_PKG_VERSION"))
                .developer_name("Churros")
                .license_type(gtk::License::Gpl30)
                .comments("Buscador de paquetes Arch, Flatpak y AUR")
                .build()
                .present();
        });
    }

    ui.refresh();
    load_installed();
    load_catalog();
    window.present();
}
