//! Internationalization (i18n) system for PWAsForAllLinux
//! 
//! Provides centralized translation management with support for multiple languages.

use std::collections::HashMap;
use std::sync::OnceLock;

/// Global translations storage
static TRANSLATIONS: OnceLock<HashMap<String, HashMap<String, String>>> = OnceLock::new();

/// Initialize translations
fn init_translations() -> HashMap<String, HashMap<String, String>> {
    let mut translations = HashMap::new();
    
    // English translations
    let mut en = HashMap::new();
    
    // General UI
    en.insert("app_name".to_string(), "PWAsForAllLinux".to_string());
    en.insert("my_pwas".to_string(), "My PWAs".to_string());
    en.insert("add_pwa".to_string(), "Add PWA".to_string());
    en.insert("settings".to_string(), "Settings".to_string());
    en.insert("about".to_string(), "About".to_string());
    
    // PWAs page
    en.insert("installed_pwas".to_string(), "Installed Progressive Web Apps".to_string());
    en.insert("no_pwas_installed".to_string(), "No PWAs installed yet.\nClick 'Add PWA' to install your first web app!".to_string());
    en.insert("refresh_list".to_string(), "Refresh list".to_string());
    en.insert("launch".to_string(), "Launch".to_string());
    en.insert("edit_pwa".to_string(), "Edit PWA".to_string());
    en.insert("delete_pwa".to_string(), "Delete PWA".to_string());
    
    // Add PWA page
    en.insert("add_new_pwa".to_string(), "Add New PWA".to_string());
    en.insert("website_url".to_string(), "Website URL:".to_string());
    en.insert("name".to_string(), "Name:".to_string());
    en.insert("window_width".to_string(), "Window Width:".to_string());
    en.insert("window_height".to_string(), "Window Height:".to_string());
    en.insert("display_mode".to_string(), "Display Mode:".to_string());
    en.insert("standalone".to_string(), "Standalone (Recommended)".to_string());
    en.insert("minimal_ui".to_string(), "Minimal UI".to_string());
    en.insert("fullscreen".to_string(), "Fullscreen".to_string());
    en.insert("auto_detect".to_string(), "Auto-detect from URL".to_string());
    en.insert("install_pwa".to_string(), "Install PWA".to_string());
    en.insert("help_tip".to_string(), "💡 Tip: Click 'Auto-detect from URL' to automatically fetch\n         the web app manifest and fill in the details.".to_string());
    
    // Settings page
    en.insert("language".to_string(), "Language:".to_string());
    en.insert("english".to_string(), "English".to_string());
    en.insert("spanish".to_string(), "Español".to_string());
    en.insert("browser_engine".to_string(), "Browser Engine:".to_string());
    en.insert("webkit_default".to_string(), "WebKitGTK (Default)".to_string());
    en.insert("firefox".to_string(), "Firefox".to_string());
    en.insert("chromium".to_string(), "Chromium".to_string());
    en.insert("hardware_acceleration".to_string(), "Hardware Acceleration:".to_string());
    en.insert("enable_notifications".to_string(), "Enable Notifications:".to_string());
    en.insert("theme".to_string(), "Theme:".to_string());
    en.insert("system_default".to_string(), "System Default".to_string());
    en.insert("light".to_string(), "Light".to_string());
    en.insert("dark".to_string(), "Dark".to_string());
    en.insert("save_settings".to_string(), "Save Settings".to_string());
    
    // About page
    en.insert("version".to_string(), "Version 1.0.0".to_string());
    en.insert("description".to_string(), "A tool to install, manage and use Progressive Web Apps (PWAs)\n         on any Linux distribution and desktop environment.".to_string());
    en.insert("website".to_string(), "Website".to_string());
    en.insert("github".to_string(), "GitHub".to_string());
    en.insert("documentation".to_string(), "Documentation".to_string());
    en.insert("copyright".to_string(), "© 2024 PWAsForAllLinux Team\nLicensed under MIT License".to_string());
    
    // Dialogs
    en.insert("delete_confirmation".to_string(), "Delete PWA".to_string());
    en.insert("delete_confirmation_text".to_string(), "Are you sure you want to delete this PWA? This action cannot be undone.".to_string());
    en.insert("success".to_string(), "Success".to_string());
    en.insert("error".to_string(), "Error".to_string());
    en.insert("pwa_installed_successfully".to_string(), "PWA installed successfully!".to_string());
    en.insert("pwa_deleted_successfully".to_string(), "PWA deleted successfully!".to_string());
    en.insert("please_enter_url".to_string(), "Please enter a URL".to_string());
    en.insert("please_enter_name".to_string(), "Please enter a name for the PWA".to_string());
    en.insert("please_enter_url_first".to_string(), "Please enter a URL first".to_string());
    en.insert("failed_to_fetch_manifest".to_string(), "Failed to fetch manifest".to_string());
    en.insert("failed_to_add_pwa".to_string(), "Failed to add PWA".to_string());
    en.insert("failed_to_delete_pwa".to_string(), "Failed to delete PWA".to_string());
    en.insert("failed_to_load_pwa_manager".to_string(), "Failed to load PWA manager".to_string());
    en.insert("failed_to_save_settings".to_string(), "Failed to save settings".to_string());
    en.insert("settings_saved_successfully".to_string(), "Settings saved successfully".to_string());
    en.insert("manifest_fetched_successfully".to_string(), "Manifest fetched successfully".to_string());
    
    // Spanish translations
    let mut es = HashMap::new();
    
    // General UI
    es.insert("app_name".to_string(), "PWAsForAllLinux".to_string());
    es.insert("my_pwas".to_string(), "Mis PWAs".to_string());
    es.insert("add_pwa".to_string(), "Agregar PWA".to_string());
    es.insert("settings".to_string(), "Configuración".to_string());
    es.insert("about".to_string(), "Acerca de".to_string());
    
    // PWAs page
    es.insert("installed_pwas".to_string(), "Progressive Web Apps Instaladas".to_string());
    es.insert("no_pwas_installed".to_string(), "No hay PWAs instaladas todavía.\n¡Haz clic en 'Agregar PWA' para instalar tu primera aplicación web!".to_string());
    es.insert("refresh_list".to_string(), "Actualizar lista".to_string());
    es.insert("launch".to_string(), "Ejecutar".to_string());
    es.insert("edit_pwa".to_string(), "Editar PWA".to_string());
    es.insert("delete_pwa".to_string(), "Eliminar PWA".to_string());
    
    // Add PWA page
    es.insert("add_new_pwa".to_string(), "Agregar Nueva PWA".to_string());
    es.insert("website_url".to_string(), "URL del sitio web:".to_string());
    es.insert("name".to_string(), "Nombre:".to_string());
    es.insert("window_width".to_string(), "Ancho de ventana:".to_string());
    es.insert("window_height".to_string(), "Alto de ventana:".to_string());
    es.insert("display_mode".to_string(), "Modo de visualización:".to_string());
    es.insert("standalone".to_string(), "Independiente (Recomendado)".to_string());
    es.insert("minimal_ui".to_string(), "UI Mínima".to_string());
    es.insert("fullscreen".to_string(), "Pantalla completa".to_string());
    es.insert("auto_detect".to_string(), "Auto-detectar desde URL".to_string());
    es.insert("install_pwa".to_string(), "Instalar PWA".to_string());
    es.insert("help_tip".to_string(), "💡 Consejo: Haz clic en 'Auto-detectar desde URL' para obtener automáticamente\n         el manifiesto de la aplicación web y completar los detalles.".to_string());
    
    // Settings page
    es.insert("language".to_string(), "Idioma:".to_string());
    es.insert("english".to_string(), "English".to_string());
    es.insert("spanish".to_string(), "Español".to_string());
    es.insert("browser_engine".to_string(), "Motor de navegador:".to_string());
    es.insert("webkit_default".to_string(), "WebKitGTK (Por defecto)".to_string());
    es.insert("firefox".to_string(), "Firefox".to_string());
    es.insert("chromium".to_string(), "Chromium".to_string());
    es.insert("hardware_acceleration".to_string(), "Aceleración por hardware:".to_string());
    es.insert("enable_notifications".to_string(), "Habilitar notificaciones:".to_string());
    es.insert("theme".to_string(), "Tema:".to_string());
    es.insert("system_default".to_string(), "Por defecto del sistema".to_string());
    es.insert("light".to_string(), "Claro".to_string());
    es.insert("dark".to_string(), "Oscuro".to_string());
    es.insert("save_settings".to_string(), "Guardar configuración".to_string());
    
    // About page
    es.insert("version".to_string(), "Versión 1.0.0".to_string());
    es.insert("description".to_string(), "Una herramienta para instalar, gestionar y usar Progressive Web Apps (PWAs)\n         en cualquier distribución Linux y entorno de escritorio.".to_string());
    es.insert("website".to_string(), "Sitio web".to_string());
    es.insert("github".to_string(), "GitHub".to_string());
    es.insert("documentation".to_string(), "Documentación".to_string());
    es.insert("copyright".to_string(), "© 2024 PWAsForAllLinux Team\nLicencia MIT".to_string());
    
    // Dialogs
    es.insert("delete_confirmation".to_string(), "Eliminar PWA".to_string());
    es.insert("delete_confirmation_text".to_string(), "¿Estás seguro de que quieres eliminar esta PWA? Esta acción no se puede deshacer.".to_string());
    es.insert("success".to_string(), "Éxito".to_string());
    es.insert("error".to_string(), "Error".to_string());
    es.insert("pwa_installed_successfully".to_string(), "¡PWA instalada exitosamente!".to_string());
    es.insert("pwa_deleted_successfully".to_string(), "¡PWA eliminada exitosamente!".to_string());
    es.insert("please_enter_url".to_string(), "Por favor ingresa una URL".to_string());
    es.insert("please_enter_name".to_string(), "Por favor ingresa un nombre para la PWA".to_string());
    es.insert("please_enter_url_first".to_string(), "Por favor ingresa una URL primero".to_string());
    es.insert("failed_to_fetch_manifest".to_string(), "Error al obtener el manifiesto".to_string());
    es.insert("failed_to_add_pwa".to_string(), "Error al agregar la PWA".to_string());
    es.insert("failed_to_delete_pwa".to_string(), "Error al eliminar la PWA".to_string());
    es.insert("failed_to_load_pwa_manager".to_string(), "Error al cargar el gestor de PWAs".to_string());
    es.insert("failed_to_save_settings".to_string(), "Error al guardar la configuración".to_string());
    es.insert("settings_saved_successfully".to_string(), "Configuración guardada exitosamente".to_string());
    es.insert("manifest_fetched_successfully".to_string(), "Manifiesto obtenido exitosamente".to_string());
    
    translations.insert("en".to_string(), en);
    translations.insert("es".to_string(), es);
    
    translations
}

/// Get translation for a key in the specified language
pub fn t(key: &str) -> String {
    let config = crate::config::load_config().unwrap_or_default();
    let lang = config.language;
    
    let translations = TRANSLATIONS.get_or_init(init_translations);
    
    if let Some(lang_translations) = translations.get(&lang) {
        if let Some(translation) = lang_translations.get(key) {
            return translation.clone();
        }
    }
    
    // Fallback to English if translation not found
    if let Some(en_translations) = translations.get("en") {
        if let Some(translation) = en_translations.get(key) {
            return translation.clone();
        }
    }
    
    // Return the key itself if no translation found
    key.to_string()
}

/// Get translation for a key with parameters
pub fn t_params(key: &str, params: &[(&str, &str)]) -> String {
    let mut result = t(key);
    
    for (param_key, param_value) in params {
        result = result.replace(&format!("{{{}}}", param_key), param_value);
    }
    
    result
}

/// Get current language
pub fn get_current_language() -> String {
    let config = crate::config::load_config().unwrap_or_default();
    config.language
}

/// Check if current language is Spanish
pub fn is_spanish() -> bool {
    get_current_language() == "es"
}

/// Check if current language is English
pub fn is_english() -> bool {
    get_current_language() == "en"
}