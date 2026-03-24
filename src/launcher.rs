//! PWA Launcher - Launches individual PWAs as standalone applications

use anyhow::{Context, Result};
use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};
use std::env;
use webkit2gtk as webkit;
use webkit::prelude::*;

mod config;
mod pwa;

const APP_ID_PREFIX: &str = "com.pwasforalllinux.Pwa.";

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: pwa-launcher <pwa-id>");
        std::process::exit(1);
    }

    let pwa_id = &args[1];

    // Load PWA
    let manager = pwa::PwaManager::new()?;
    let pwa = manager.get_by_id(pwa_id)
        .context(format!("PWA with ID '{}' not found", pwa_id))?
        .clone();

    // Record launch
    let mut manager = pwa::PwaManager::new()?;
    manager.record_launch(pwa_id)?;

    // Create GTK application with unique ID for this PWA
    let app_id = format!("{}{}", APP_ID_PREFIX, pwa.id);
    let app = Application::builder()
        .application_id(&app_id)
        .build();

    let pwa_clone = pwa.clone();
    app.connect_activate(move |app| {
        if let Err(e) = build_pwa_window(app, &pwa_clone) {
            eprintln!("Failed to build PWA window: {}", e);
        }
    });

    // Run application
    app.run();

    Ok(())
}

/// Build the PWA window
fn build_pwa_window(app: &Application, pwa: &pwa::Pwa) -> Result<()> {
    // Create window
    let window = ApplicationWindow::builder()
        .application(app)
        .title(&pwa.name)
        .default_width(pwa.width)
        .default_height(pwa.height)
        .build();

    // Set window icon if available
    if let Some(icon_path) = &pwa.icon_path {
        let file = gtk::gio::File::for_path(icon_path);
        if let Ok(texture) = gtk::gdk::Texture::from_file(&file) {
            window.set_icon(Some(&texture));
        }
    }

    // Create WebView
    let webview = webkit::WebView::builder()
        .build();

    // Configure settings
    if let Some(settings) = webview.settings() {
        settings.set_enable_javascript(true);
        settings.set_enable_page_cache(true);
        settings.set_enable_developer_extras(false);
        settings.set_enable_offline_web_application_cache(true);
        settings.set_enable_local_storage(true);
        settings.set_enable_database(true);
        settings.set_enable_dns_prefetching(true);
        settings.set_enable_hyperlink_auditing(false);
        settings.set_enable_smooth_scrolling(true);
        settings.set_enable_webgl(true);
        settings.set_enable_media_stream(true);
        settings.set_enable_mediasource(true);
        settings.set_enable_encrypted_media(true);
        
        // Set user agent
        let config = config::load_config().unwrap_or_default();
        if !config.custom_user_agent.is_empty() {
            settings.set_user_agent(&config.custom_user_agent);
        } else {
            settings.set_user_agent(&format!(
                "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.0 PWAsForAllLinux/1.0"
            ));
        }
    }

    // Create web context with custom profile
    let profile_path = pwa.profile_path()?;
    let context = webkit::WebContext::new();
    
    // Set up persistent storage
    if let Some(website_data_manager) = context.website_data_manager() {
        let base_path = profile_path.to_string_lossy();
        website_data_manager.set_base_data_directory(&base_path);
        website_data_manager.set_base_cache_directory(&format!("{}/cache", base_path));
    }

    // Load URL
    webview.load_uri(&pwa.url);

    // Handle navigation events
    webview.connect_load_changed(|webview, load_event| {
        match load_event {
            webkit::LoadEvent::Started => {
                tracing::info!("Loading started");
            }
            webkit::LoadEvent::Finished => {
                tracing::info!("Loading finished");
            }
            _ => {}
        }
    });

    // Handle new window policy decision (open links in same window for app-like experience)
    webview.connect_decide_policy(move |_webview, decision, policy_type| {
        if policy_type == webkit::PolicyDecisionType::NewWindowAction {
            if let Some(nav_decision) = decision.downcast_ref::<webkit::NavigationPolicyDecision>() {
                if let Some(nav_action) = nav_decision.navigation_action() {
                    if let Some(uri) = nav_action.request().uri() {
                        // For external links, open in default browser
                        let uri_str = uri.to_string();
                        if uri_str.starts_with("http://") || uri_str.starts_with("https://") {
                            // Open in default browser
                            let _ = open_url_in_browser(&uri_str);
                            nav_decision.ignore();
                            return true;
                        }
                    }
                }
            }
        }
        false
    });

    // Inject custom CSS if provided
    if let Some(css) = &pwa.custom_css {
        let script = format!(
            r#"
            (function() {{
                const style = document.createElement('style');
                style.textContent = `{}`;
                document.head.appendChild(style);
            }})();
            "#,
            css
        );
        
        let user_content_manager = webview.user_content_manager()
            .unwrap_or_else(|| webkit::UserContentManager::new());
        
        user_content_manager.add_script(
            &webkit::UserScript::new(
                &script,
                webkit::UserContentInjectedFrames::AllFrames,
                webkit::UserScriptInjectionTime::End,
                None,
                None,
            )
        );
    }

    window.set_child(Some(&webview));
    window.present();

    Ok(())
}

/// Open URL in default browser
fn open_url_in_browser(url: &str) -> Result<()> {
    open::that(url)
        .context("Failed to open URL in default browser")?;
    Ok(())
}
