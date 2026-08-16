// ==========================================
// Sidebar — barra lateral con logo, buscador y navegación
// (equivalente a widgets/sidebar.py)
//
// FIX heredado: en Python se conectaba "activated" en GtkListBoxRow, que no
// existe (la señal correcta es "activate") → TypeError en runtime → el popover
// de búsqueda nunca mostraba resultados. Aquí se usa "activate".
// ==========================================

use gtk::prelude::*;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::widgets::search::Search;
use crate::widgets::sidebar_item::SidebarItem;

#[derive(Clone)]
pub struct PageEntry {
    pub id: String,
    pub parent: Option<String>,
    pub title: String,
    pub subtitle: String,
    pub icon: Option<String>,
}

pub struct Sidebar {
    pub root: gtk::Box,
    pub search: Search,
    pub menu: gtk::Box,
    pub buttons: HashMap<String, SidebarItem>,
    catalog: RefCell<Vec<PageEntry>>,
    popover: gtk::Popover,
    popover_results: gtk::ListBox,
    callbacks: Rc<RefCell<Vec<Box<dyn Fn(&str)>>>>,
    pages: Vec<(String, String, String)>, // (id, icon, title)
}

impl Sidebar {
    pub fn new() -> Self {
        let root = gtk::Box::new(gtk::Orientation::Vertical, 0);
        root.set_size_request(280, -1);
        root.add_css_class("sidebar");

        // Logo
        let logo = gtk::Image::from_file(crate::assets::logo_path());
        logo.set_pixel_size(56);
        logo.set_margin_top(20);
        logo.set_margin_bottom(16);
        root.append(&logo);

        // Buscador
        let search = Search::new();
        let search_widget = search.widget();
        search_widget.set_margin_start(16);
        search_widget.set_margin_end(16);
        search_widget.set_margin_bottom(20);
        root.append(search_widget);

        // Menú
        let menu = gtk::Box::new(gtk::Orientation::Vertical, 6);
        menu.set_margin_start(10);
        menu.set_margin_end(10);
        root.append(&menu);

        let sidebar = Sidebar {
            root,
            search,
            menu,
            buttons: HashMap::new(),
            catalog: RefCell::new(Vec::new()),
            popover: gtk::Popover::new(),
            popover_results: gtk::ListBox::new(),
            callbacks: Rc::new(RefCell::new(Vec::new())),
            pages: Vec::new(),
        };

        sidebar.setup_popover();
        sidebar.select("system");

        sidebar
    }

    fn setup_popover(&self) {
        self.popover.set_parent(self.search.widget());
        self.popover.set_position(gtk::PositionType::Bottom);
        self.popover.set_autohide(true);

        self.popover_results
            .set_selection_mode(gtk::SelectionMode::Single);
        self.popover_results.add_css_class("search-results");
        self.popover_results.set_size_request(260, -1);

        let scrolled = gtk::ScrolledWindow::new();
        scrolled.set_child(Some(&self.popover_results));
        scrolled.set_max_content_height(360);
        scrolled.set_propagate_natural_height(true);

        self.popover.set_child(Some(&scrolled));
    }

    pub fn connect_page_selected(&self, cb: impl Fn(&str) + 'static) {
        self.callbacks.borrow_mut().push(Box::new(cb));
    }

    fn emit_page_selected(&self, page: &str) {
        let callbacks = self.callbacks.borrow();
        for cb in callbacks.iter() {
            cb(page);
        }
    }

    pub fn register_page(&mut self, id: &str, icon: &str, title: &str) {
        self.pages
            .push((id.to_string(), icon.to_string(), title.to_string()));

        let item = SidebarItem::new(icon, title);
        let page_id = id.to_string();
        let callbacks = Rc::clone(&self.callbacks);

        item.widget().connect_clicked(move |_| {
            for cb in callbacks.borrow().iter() {
                cb(&page_id);
            }
        });

        self.menu.append(item.widget());
        self.buttons.insert(id.to_string(), item);
    }

    pub fn register_subpage(
        &self,
        page_id: &str,
        parent_id: &str,
        title: &str,
        subtitle: &str,
        icon: Option<&str>,
    ) {
        self.catalog.borrow_mut().push(PageEntry {
            id: page_id.to_string(),
            parent: Some(parent_id.to_string()),
            title: title.to_string(),
            subtitle: subtitle.to_string(),
            icon: icon.map(|s| s.to_string()),
        });
    }

    pub fn select(&self, page: &str) {
        for item in self.buttons.values() {
            item.deactivate();
        }
        if let Some(item) = self.buttons.get(page) {
            item.activate();
        }
    }

    /// Filtra el menú y muestra el popover de resultados.
    /// Público porque el wiring se hace desde window.rs (Sidebar no puede
    /// auto-referenciarse para conectar su propio Search).
    pub fn on_search(&self, query: &str) {
        let query = query.to_lowercase();

        // Filtrar botones del menú principal
        for (page_id, item) in &self.buttons {
            let title = self
                .pages
                .iter()
                .find(|(pid, _, _)| pid == page_id)
                .map(|(_, _, t)| t.clone())
                .unwrap_or_default();

            let visible = query.is_empty() || title.to_lowercase().contains(&query);
            item.widget().set_visible(visible);
        }

        self.update_popover(&query);
    }

    fn update_popover(&self, query: &str) {
        // Vaciar resultados
        while let Some(child) = self.popover_results.first_child() {
            self.popover_results.remove(&child);
        }

        if query.is_empty() {
            self.popover.popdown();
            return;
        }

        let catalog = self.catalog.borrow();
        let matches: Vec<PageEntry> = catalog
            .iter()
            .filter(|e| {
                let haystack = format!("{} {}", e.title, e.subtitle).to_lowercase();
                haystack.contains(query)
            })
            .cloned()
            .collect();

        if matches.is_empty() {
            self.popover.popdown();
            return;
        }

        // Cablear cada fila: al activarse navega a la página y cierra el popover.
        // FIX: señal correcta "activate" (el Python usaba "activated", que no
        // existe en GtkListBoxRow y lanzaba TypeError).
        let callbacks = Rc::clone(&self.callbacks);
        let popover = self.popover.clone();
        let search = self.search.widget().clone();

        for entry in &matches {
            let row = Self::build_result_row(entry);
            let page_id = entry.id.clone();
            let callbacks = Rc::clone(&callbacks);
            let popover = popover.clone();
            let search = search.clone();

            row.connect_activate(move |_| {
                popover.popdown();
                search.set_text("");
                for cb in callbacks.borrow().iter() {
                    cb(&page_id);
                }
            });

            self.popover_results.append(&row);
        }

        self.popover.popup();
    }

    fn build_result_row(entry: &PageEntry) -> gtk::ListBoxRow {
        let row = gtk::ListBoxRow::new();
        row.add_css_class("search-result");

        let box_ = gtk::Box::new(gtk::Orientation::Vertical, 2);
        box_.set_margin_top(8);
        box_.set_margin_bottom(8);
        box_.set_margin_start(12);
        box_.set_margin_end(12);

        let title_label = gtk::Label::builder()
            .label(&entry.title)
            .xalign(0.0)
            .build();
        title_label.set_hexpand(true);
        title_label.add_css_class("row-title");
        box_.append(&title_label);

        if !entry.subtitle.is_empty() {
            let sub_label = gtk::Label::builder()
                .label(&entry.subtitle)
                .xalign(0.0)
                .build();
            sub_label.set_hexpand(true);
            sub_label.add_css_class("row-subtitle");
            box_.append(&sub_label);
        }

        row.set_child(Some(&box_));
        row
    }
}
