//! Utility functions for PWAsForAllLinux

use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{MessageDialog, MessageType, ButtonsType, Window};

/// Show an error dialog
pub fn show_error_dialog(parent: Option<&Window>, message: &str) {
    let dialog = MessageDialog::builder()
        .message_type(MessageType::Error)
        .buttons(ButtonsType::Close)
        .text("Error")
        .secondary_text(message)
        .build();

    if let Some(p) = parent {
        dialog.set_transient_for(Some(p));
    }

    dialog.connect_response(|dialog, _| {
        dialog.close();
    });

    dialog.present();
}

/// Show an info dialog
pub fn show_info_dialog(parent: Option<&Window>, title: &str, message: &str) {
    let dialog = MessageDialog::builder()
        .message_type(MessageType::Info)
        .buttons(ButtonsType::Ok)
        .text(title)
        .secondary_text(message)
        .build();

    if let Some(p) = parent {
        dialog.set_transient_for(Some(p));
    }

    dialog.connect_response(|dialog, _| {
        dialog.close();
    });

    dialog.present();
}

/// Show a question dialog and return true if user clicked "Yes"
pub async fn show_question_dialog(parent: Option<&Window>, title: &str, message: &str) -> bool {
    let dialog = MessageDialog::builder()
        .message_type(MessageType::Question)
        .buttons(ButtonsType::YesNo)
        .text(title)
        .secondary_text(message)
        .build();

    if let Some(p) = parent {
        dialog.set_transient_for(Some(p));
    }

    let response = dialog.run_future().await;
    dialog.close();

    response == gtk::ResponseType::Yes
}

/// Validate a URL
pub fn is_valid_url(url: &str) -> bool {
    url::Url::parse(url)
        .map(|u| u.scheme() == "http" || u.scheme() == "https")
        .unwrap_or(false)
}

/// Sanitize a filename
pub fn sanitize_filename(name: &str) -> String {
    name.chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '-' || c == '_' || c == '.' {
                c
            } else {
                '_'
            }
        })
        .collect()
}

/// Format bytes to human readable string
pub fn format_bytes(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

/// Get system information
pub fn get_system_info() -> SystemInfo {
    SystemInfo {
        os_name: std::env::consts::OS.to_string(),
        arch: std::env::consts::ARCH.to_string(),
        desktop_env: std::env::var("XDG_CURRENT_DESKTOP")
            .unwrap_or_else(|_| "Unknown".to_string()),
        gtk_version: format!(
            "{}.{}.{}",
            gtk::major_version(),
            gtk::minor_version(),
            gtk::micro_version()
        ),
    }
}

/// System information
pub struct SystemInfo {
    pub os_name: String,
    pub arch: String,
    pub desktop_env: String,
    pub gtk_version: String,
}
