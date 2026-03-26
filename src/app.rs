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

use crate::config;
use crate::pwa::{Pwa, PwaManager};
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

    // PWAs page
    let pwas_page = create_pwas_page()?;
    stack.add_titled(&pwas_page, Some("pwas"), "My PWAs");

    // Add PWA page
    let add_page = create_add_pwa_page()?;
    stack.add_titled(&add_page, Some("add"), "Add PWA");

    // Settings page
    let settings_page = create_settings_page()?;
    stack.add_titled(&settings_page, Some("settings"), "Settings");

    // About page
    let about_page = create_about_page();
    stack.add_titled(&about_page, Some("about"), "About");

    // Pack widgets
    main_box.append(&sidebar);
    main_box.append(&stack);

    window.set_child(Some(&main_box));
    window.present();

    Ok(())
}

/// Create PWAs list page
fn create_pwas_page() -> Result<gtk::Box> {
    let container = Box::new(Orientation::Vertical, 12);
    container.set_margin_top(12);
    container.set_margin_bottom(12);
    container.set_margin_start(12);
    container.set_margin_end(12);

    // Header
    let header_box = Box::new(Orientation::Horizontal, 12);
    let title = Label::new(Some("Installed Progressive Web Apps"));
    title.add_css_class("title-2");
    header_box.append(&title);

    // Refresh button
    let refresh_btn = Button::from_icon_name("view-refresh-symbolic");
    refresh_btn.set_tooltip_text(Some("Refresh list"));
    header_box.append(&refresh_btn);

    container.append(&header_box);

    // PWA list
    let scrolled = ScrolledWindow::new();
    scrolled.set_vexpand(true);
    scrolled.set_min_content_height(300);

    let list_box = ListBox::new();
    list_box.add_css_class("boxed-list");

    // Load PWAs
    let pwa_manager = PwaManager::new()?;
    let pwas = pwa_manager.get_all();

    if pwas.is_empty() {
        let empty_label = Label::new(Some("No PWAs installed yet.\nClick 'Add PWA' to install your first web app!"));
        empty_label.add_css_class("dim-label");
        list_box.append(&empty_label);
    } else {
        for pwa in pwas {
            let row = create_pwa_row(pwa)?;
            list_box.append(&row);
        }
    }

    scrolled.set_child(Some(&list_box));
    container.append(&scrolled);

    Ok(container)
}

/// Create a PWA list row
fn create_pwa_row(pwa: &Pwa) -> Result<ListBoxRow> {
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
    let launch_btn = Button::with_label("Launch");
    launch_btn.add_css_class("suggested-action");
    launch_btn.add_css_class("pill");
    let pwa_id_launch = pwa.id.clone();
    launch_btn.connect_clicked(move |_| {
        launch_pwa(&pwa_id_launch);
    });
    actions_box.append(&launch_btn);

    // Edit button
    let edit_btn = Button::from_icon_name("document-edit-symbolic");
    edit_btn.set_tooltip_text(Some("Edit PWA"));
    let pwa_id_edit = pwa.id.clone();
    edit_btn.connect_clicked(move |_| {
        edit_pwa(&pwa_id_edit);
    });
    actions_box.append(&edit_btn);

    // Delete button
    let delete_btn = Button::from_icon_name("user-trash-symbolic");
    delete_btn.add_css_class("destructive-action");
    delete_btn.set_tooltip_text(Some("Delete PWA"));
    let pwa_id_delete = pwa.id.clone();
    delete_btn.connect_clicked(move |_| {
        delete_pwa(&pwa_id_delete);
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
fn delete_pwa(pwa_id: &str) {
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
                        // TODO: Refresh the PWA list
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
fn create_add_pwa_page() -> Result<gtk::Box> {
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

    let auto_detect_btn = Button::with_label("Auto-detect from URL");
    btn_box.append(&auto_detect_btn);

    let install_btn = Button::with_label("Install PWA");
    install_btn.add_css_class("suggested-action");
    install_btn.add_css_class("pill");
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
    let title = Label::new(Some("Settings"));
    title.add_css_class("title-2");
    container.append(&title);

    // Settings form
    let form = Grid::new();
    form.set_row_spacing(12);
    form.set_column_spacing(12);
    form.set_margin_top(12);

    // Load config
    let config = config::load_config().unwrap_or_default();

    // Default engine
    let engine_label = Label::new(Some("Browser Engine:"));
    engine_label.set_halign(gtk::Align::Start);
    form.attach(&engine_label, 0, 0, 1, 1);

    let engine_combo = ComboBoxText::new();
    engine_combo.append(Some("webkit"), "WebKitGTK (Default)");
    engine_combo.append(Some("firefox"), "Firefox");
    engine_combo.append(Some("chromium"), "Chromium");
    engine_combo.set_active_id(Some(&config.default_engine));
    form.attach(&engine_combo, 1, 0, 2, 1);

    // Hardware acceleration
    let hw_accel_label = Label::new(Some("Hardware Acceleration:"));
    hw_accel_label.set_halign(gtk::Align::Start);
    form.attach(&hw_accel_label, 0, 1, 1, 1);

    let hw_accel_switch = Switch::new();
    hw_accel_switch.set_active(config.hardware_acceleration);
    form.attach(&hw_accel_switch, 1, 1, 1, 1);

    // Notifications
    let notif_label = Label::new(Some("Enable Notifications:"));
    notif_label.set_halign(gtk::Align::Start);
    form.attach(&notif_label, 0, 2, 1, 1);

    let notif_switch = Switch::new();
    notif_switch.set_active(config.enable_notifications);
    form.attach(&notif_switch, 1, 2, 1, 1);

    // Theme
    let theme_label = Label::new(Some("Theme:"));
    theme_label.set_halign(gtk::Align::Start);
    form.attach(&theme_label, 0, 3, 1, 1);

    let theme_combo = ComboBoxText::new();
    theme_combo.append(Some("system"), "System Default");
    theme_combo.append(Some("light"), "Light");
    theme_combo.append(Some("dark"), "Dark");
    theme_combo.set_active_id(Some(&config.theme));
    form.attach(&theme_combo, 1, 3, 2, 1);

    container.append(&form);

    // Separator
    container.append(&Separator::new(Orientation::Horizontal));

    // Save button
    let save_btn = Button::with_label("Save Settings");
    save_btn.add_css_class("suggested-action");
    save_btn.set_halign(gtk::Align::End);
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
