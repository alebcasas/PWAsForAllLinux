//! PWA (Progressive Web App) management

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::PathBuf;
use uuid::Uuid;

use crate::config;

/// Represents an installed PWA
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pwa {
    /// Unique identifier
    pub id: String,
    /// PWA name
    pub name: String,
    /// PWA description
    pub description: Option<String>,
    /// Start URL
    pub url: String,
    /// Scope URL
    pub scope: Option<String>,
    /// Icon path (local)
    pub icon_path: Option<String>,
    /// Remote icon URL
    pub icon_url: Option<String>,
    /// Window width
    pub width: i32,
    /// Window height
    pub height: i32,
    /// Display mode (standalone, minimal-ui, fullscreen)
    pub display_mode: String,
    /// Theme color
    pub theme_color: Option<String>,
    /// Background color
    pub background_color: Option<String>,
    /// Categories
    pub categories: Vec<String>,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last launch timestamp
    pub last_launched_at: Option<DateTime<Utc>>,
    /// Launch count
    pub launch_count: i32,
    /// Is enabled
    pub enabled: bool,
    /// Custom CSS injections
    pub custom_css: Option<String>,
    /// Custom JS injections
    pub custom_js: Option<String>,
    /// Profile name (for isolated sessions)
    pub profile: String,
}

impl Pwa {
    /// Create a new PWA
    pub fn new(name: String, url: String) -> Self {
        let id = Uuid::new_v4().to_string();
        let profile = format!("profile_{}", &id[..8]);
        let url_hash = Self::generate_url_hash(&url);

        Self {
            id: if url_hash.len() > 8 {
                format!("pwa_{}", &url_hash[..8])
            } else {
                format!("pwa_{}", url_hash)
            },
            name,
            description: None,
            url,
            scope: None,
            icon_path: None,
            icon_url: None,
            width: 1280,
            height: 800,
            display_mode: "standalone".to_string(),
            theme_color: None,
            background_color: None,
            categories: vec![],
            created_at: Utc::now(),
            last_launched_at: None,
            launch_count: 0,
            enabled: true,
            custom_css: None,
            custom_js: None,
            profile,
        }
    }

    /// Generate a hash from URL for ID
    fn generate_url_hash(url: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(url.as_bytes());
        let result = hasher.finalize();
        hex::encode(result)
    }

    /// Get the .desktop file path
    pub fn desktop_file_path(&self) -> Result<PathBuf> {
        let filename = format!("pwasforalllinux-{}.desktop", self.id);
        Ok(config::applications_dir()?.join(filename))
    }

    /// Get the profile directory path
    pub fn profile_path(&self) -> Result<PathBuf> {
        Ok(config::data_dir()?.join("profiles").join(&self.profile))
    }

    /// Get the icon path
    pub fn get_icon_path(&self) -> Result<PathBuf> {
        Ok(config::icons_dir()?.join(format!("{}.png", self.id)))
    }
}

/// Web App Manifest structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebAppManifest {
    pub name: Option<String>,
    pub short_name: Option<String>,
    pub description: Option<String>,
    pub start_url: Option<String>,
    pub scope: Option<String>,
    pub display: Option<String>,
    pub orientation: Option<String>,
    pub theme_color: Option<String>,
    pub background_color: Option<String>,
    pub icons: Option<Vec<ManifestIcon>>,
    pub categories: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManifestIcon {
    pub src: String,
    pub sizes: Option<String>,
    pub r#type: Option<String>,
    pub purpose: Option<String>,
}

/// PWA Manager
pub struct PwaManager {
    pwas: Vec<Pwa>,
    pwas_file: PathBuf,
}

impl PwaManager {
    /// Create a new PWA manager
    pub fn new() -> Result<Self> {
        let pwas_file = config::data_dir()?.join("pwas.json");
        let pwas = if pwas_file.exists() {
            let content = fs::read_to_string(&pwas_file)
                .context("Failed to read PWAs file")?;
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            vec![]
        };

        Ok(Self { pwas, pwas_file })
    }

    /// Get all PWAs
    pub fn get_all(&self) -> &[Pwa] {
        &self.pwas
    }

    /// Get a PWA by ID
    pub fn get_by_id(&self, id: &str) -> Option<&Pwa> {
        self.pwas.iter().find(|p| p.id == id)
    }

    /// Add a new PWA with validation
    pub fn add(&mut self, mut pwa: Pwa) -> Result<()> {
        // Validate PWA data
        self.validate_pwa(&pwa)?;

        // Check for duplicates
        if self.pwas.iter().any(|p| p.url == pwa.url) {
            return Err(anyhow::anyhow!("A PWA with this URL already exists"));
        }

        // Create profile directory
        let profile_path = pwa.profile_path()?;
        fs::create_dir_all(&profile_path)
            .context("Failed to create profile directory")?;

        // Create .desktop file
        self.create_desktop_file(&pwa)?;

        // Verify .desktop file was created
        let desktop_path = pwa.desktop_file_path()?;
        if !desktop_path.exists() {
            return Err(anyhow::anyhow!("Failed to create .desktop file"));
        }

        // Add to list
        pwa.created_at = Utc::now();
        let pwa_clone = pwa.clone();
        self.pwas.push(pwa);

        // Save
        self.save()?;

        // Verify installation
        self.verify_installation(&pwa_clone)?;

        Ok(())
    }

    /// Validate PWA data
    fn validate_pwa(&self, pwa: &Pwa) -> Result<()> {
        // Validate name
        if pwa.name.trim().is_empty() {
            return Err(anyhow::anyhow!("PWA name cannot be empty"));
        }

        // Validate URL
        if pwa.url.trim().is_empty() {
            return Err(anyhow::anyhow!("PWA URL cannot be empty"));
        }

        // Validate URL format
        if !pwa.url.starts_with("http://") && !pwa.url.starts_with("https://") {
            return Err(anyhow::anyhow!("PWA URL must start with http:// or https://"));
        }

        // Validate dimensions
        if pwa.width < 100 || pwa.width > 7680 {
            return Err(anyhow::anyhow!("Window width must be between 100 and 7680"));
        }

        if pwa.height < 100 || pwa.height > 4320 {
            return Err(anyhow::anyhow!("Window height must be between 100 and 4320"));
        }

        // Validate display mode
        let valid_modes = ["standalone", "minimal-ui", "fullscreen"];
        if !valid_modes.contains(&pwa.display_mode.as_str()) {
            return Err(anyhow::anyhow!("Invalid display mode: {}", pwa.display_mode));
        }

        Ok(())
    }

    /// Verify PWA installation
    fn verify_installation(&self, pwa: &Pwa) -> Result<()> {
        // Check .desktop file exists
        let desktop_path = pwa.desktop_file_path()?;
        if !desktop_path.exists() {
            return Err(anyhow::anyhow!(".desktop file was not created"));
        }

        // Check .desktop file is readable
        let content = fs::read_to_string(&desktop_path)
            .context("Failed to read .desktop file")?;

        // Verify .desktop file has required fields
        if !content.contains("Exec=pwa-launcher") {
            return Err(anyhow::anyhow!(".desktop file is missing Exec field"));
        }

        if !content.contains("Name=") {
            return Err(anyhow::anyhow!(".desktop file is missing Name field"));
        }

        // Check profile directory exists
        let profile_path = pwa.profile_path()?;
        if !profile_path.exists() {
            return Err(anyhow::anyhow!("Profile directory was not created"));
        }

        tracing::info!("PWA installation verified successfully: {}", pwa.name);
        Ok(())
    }

    /// Update a PWA
    pub fn update(&mut self, pwa: &Pwa) -> Result<()> {
        if let Some(existing) = self.pwas.iter_mut().find(|p| p.id == pwa.id) {
            *existing = pwa.clone();
            self.save()?;
        }
        Ok(())
    }

    /// Remove a PWA
    pub fn remove(&mut self, id: &str) -> Result<()> {
        if let Some(pwa) = self.pwas.iter().find(|p| p.id == id).cloned() {
            // Remove .desktop file
            let desktop_path = pwa.desktop_file_path()?;
            if desktop_path.exists() {
                fs::remove_file(&desktop_path)?;
            }

            // Remove profile directory
            let profile_path = pwa.profile_path()?;
            if profile_path.exists() {
                fs::remove_dir_all(&profile_path)?;
            }

            // Remove icon
            let icon_path = pwa.get_icon_path()?;
            if icon_path.exists() {
                fs::remove_file(&icon_path)?;
            }

            // Remove from list
            self.pwas.retain(|p| p.id != id);
            self.save()?;
        }
        Ok(())
    }

    /// Record a launch
    pub fn record_launch(&mut self, id: &str) -> Result<()> {
        if let Some(pwa) = self.pwas.iter_mut().find(|p| p.id == id) {
            pwa.last_launched_at = Some(Utc::now());
            pwa.launch_count += 1;
            self.save()?;
        }
        Ok(())
    }

    /// Save PWAs to file
    fn save(&self) -> Result<()> {
        let content = serde_json::to_string_pretty(&self.pwas)
            .context("Failed to serialize PWAs")?;
        fs::write(&self.pwas_file, content)
            .context("Failed to write PWAs file")?;
        Ok(())
    }

    /// Create .desktop file for a PWA
    fn create_desktop_file(&self, pwa: &Pwa) -> Result<()> {
        let desktop_path = pwa.desktop_file_path()?;

        // Ensure applications directory exists
        if let Some(parent) = desktop_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let icon_path = pwa.get_icon_path()?;
        let icon_str = if icon_path.exists() {
            icon_path.to_string_lossy().to_string()
        } else {
            "pwasforalllinux".to_string()
        };

        let content = format!(
            r#"[Desktop Entry]
Version=1.0
Name={}
Comment={}
Exec=pwa-launcher {}
Icon={}
Terminal=false
Type=Application
Categories=WebApp;{}
StartupNotify=true
StartupWMClass={}
MimeType=text/html;text/xml;application/xhtml+xml;
X-GNOME-Autostart-enabled=true
"#,
            pwa.name,
            pwa.description.as_deref().unwrap_or(""),
            pwa.id,
            icon_str,
            pwa.categories.join(";"),
            pwa.id
        );

        fs::write(&desktop_path, content)?;
        tracing::info!("Created desktop file: {:?}", desktop_path);

        Ok(())
    }
}

impl Default for PwaManager {
    fn default() -> Self {
        Self::new().expect("Failed to create PWA manager")
    }
}

/// Fetch web app manifest from URL with improved error handling
pub async fn fetch_manifest(url: &str) -> Result<WebAppManifest> {
    let client = reqwest::Client::builder()
        .user_agent("PWAsForAllLinux/1.0")
        .timeout(std::time::Duration::from_secs(10))
        .build()?;

    // Fetch HTML with error handling
    let response = match client.get(url).send().await {
        Ok(resp) => resp,
        Err(e) => {
            tracing::error!("Network error fetching URL {}: {}", url, e);
            return Err(anyhow::anyhow!("Failed to fetch URL: {}", e));
        }
    };

    if !response.status().is_success() {
        return Err(anyhow::anyhow!("HTTP error: {}", response.status()));
    }

    let html = match response.text().await {
        Ok(text) => text,
        Err(e) => {
            tracing::error!("Failed to read response text: {}", e);
            return Err(anyhow::anyhow!("Failed to read response: {}", e));
        }
    };

    // Try to find manifest URL
    match extract_manifest_url(&html, url) {
        Ok(manifest_url) => {
            // Fetch manifest with error handling
            let manifest_response = match client.get(&manifest_url).send().await {
                Ok(resp) => resp,
                Err(e) => {
                    tracing::warn!("Failed to fetch manifest from {}: {}", manifest_url, e);
                    return Ok(create_fallback_manifest(url, &html));
                }
            };

            if !manifest_response.status().is_success() {
                tracing::warn!("Manifest request failed with status: {}", manifest_response.status());
                return Ok(create_fallback_manifest(url, &html));
            }

            match manifest_response.json::<WebAppManifest>().await {
                Ok(manifest) => Ok(manifest),
                Err(e) => {
                    tracing::warn!("Failed to parse manifest JSON: {}", e);
                    Ok(create_fallback_manifest(url, &html))
                }
            }
        }
        Err(e) => {
            tracing::info!("No manifest found in HTML: {}", e);
            Ok(create_fallback_manifest(url, &html))
        }
    }
}

/// Create fallback manifest from HTML when manifest is not available
fn create_fallback_manifest(url: &str, html: &str) -> WebAppManifest {
    let mut manifest = WebAppManifest {
        name: None,
        short_name: None,
        description: None,
        start_url: Some(url.to_string()),
        scope: None,
        display: Some("standalone".to_string()),
        orientation: None,
        theme_color: None,
        background_color: None,
        icons: None,
        categories: None,
    };

    // Try to extract title from HTML
    if let Some(title_match) = regex::Regex::new(r#"<title[^>]*>([^<]+)</title>"#)
        .ok()
        .and_then(|re| re.captures(html))
    {
        manifest.name = Some(title_match.get(1).unwrap().as_str().trim().to_string());
    }

    // Try to extract description from meta tag
    if let Some(desc_match) = regex::Regex::new(r#"<meta[^>]*name=["']description["'][^>]*content=["']([^"']+)["']"#)
        .ok()
        .and_then(|re| re.captures(html))
    {
        manifest.description = Some(desc_match.get(1).unwrap().as_str().trim().to_string());
    }

    // Try to extract theme color from meta tag
    if let Some(color_match) = regex::Regex::new(r#"<meta[^>]*name=["']theme-color["'][^>]*content=["']([^"']+)["']"#)
        .ok()
        .and_then(|re| re.captures(html))
    {
        manifest.theme_color = Some(color_match.get(1).unwrap().as_str().trim().to_string());
    }

    // Try to find icon from link rel="icon"
    if let Some(icon_match) = regex::Regex::new(r#"<link[^>]*rel=["']icon["'][^>]*href=["']([^"']+)["']"#)
        .ok()
        .and_then(|re| re.captures(html))
    {
        let icon_href = icon_match.get(1).unwrap().as_str();
        let icon_url = if icon_href.starts_with("http") {
            icon_href.to_string()
        } else if let Ok(base) = url::Url::parse(url) {
            base.join(icon_href).map(|u| u.to_string()).unwrap_or_else(|_| icon_href.to_string())
        } else {
            icon_href.to_string()
        };

        manifest.icons = Some(vec![ManifestIcon {
            src: icon_url,
            sizes: None,
            r#type: None,
            purpose: Some("any".to_string()),
        }]);
    }

    manifest
}

/// Extract manifest URL from HTML with improved parsing
fn extract_manifest_url(html: &str, base_url: &str) -> Result<String> {
    // Look for <link rel="manifest" href="...">
    let re = regex::Regex::new(r#"<link[^>]*rel=["']manifest["'][^>]*href=["']([^"']+)["']"#)?;
    
    if let Some(caps) = re.captures(html) {
        let href = caps.get(1).unwrap().as_str();
        
        // Resolve relative URL
        if href.starts_with("http") {
            return Ok(href.to_string());
        } else {
            let base = url::Url::parse(base_url)?;
            let resolved = base.join(href)?;
            return Ok(resolved.to_string());
        }
    }

    // Also look for <link href="..." rel="manifest">
    let re2 = regex::Regex::new(r#"<link[^>]*href=["']([^"']+)["'][^>]*rel=["']manifest["']"#)?;
    
    if let Some(caps) = re2.captures(html) {
        let href = caps.get(1).unwrap().as_str();
        
        // Resolve relative URL
        if href.starts_with("http") {
            return Ok(href.to_string());
        } else {
            let base = url::Url::parse(base_url)?;
            let resolved = base.join(href)?;
            return Ok(resolved.to_string());
        }
    }

    anyhow::bail!("No manifest link found in HTML")
}
