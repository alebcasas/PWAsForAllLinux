//! PWAsForAllLinux - A Progressive Web App manager for Linux
//!
//! This application allows users to install, manage, and run Progressive Web Apps
//! as standalone applications on any Linux distribution.

mod app;
mod config;
mod pwa;
mod utils;

use anyhow::Result;
use gtk4::prelude::*;
use gtk4::Application;
use tracing_subscriber::EnvFilter;

const APP_ID: &str = "com.pwasforalllinux.PWAsForAllLinux";

fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("info"))
        )
        .init();

    tracing::info!("Starting PWAsForAllLinux...");

    // Ensure config directory exists
    config::ensure_config_dir()?;

    // Create GTK application
    let app = Application::builder()
        .application_id(APP_ID)
        .build();

    // Connect activate signal
    app.connect_activate(|app| {
        if let Err(e) = app::build_ui(app) {
            tracing::error!("Failed to build UI: {}", e);
            utils::show_error_dialog(None, &format!("Failed to start application: {}", e));
        }
    });

    // Run application
    app.run();

    Ok(())
}
