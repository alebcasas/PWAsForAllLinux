//! GTK Application UI

use anyhow::Result;
use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{
    Application, ApplicationWindow, Box, Button, Entry, HeaderBar, Label, ListBox,
    ListBoxRow, Orientation, ScrolledWindow, Separator, Stack, StackSidebar,
    Image, MessageDialog, MessageType, ButtonsType,
    Grid, ComboBoxText, SpinButton, Switch, Adjustment,
};
use gtk::gdk::Texture;
use gtk::gio::File as GioFile;
use std::cell::RefCell;
use std::rc::Rc;

use crate::config;
use crate::i18n::t;
use crate::pwa::{self, Pwa, PwaManager};
use crate::utils;

/// Build the main UI
pub fn build_ui(app: &Application) -> Result<()> {
    // Create main window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("PWAsForAllLinux")
        .default_width(900)
        .default_height(600)
        .build();

    // Create header bar
    let header = HeaderBar::new();
    header.set_title_widget(Some(&Label::new(Some("PWAsForAllLinux"))));
    window.set_titlebar(Some(&header));

    // Create main container
    let main_box = Box::new(Orientation::Horizontal, 0);

    // Create stack and sidebar
    let stack = Stack::new();
    stack.set_vexpand(true);
    stack.set_hexpand(true);

    let sidebar = StackSidebar::new();
    sidebar.set_stack(&stack);
    sidebar.set_width_request(200);

    // Create shared PWA list for dynamic updates
    let pwa_list_box = Rc::new(RefCell::new(ListBox::new()));
    pwa_list_box.borrow().add_css_class("boxed-list");

    // PWAs page
    let pwas_page = create_pwas_page_with_list(pwa_list_box.clone())?;
    stack.add_titled(&pwas_page, Some("pwas"), &t("my_pwas"));

    // Add PWA page
    let add_page = create_add_pwa_page(pwa_list_box.clone())?;
    stack.add_titled(&add_page, Some("add"), &t("add_pwa"));

    // Settings page
    let settings_page = create_settings_page()?;
    stack.add_titled(&settings_page, Some("settings"), &t("settings"));

    // About page
    let about_page = create_about_page();
    stack.add_titled(&about_page, Some("about"), &t("about"));

    // Pack widgets
    main_box.append(&sidebar);
    main_box.append(&stack);

    window.set_child(Some(&main_box));
    window.present();

    Ok(())
}

/// Create PWAs list page with shared list box
fn create_pwas_page_with_list(pwa_list_box: Rc<RefCell<ListBox>>) -> Result<gtk::Box> {
    let container = Box::new(Orientation::Vertical, 12);
    container.set_margin_top(12);
    container.set_margin_bottom(12);
    container.set_margin_start(12);
    container.set_margin_end(12);

    // Header
    let header_box = Box::new(Orientation::Horizontal, 12);
    let title = Label::new(Some(&t("installed_pwas")));
    title.add_css_class("title-2");
    header_box.append(&title);

    // Refresh button
    let refresh_btn = Button::from_icon_name("view-refresh-symbolic");
    refresh_btn.set_tooltip_text(Some(&t("refresh_list")));
    let pwa_list_box_clone = pwa_list_box.clone();
    refresh_btn.connect_clicked(move |_| {
        update_pwa_list(&pwa_list_box_clone);
    });
    header_box.append(&refresh_btn);

    container.append(&header_box);

    // PWA list
    let scrolled = ScrolledWindow::new();
    scrolled.set_vexpand(true);
    scrolled.set_min_content_height(300);

    // Load initial PWAs
    update_pwa_list(&pwa_list_box);

    // Get the list box widget
    let list_box_widget = pwa_list_box.borrow().clone();
    scrolled.set_child(Some(&list_box_widget));
    container.append(&scrolled);

    Ok(container)
}

/// Update the PWA list
fn update_pwa_list(pwa_list_box: &Rc<RefCell<ListBox>>) {
    let list_box = pwa_list_box.borrow();
    
    // Clear existing items
    while let Some(child) = list_box.first_child() {
        list_box.remove(&child);
    }

    // Load PWAs
    match PwaManager::new() {
        Ok(pwa_manager) => {
            let pwas = pwa_manager.get_all();

            if pwas.is_empty() {
                let empty_label = Label::new(Some(&t("no_pwas_installed")));
                empty_label.add_css_class("dim-label");
                list_box.append(&empty_label);
            } else {
                for pwa in pwas {
                    match create_pwa_row(&pwa, pwa_list_box.clone()) {
                        Ok(row) => list_box.append(&row),
                        Err(e) => {
                            tracing::error!("Failed to create PWA row: {}", e);
                        }
                    }
                }
            }
        }
        Err(e) => {
            tracing::error!("Failed to load PWA manager: {}", e);
            let error_label = Label::new(Some(&format!("Error loading PWAs: {}", e)));
            error_label.add_css_class("dim-label");
            list_box.append(&error_label);
        }
    }
}

/// Create a PWA list row
fn create_pwa_row(pwa: &Pwa, pwa_list_box: Rc<RefCell<ListBox>>) -> Result<ListBoxRow> {
    let row = ListBoxRow::new();
    row.set_activatable(true);

    let row_box = Box::new(Orientation::Horizontal, 12);
    row_box.set_margin_top(8);
    row_box.set_margin_bottom(8);
    row_box.set_margin_start(12);
    row_box.set_margin_end(12);

    // Icon
    let icon = if let Some(icon_path) = &pwa.icon_path {
        let file = GioFile::for_path(icon_path);
        let texture = Texture::from_file(&file).ok();
        if let Some(tex) = texture {
            let image = Image::from_paintable(Some(&tex));
            image.set_pixel_size(48);
            image
        } else {
            create_default_icon()
        }
    } else {
        create_default_icon()
    };
    row_box.append(&icon);

    // Info
    let info_box = Box::new(Orientation::Vertical, 4);
    info_box.set_hexpand(true);

    let name_label = Label::new(Some(&pwa.name));
    name_label.add_css_class("title-4");
    name_label.set_halign(gtk::Align::Start);
    info_box.append(&name_label);

    let url_label = Label::new(Some(&pwa.url));
    url_label.add_css_class("dim-label");
    url_label.add_css_class("caption");
    url_label.set_halign(gtk::Align::Start);
    url_label.set_ellipsize(gtk::pango::EllipsizeMode::End);
    info_box.append(&url_label);

    row_box.append(&info_box);

    // Actions
    let actions_box = Box::new(Orientation::Horizontal, 6);

    // Launch button
    let launch_btn = Button::with_label(&t("launch"));
    launch_btn.add_css_class("suggested-action");
    launch_btn.add_css_class("pill");
    let pwa_id_launch = pwa.id.clone();
    launch_btn.connect_clicked(move |_| {
        launch_pwa(&pwa_id_launch);
    });
    actions_box.append(&launch_btn);

    // Edit button
    let edit_btn = Button::from_icon_name("document-edit-symbolic");
    edit_btn.set_tooltip_text(Some(&t("edit_pwa")));
    let pwa_id_edit = pwa.id.clone();
    edit_btn.connect_clicked(move |_| {
        edit_pwa(&pwa_id_edit);
    });
    actions_box.append(&edit_btn);

    // Delete button
    let delete_btn = Button::from_icon_name("user-trash-symbolic");
    delete_btn.add_css_class("destructive-action");
    delete_btn.set_tooltip_text(Some(&t("delete_pwa")));
    let pwa_id_delete = pwa.id.clone();
    let pwa_list_box_clone = pwa_list_box.clone();
    delete_btn.connect_clicked(move |_| {
        delete_pwa(&pwa_id_delete, pwa_list_box_clone.clone());
    });
    actions_box.append(&delete_btn);

    row_box.append(&actions_box);
    row.set_child(Some(&row_box));

    Ok(row)
}

/// Create default icon
fn create_default_icon() -> Image {
    let icon = Image::from_icon_name("application-x-executable");
    icon.set_pixel_size(48);
    icon
}

/// Launch a PWA
fn launch_pwa(pwa_id: &str) {
    // Get the path to the launcher binary
    let launcher_path = std::env::current_exe()
        .map(|p| {
            let parent = p.parent().unwrap_or(&p);
            parent.join("pwa-launcher")
        })
        .unwrap_or_else(|_| std::path::PathBuf::from("pwa-launcher"));

    // Spawn the launcher
    std::process::Command::new(&launcher_path)
        .arg(pwa_id)
        .spawn()
        .ok();
}

/// Edit a PWA
fn edit_pwa(pwa_id: &str) {
    // TODO: Open edit dialog for the PWA
    tracing::info!("Edit PWA: {}", pwa_id);
    // For now, just log the action
    // In a full implementation, this would open a dialog to edit PWA settings
}

/// Delete a PWA
fn delete_pwa(pwa_id: &str, pwa_list_box: Rc<RefCell<ListBox>>) {
    // Show confirmation dialog
    let dialog = MessageDialog::builder()
        .message_type(MessageType::Question)
        .buttons(ButtonsType::YesNo)
        .text("Delete PWA")
        .secondary_text("Are you sure you want to delete this PWA? This action cannot be undone.")
        .build();

    let pwa_id_owned = pwa_id.to_string();
    dialog.connect_response(move |dialog, response| {
        if response == gtk::ResponseType::Yes {
            // Delete the PWA
            match PwaManager::new() {
                Ok(mut manager) => {
                    if let Err(e) = manager.remove(&pwa_id_owned) {
                        tracing::error!("Failed to delete PWA: {}", e);
                        utils::show_error_dialog(None, &format!("Failed to delete PWA: {}", e));
                    } else {
                        tracing::info!("PWA deleted successfully: {}", pwa_id_owned);
                        // Update the PWA list
                        update_pwa_list(&pwa_list_box);
                    }
                }
                Err(e) => {
                    tracing::error!("Failed to load PWA manager: {}", e);
                    utils::show_error_dialog(None, &format!("Failed to load PWA manager: {}", e));
                }
            }
        }
        dialog.close();
    });

    dialog.present();
}

/// Create Add PWA page
fn create_add_pwa_page(pwa_list_box: Rc<RefCell<ListBox>>) -> Result<gtk::Box> {
    let container = Box::new(Orientation::Vertical, 12);
    container.set_margin_top(12);
    container.set_margin_bottom(12);
    container.set_margin_start(12);
    container.set_margin_end(12);

    // Title
    let title = Label::new(Some("Add New PWA"));
    title.add_css_class("title-2");
    container.append(&title);

    // Form
    let form = Grid::new();
    form.set_row_spacing(12);
    form.set_column_spacing(12);
    form.set_margin_top(12);

    // URL
    let url_label = Label::new(Some("Website URL:"));
    url_label.set_halign(gtk::Align::Start);
    form.attach(&url_label, 0, 0, 1, 1);

    let url_entry = Entry::new();
    url_entry.set_placeholder_text(Some("https://example.com"));
    url_entry.set_hexpand(true);
    form.attach(&url_entry, 1, 0, 2, 1);

    // Name
    let name_label = Label::new(Some("Name:"));
    name_label.set_halign(gtk::Align::Start);
    form.attach(&name_label, 0, 1, 1, 1);

    let name_entry = Entry::new();
    name_entry.set_placeholder_text(Some("My Web App"));
    form.attach(&name_entry, 1, 1, 2, 1);

    // Width
    let width_label = Label::new(Some("Window Width:"));
    width_label.set_halign(gtk::Align::Start);
    form.attach(&width_label, 0, 2, 1, 1);

    let width_adj = Adjustment::new(1280.0, 400.0, 3840.0, 10.0, 100.0, 0.0);
    let width_spin = SpinButton::new(Some(&width_adj), 1.0, 0);
    form.attach(&width_spin, 1, 2, 1, 1);

    // Height
    let height_label = Label::new(Some("Window Height:"));
    height_label.set_halign(gtk::Align::Start);
    form.attach(&height_label, 0, 3, 1, 1);

    let height_adj = Adjustment::new(800.0, 300.0, 2160.0, 10.0, 100.0, 0.0);
    let height_spin = SpinButton::new(Some(&height_adj), 1.0, 0);
    form.attach(&height_spin, 1, 3, 1, 1);

    // Display mode
    let display_label = Label::new(Some("Display Mode:"));
    display_label.set_halign(gtk::Align::Start);
    form.attach(&display_label, 0, 4, 1, 1);

    let display_combo = ComboBoxText::new();
    display_combo.append(Some("standalone"), "Standalone (Recommended)");
    display_combo.append(Some("minimal-ui"), "Minimal UI");
    display_combo.append(Some("fullscreen"), "Fullscreen");
    display_combo.set_active_id(Some("standalone"));
    form.attach(&display_combo, 1, 4, 2, 1);

    container.append(&form);

    // Buttons
    let btn_box = Box::new(Orientation::Horizontal, 12);
    btn_box.set_halign(gtk::Align::End);
    btn_box.set_margin_top(12);

    let auto_detect_btn = Button::with_label(&t("auto_detect"));
    let url_entry_clone = url_entry.clone();
    let name_entry_clone = name_entry.clone();
    let width_spin_clone = width_spin.clone();
    let height_spin_clone = height_spin.clone();
    let display_combo_clone = display_combo.clone();
    let auto_detect_btn_for_closure = auto_detect_btn.clone();
    
    auto_detect_btn.connect_clicked(move |_| {
        let url = url_entry_clone.text().to_string();
        if url.is_empty() {
            utils::show_error_dialog(None, &t("please_enter_url_first"));
            return;
        }
        
        // Validate URL format
        if !url.starts_with("http://") && !url.starts_with("https://") {
            utils::show_error_dialog(None, "URL must start with http:// or https://");
            return;
        }
        
        // Show loading state
        auto_detect_btn_for_closure.set_sensitive(false);
        auto_detect_btn_for_closure.set_label("Loading...");
        
        // Use timeout to simulate async operation without Tokio
        let url_clone = url.clone();
        let name_entry_clone2 = name_entry_clone.clone();
        let width_spin_clone2 = width_spin_clone.clone();
        let height_spin_clone2 = height_spin_clone.clone();
        let display_combo_clone2 = display_combo_clone.clone();
        let auto_detect_btn_for_timeout = auto_detect_btn_for_closure.clone();
        
        glib::timeout_add_local(std::time::Duration::from_millis(100), move || {
            // Fetch manifest synchronously in a blocking way
            match std::thread::spawn({
                let url = url_clone.clone();
                move || {
                    let rt = tokio::runtime::Runtime::new().unwrap();
                    rt.block_on(pwa::fetch_manifest(&url))
                }
            }).join() {
                Ok(result) => {
                    match result {
                        Ok(manifest) => {
                            // Update form with manifest data
                            if let Some(name) = manifest.name {
                                name_entry_clone2.set_text(&name);
                            }
                            if let Some(display) = manifest.display {
                                match display.as_str() {
                                    "standalone" => { display_combo_clone2.set_active_id(Some("standalone")); }
                                    "fullscreen" => { display_combo_clone2.set_active_id(Some("fullscreen")); }
                                    "minimal-ui" => { display_combo_clone2.set_active_id(Some("minimal-ui")); }
                                    _ => {}
                                }
                            }
                            tracing::info!("Manifest fetched successfully");
                            utils::show_info_dialog(None, &t("success"), &t("manifest_fetched_successfully"));
                        }
                        Err(e) => {
                            tracing::error!("Failed to fetch manifest: {}", e);
                            utils::show_error_dialog(None, &format!("{}: {}", t("failed_to_fetch_manifest"), e));
                        }
                    }
                }
                Err(_) => {
                    tracing::error!("Thread panicked while fetching manifest");
                    utils::show_error_dialog(None, "Failed to fetch manifest: thread panic");
                }
            }
            
            // Restore button state
            auto_detect_btn_for_timeout.set_sensitive(true);
            auto_detect_btn_for_timeout.set_label(&t("auto_detect"));
            
            glib::ControlFlow::Break
        });
    });
    btn_box.append(&auto_detect_btn);

    let install_btn = Button::with_label(&t("install_pwa"));
    install_btn.add_css_class("suggested-action");
    install_btn.add_css_class("pill");
    
    let url_entry_clone2 = url_entry.clone();
    let name_entry_clone3 = name_entry.clone();
    let width_spin_clone3 = width_spin.clone();
    let height_spin_clone3 = height_spin.clone();
    let display_combo_clone3 = display_combo.clone();
    let install_btn_for_closure = install_btn.clone();
    let pwa_list_box_clone2 = pwa_list_box.clone();
    
    install_btn.connect_clicked(move |_| {
        let url = url_entry_clone2.text().to_string();
        let name = name_entry_clone3.text().to_string();
        
        if url.is_empty() {
            utils::show_error_dialog(None, &t("please_enter_url"));
            return;
        }
        
        if name.is_empty() {
            utils::show_error_dialog(None, &t("please_enter_name"));
            return;
        }
        
        let width = width_spin_clone3.value() as i32;
        let height = height_spin_clone3.value() as i32;
        let display_mode = display_combo_clone3.active_id()
            .map(|s| s.to_string())
            .unwrap_or_else(|| "standalone".to_string());
        
        // Show loading state
        install_btn_for_closure.set_sensitive(false);
        install_btn_for_closure.set_label("Installing...");
        
        // Create PWA
        let mut pwa = pwa::Pwa::new(name, url);
        pwa.width = width;
        pwa.height = height;
        pwa.display_mode = display_mode;
        
        // Add PWA
        match pwa::PwaManager::new() {
            Ok(mut manager) => {
                if let Err(e) = manager.add(pwa) {
                    tracing::error!("Failed to add PWA: {}", e);
                    utils::show_error_dialog(None, &format!("{}: {}", t("failed_to_add_pwa"), e));
                } else {
                    tracing::info!("PWA added successfully");
                    utils::show_info_dialog(None, &t("success"), &t("pwa_installed_successfully"));
                    // Update the PWA list
                    update_pwa_list(&pwa_list_box_clone2);
                }
            }
            Err(e) => {
                tracing::error!("Failed to load PWA manager: {}", e);
                utils::show_error_dialog(None, &format!("{}: {}", t("failed_to_load_pwa_manager"), e));
            }
        }
        
        // Restore button state
        install_btn_for_closure.set_sensitive(true);
        install_btn_for_closure.set_label(&t("install_pwa"));
    });
    btn_box.append(&install_btn);

    container.append(&btn_box);

    // Separator
    container.append(&Separator::new(Orientation::Horizontal));

    // Help text
    let help_label = Label::new(Some(
        "💡 Tip: Click 'Auto-detect from URL' to automatically fetch\n\
         the web app manifest and fill in the details."
    ));
    help_label.add_css_class("dim-label");
    help_label.add_css_class("caption");
    container.append(&help_label);

    Ok(container)
}

/// Create Settings page
fn create_settings_page() -> Result<gtk::Box> {
    let container = Box::new(Orientation::Vertical, 12);
    container.set_margin_top(12);
    container.set_margin_bottom(12);
    container.set_margin_start(12);
    container.set_margin_end(12);

    // Title
    let title = Label::new(Some(&t("settings")));
    title.add_css_class("title-2");
    container.append(&title);

    // Settings form
    let form = Grid::new();
    form.set_row_spacing(12);
    form.set_column_spacing(12);
    form.set_margin_top(12);

    // Load config
    let config = config::load_config().unwrap_or_default();

    // Language
    let lang_label = Label::new(Some(&t("language")));
    lang_label.set_halign(gtk::Align::Start);
    form.attach(&lang_label, 0, 0, 1, 1);

    let lang_combo = ComboBoxText::new();
    lang_combo.append(Some("en"), "English");
    lang_combo.append(Some("es"), "Español");
    lang_combo.set_active_id(Some(&config.language));
    form.attach(&lang_combo, 1, 0, 2, 1);

    // Default engine
    let engine_label = Label::new(Some(&t("browser_engine")));
    engine_label.set_halign(gtk::Align::Start);
    form.attach(&engine_label, 0, 1, 1, 1);

    let engine_combo = ComboBoxText::new();
    engine_combo.append(Some("webkit"), "WebKitGTK (Default)");
    engine_combo.append(Some("firefox"), "Firefox");
    engine_combo.append(Some("chromium"), "Chromium");
    engine_combo.set_active_id(Some(&config.default_engine));
    form.attach(&engine_combo, 1, 1, 2, 1);

    // Hardware acceleration
    let hw_accel_label = Label::new(Some(&t("hardware_acceleration")));
    hw_accel_label.set_halign(gtk::Align::Start);
    form.attach(&hw_accel_label, 0, 2, 1, 1);

    let hw_accel_switch = Switch::new();
    hw_accel_switch.set_active(config.hardware_acceleration);
    form.attach(&hw_accel_switch, 1, 2, 1, 1);

    // Notifications
    let notif_label = Label::new(Some(&t("enable_notifications")));
    notif_label.set_halign(gtk::Align::Start);
    form.attach(&notif_label, 0, 3, 1, 1);

    let notif_switch = Switch::new();
    notif_switch.set_active(config.enable_notifications);
    form.attach(&notif_switch, 1, 3, 1, 1);

    // Theme
    let theme_label = Label::new(Some(&t("theme")));
    theme_label.set_halign(gtk::Align::Start);
    form.attach(&theme_label, 0, 4, 1, 1);

    let theme_combo = ComboBoxText::new();
    theme_combo.append(Some("system"), &t("system_default"));
    theme_combo.append(Some("light"), &t("light"));
    theme_combo.append(Some("dark"), &t("dark"));
    theme_combo.set_active_id(Some(&config.theme));
    form.attach(&theme_combo, 1, 4, 2, 1);

    container.append(&form);

    // Separator
    container.append(&Separator::new(Orientation::Horizontal));

    // Save button
    let save_btn = Button::with_label(&t("save_settings"));
    save_btn.add_css_class("suggested-action");
    save_btn.set_halign(gtk::Align::End);
    
    // Connect save button
    let lang_combo_clone = lang_combo.clone();
    let engine_combo_clone = engine_combo.clone();
    let hw_accel_switch_clone = hw_accel_switch.clone();
    let notif_switch_clone = notif_switch.clone();
    let theme_combo_clone = theme_combo.clone();
    
    save_btn.connect_clicked(move |_| {
        let mut config = config::load_config().unwrap_or_default();
        
        // Update config from UI
        if let Some(lang) = lang_combo_clone.active_id() {
            config.language = lang.to_string();
        }
        if let Some(engine) = engine_combo_clone.active_id() {
            config.default_engine = engine.to_string();
        }
        config.hardware_acceleration = hw_accel_switch_clone.is_active();
        config.enable_notifications = notif_switch_clone.is_active();
        if let Some(theme) = theme_combo_clone.active_id() {
            config.theme = theme.to_string();
        }
        
        // Save config
        if let Err(e) = config::save_config(&config) {
            tracing::error!("Failed to save config: {}", e);
            utils::show_error_dialog(None, &format!("Failed to save settings: {}", e));
        } else {
            tracing::info!("Settings saved successfully");
            // Apply theme
            apply_theme(&config.theme);
        }
    });
    
    container.append(&save_btn);

    Ok(container)
}

/// Create About page
fn create_about_page() -> gtk::Box {
    let container = Box::new(Orientation::Vertical, 12);
    container.set_margin_top(12);
    container.set_margin_bottom(12);
    container.set_margin_start(12);
    container.set_margin_end(12);

    // App icon
    let icon = Image::from_icon_name("pwasforalllinux");
    icon.set_pixel_size(128);
    container.append(&icon);

    // Title
    let title = Label::new(Some("PWAsForAllLinux"));
    title.add_css_class("title-1");
    container.append(&title);

    // Version
    let version = Label::new(Some("Version 1.0.0"));
    version.add_css_class("dim-label");
    container.append(&version);

    // Description
    let description = Label::new(Some(
        "A tool to install, manage and use Progressive Web Apps (PWAs)\n\
         on any Linux distribution and desktop environment."
    ));
    description.add_css_class("dim-label");
    container.append(&description);

    // Separator
    container.append(&Separator::new(Orientation::Horizontal));

    // Links
    let links_box = Box::new(Orientation::Horizontal, 12);
    links_box.set_halign(gtk::Align::Center);

    let website_btn = Button::with_label("Website");
    links_box.append(&website_btn);

    let github_btn = Button::with_label("GitHub");
    links_box.append(&github_btn);

    let docs_btn = Button::with_label("Documentation");
    links_box.append(&docs_btn);

    container.append(&links_box);

    // Copyright
    let copyright = Label::new(Some("© 2024 PWAsForAllLinux Team\nLicensed under MIT License"));
    copyright.add_css_class("dim-label");
    copyright.add_css_class("caption");
    container.append(&copyright);

    container
}

/// Apply theme to the application
fn apply_theme(theme: &str) {
    let settings = gtk::Settings::default();
    if let Some(settings) = settings {
        match theme {
            "dark" => {
                settings.set_gtk_application_prefer_dark_theme(true);
            }
            "light" => {
                settings.set_gtk_application_prefer_dark_theme(false);
            }
            "system" => {
                // Use system default
                settings.set_gtk_application_prefer_dark_theme(false);
            }
            _ => {}
        }
    }
}
