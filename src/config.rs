//! Configuration management for PWAsForAllLinux

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Default browser engine (webkit, firefox, chromium)
    pub default_engine: String,
    /// Default window width for new PWAs
    pub default_width: i32,
    /// Default window height for new PWAs
    pub default_height: i32,
    /// Enable hardware acceleration
    pub hardware_acceleration: bool,
    /// Enable notifications
    pub enable_notifications: bool,
    /// Custom user agent (empty = default)
    pub custom_user_agent: String,
    /// Theme (system, light, dark)
    pub theme: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            default_engine: "webkit".to_string(),
            default_width: 1280,
            default_height: 800,
            hardware_acceleration: true,
            enable_notifications: true,
            custom_user_agent: String::new(),
            theme: "system".to_string(),
        }
    }
}

/// Get the configuration directory path
pub fn config_dir() -> Result<PathBuf> {
    let config_dir = dirs::config_dir()
        .context("Could not determine config directory")?
        .join("pwasforalllinux");
    Ok(config_dir)
}

/// Get the data directory path
pub fn data_dir() -> Result<PathBuf> {
    let data_dir = dirs::data_dir()
        .context("Could not determine data directory")?
        .join("pwasforalllinux");
    Ok(data_dir)
}

/// Get the cache directory path
pub fn cache_dir() -> Result<PathBuf> {
    let cache_dir = dirs::cache_dir()
        .context("Could not determine cache directory")?
        .join("pwasforalllinux");
    Ok(cache_dir)
}

/// Get the icons directory path
pub fn icons_dir() -> Result<PathBuf> {
    let icons_dir = data_dir()?.join("icons");
    Ok(icons_dir)
}

/// Get the PWAs directory path
pub fn pwas_dir() -> Result<PathBuf> {
    let pwas_dir = data_dir()?.join("pwas");
    Ok(pwas_dir)
}

/// Ensure configuration directories exist
pub fn ensure_config_dir() -> Result<()> {
    let dirs_to_create = vec![
        config_dir()?,
        data_dir()?,
        cache_dir()?,
        icons_dir()?,
        pwas_dir()?,
    ];

    for dir in dirs_to_create {
        if !dir.exists() {
            fs::create_dir_all(&dir)
                .with_context(|| format!("Failed to create directory: {:?}", dir))?;
            tracing::info!("Created directory: {:?}", dir);
        }
    }

    Ok(())
}

/// Load configuration from file
pub fn load_config() -> Result<Config> {
    let config_path = config_dir()?.join("config.json");

    if !config_path.exists() {
        let default_config = Config::default();
        save_config(&default_config)?;
        return Ok(default_config);
    }

    let content = fs::read_to_string(&config_path)
        .with_context(|| format!("Failed to read config file: {:?}", config_path))?;

    let config: Config = serde_json::from_str(&content)
        .with_context(|| "Failed to parse config file")?;

    Ok(config)
}

/// Save configuration to file
pub fn save_config(config: &Config) -> Result<()> {
    let config_path = config_dir()?.join("config.json");

    let content = serde_json::to_string_pretty(config)
        .context("Failed to serialize config")?;

    fs::write(&config_path, content)
        .with_context(|| format!("Failed to write config file: {:?}", config_path))?;

    Ok(())
}

/// Get the applications directory (for .desktop files)
pub fn applications_dir() -> Result<PathBuf> {
    let applications_dir = dirs::data_dir()
        .context("Could not determine data directory")?
        .join("applications");
    Ok(applications_dir)
}
