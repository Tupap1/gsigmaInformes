use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Estructura de configuración persistida en config.json.
/// Almacena ÚNICAMENTE el host, puerto y usuarios limitados.
/// NUNCA se guarda la contraseña de root.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
    pub read_user: String,
    pub read_password: String,
    pub write_user: String,
    pub write_password: String,
}

impl AppConfig {
    /// Crea una configuración con valores por defecto para los usuarios limitados.
    pub fn new(host: String, port: u16) -> Self {
        Self {
            host,
            port,
            read_user: "reci_read".to_string(),
            read_password: "read_pass_123".to_string(),
            write_user: "reci_write".to_string(),
            write_password: "write_pass_123".to_string(),
        }
    }
}

/// Determina la ruta del config.json.
/// En producción usa el directorio AppData de Tauri.
/// En tests o si se provee un directorio override, usa ese directorio.
pub fn get_config_path(override_dir: Option<&Path>) -> PathBuf {
    if let Some(dir) = override_dir {
        return dir.join("config.json");
    }

    // Ruta estándar AppData: C:\Users\<User>\AppData\Roaming\com.recicladoraboyaca.informes\
    let app_data = std::env::var("APPDATA")
        .unwrap_or_else(|_| {
            dirs::config_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .to_string_lossy()
                .to_string()
        });

    PathBuf::from(app_data)
        .join("com.recicladoraboyaca.informes")
        .join("config.json")
}

/// Lee la configuración desde el archivo config.json.
pub fn load_config(override_dir: Option<&Path>) -> Option<AppConfig> {
    let path = get_config_path(override_dir);
    if !path.exists() {
        return None;
    }

    let content = std::fs::read_to_string(&path).ok()?;
    serde_json::from_str(&content).ok()
}

/// Escribe la configuración al archivo config.json, creando directorios padre si es necesario.
pub fn save_config(config: &AppConfig, override_dir: Option<&Path>) -> Result<(), String> {
    let path = get_config_path(override_dir);

    // Crear directorio padre si no existe
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| {
            format!("No se pudo crear el directorio de configuración: {}", e)
        })?;
    }

    let json = serde_json::to_string_pretty(config).map_err(|e| {
        format!("Error al serializar la configuración: {}", e)
    })?;

    std::fs::write(&path, json).map_err(|e| {
        format!("Error al escribir el archivo de configuración: {}", e)
    })?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_config_roundtrip() {
        let temp_dir = std::env::temp_dir().join("recicladora_config_test");
        let _ = fs::remove_dir_all(&temp_dir);
        fs::create_dir_all(&temp_dir).unwrap();

        let config = AppConfig::new("192.168.1.100".to_string(), 3306);
        assert_eq!(config.read_user, "reci_read");
        assert_eq!(config.write_user, "reci_write");

        // Guardar
        save_config(&config, Some(&temp_dir)).unwrap();

        // Cargar
        let loaded = load_config(Some(&temp_dir)).unwrap();
        assert_eq!(loaded.host, "192.168.1.100");
        assert_eq!(loaded.port, 3306);
        assert_eq!(loaded.read_user, "reci_read");
        assert_eq!(loaded.read_password, "read_pass_123");
        assert_eq!(loaded.write_user, "reci_write");
        assert_eq!(loaded.write_password, "write_pass_123");

        // Limpiar
        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_load_nonexistent_config() {
        let temp_dir = std::env::temp_dir().join("recicladora_config_noexist");
        let _ = std::fs::remove_dir_all(&temp_dir);

        let result = load_config(Some(&temp_dir));
        assert!(result.is_none());
    }
}
