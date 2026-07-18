use crate::config::{self, AppConfig};
use crate::db::{AppState, create_pools_from_config};
use sqlx::mysql::MySqlPoolOptions;
use std::path::Path;
use tauri::State;

/// Sentencias SQL para crear los usuarios y asignar privilegios.
/// Compatible con MySQL 5.5 (usa IDENTIFIED BY en GRANT).
const SETUP_SQL: &[&str] = &[
    // Crear esquemas si no existen
    "CREATE DATABASE IF NOT EXISTS `adm` CHARACTER SET utf8 COLLATE utf8_general_ci",
    "CREATE DATABASE IF NOT EXISTS `pv` CHARACTER SET utf8 COLLATE utf8_general_ci",
    "CREATE DATABASE IF NOT EXISTS `sis` CHARACTER SET utf8 COLLATE utf8_general_ci",
    "CREATE DATABASE IF NOT EXISTS `tcc` CHARACTER SET utf8 COLLATE utf8_general_ci",
    // 1. Usuario Read-Only (reci_read)
    "GRANT SELECT ON `adm`.* TO 'reci_read'@'%' IDENTIFIED BY 'read_pass_123'",
    "GRANT SELECT ON `adm`.* TO 'reci_read'@'localhost' IDENTIFIED BY 'read_pass_123'",
    "GRANT SELECT ON `pv`.* TO 'reci_read'@'%' IDENTIFIED BY 'read_pass_123'",
    "GRANT SELECT ON `pv`.* TO 'reci_read'@'localhost' IDENTIFIED BY 'read_pass_123'",
    "GRANT SELECT ON `sis`.* TO 'reci_read'@'%' IDENTIFIED BY 'read_pass_123'",
    "GRANT SELECT ON `sis`.* TO 'reci_read'@'localhost' IDENTIFIED BY 'read_pass_123'",
    "GRANT SELECT ON `tcc`.* TO 'reci_read'@'%' IDENTIFIED BY 'read_pass_123'",
    "GRANT SELECT ON `tcc`.* TO 'reci_read'@'localhost' IDENTIFIED BY 'read_pass_123'",
    // 2. Usuario de Escritura Restringido (reci_write)
    "GRANT SELECT ON `adm`.* TO 'reci_write'@'%' IDENTIFIED BY 'write_pass_123'",
    "GRANT SELECT ON `adm`.* TO 'reci_write'@'localhost' IDENTIFIED BY 'write_pass_123'",
    "GRANT SELECT ON `pv`.* TO 'reci_write'@'%' IDENTIFIED BY 'write_pass_123'",
    "GRANT SELECT ON `pv`.* TO 'reci_write'@'localhost' IDENTIFIED BY 'write_pass_123'",
    "GRANT SELECT ON `sis`.* TO 'reci_write'@'%' IDENTIFIED BY 'write_pass_123'",
    "GRANT SELECT ON `sis`.* TO 'reci_write'@'localhost' IDENTIFIED BY 'write_pass_123'",
    "GRANT SELECT ON `tcc`.* TO 'reci_write'@'%' IDENTIFIED BY 'write_pass_123'",
    "GRANT SELECT ON `tcc`.* TO 'reci_write'@'localhost' IDENTIFIED BY 'write_pass_123'",
    // 3. Privilegios de escritura exclusivos en adm.trc y pv.proveedo
    "GRANT INSERT, UPDATE, DELETE ON `adm`.`trc` TO 'reci_write'@'%'",
    "GRANT INSERT, UPDATE, DELETE ON `adm`.`trc` TO 'reci_write'@'localhost'",
    "GRANT INSERT, UPDATE, DELETE ON `pv`.`proveedo` TO 'reci_write'@'%'",
    "GRANT INSERT, UPDATE, DELETE ON `pv`.`proveedo` TO 'reci_write'@'localhost'",
    // 4. Aplicar cambios de privilegios
    "FLUSH PRIVILEGES",
];

/// Ejecuta la configuración inicial de la base de datos:
/// 1. Conecta como root temporalmente
/// 2. Crea usuarios reci_read / reci_write con sus GRANTs
/// 3. Guarda config.json (sin contraseña de root)
/// 4. Inicializa los pools de lectura/escritura globales
pub async fn perform_setup_db(
    host: &str,
    port: u16,
    root_password: &str,
    config_dir: Option<&Path>,
    app_state: &AppState,
) -> Result<(), String> {
    // 1. Conectar como root de forma temporal
    let root_url = format!(
        "mysql://root:{}@{}:{}/?ssl-mode=disabled",
        root_password, host, port
    );

    let root_pool = MySqlPoolOptions::new()
        .max_connections(1)
        .connect(&root_url)
        .await
        .map_err(|e| {
            log::error!("Error conectando como root a MySQL: {:?}", e);
            format!(
                "No se pudo conectar al servidor MySQL en {}:{}. Verifique la dirección, el puerto y la contraseña de root. Detalle: {}",
                host, port, e
            )
        })?;

    // 2. Ejecutar sentencias de creación de usuarios y GRANTs
    for sql in SETUP_SQL {
        sqlx::query(sql)
            .execute(&root_pool)
            .await
            .map_err(|e| {
                log::error!("Error ejecutando SQL de setup '{}': {:?}", sql, e);
                format!(
                    "Error al configurar los usuarios de la base de datos: {}",
                    e
                )
            })?;
    }

    // Cerrar conexión temporal de root
    root_pool.close().await;

    // 3. Crear y guardar la configuración (NUNCA la contraseña de root)
    let app_config = AppConfig::new(host.to_string(), port);
    config::save_config(&app_config, config_dir)?;

    // 4. Inicializar los pools de la aplicación con los usuarios limitados
    let db = create_pools_from_config(&app_config).await.map_err(|e| {
        log::error!("Error creando pools después del setup: {:?}", e);
        format!(
            "Los usuarios se crearon correctamente pero no se pudo conectar con ellos: {}",
            e
        )
    })?;

    // 5. Actualizar el estado global de la aplicación
    {
        let mut db_guard = app_state.db.write().await;
        *db_guard = Some(db);
    }
    app_state
        .configured
        .store(true, std::sync::atomic::Ordering::SeqCst);

    Ok(())
}

// -------------------------------------------------------------------------
// Tauri Commands
// -------------------------------------------------------------------------

#[tauri::command]
pub async fn setup_db_connection(
    app_state: State<'_, AppState>,
    host: String,
    port: u16,
    root_password: String,
) -> Result<(), String> {
    perform_setup_db(&host, port, &root_password, None, &app_state).await
}

#[tauri::command]
pub async fn check_configured(
    app_state: State<'_, AppState>,
) -> Result<bool, String> {
    Ok(app_state
        .configured
        .load(std::sync::atomic::Ordering::SeqCst))
}

// -------------------------------------------------------------------------
// Tests
// -------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::AppState;
    use std::sync::atomic::AtomicBool;
    use tokio::sync::RwLock;

    #[tokio::test]
    async fn test_setup_db_connection() {
        // Este test requiere Docker MySQL 5.5 corriendo en puerto 3309
        // con la contraseña root = devrootpassword
        let app_state = AppState {
            db: RwLock::new(None),
            configured: AtomicBool::new(false),
        };

        let temp_dir = std::env::temp_dir().join("recicladora_setup_test");
        let _ = std::fs::remove_dir_all(&temp_dir);
        std::fs::create_dir_all(&temp_dir).unwrap();

        // Ejecutar setup
        let result = perform_setup_db(
            "127.0.0.1",
            3309,
            "devrootpassword",
            Some(&temp_dir),
            &app_state,
        )
        .await;

        assert!(
            result.is_ok(),
            "Setup debería ser exitoso: {:?}",
            result.err()
        );

        // Verificar que el estado cambió a configurado
        assert!(
            app_state
                .configured
                .load(std::sync::atomic::Ordering::SeqCst),
            "El estado debería ser configurado"
        );

        // Verificar que los pools están activos
        {
            let db_guard = app_state.db.read().await;
            let db = db_guard.as_ref().expect("Los pools deberían existir");

            // Test read pool
            let read_result: i32 = sqlx::query_scalar("SELECT 1")
                .fetch_one(&db.read_pool)
                .await
                .expect("Read pool debería funcionar");
            assert_eq!(read_result, 1);

            // Test write pool
            let write_result: i32 = sqlx::query_scalar("SELECT 1")
                .fetch_one(&db.write_pool)
                .await
                .expect("Write pool debería funcionar");
            assert_eq!(write_result, 1);
        }

        // Verificar que config.json se creó correctamente
        let loaded = config::load_config(Some(&temp_dir))
            .expect("config.json debería existir");
        assert_eq!(loaded.host, "127.0.0.1");
        assert_eq!(loaded.port, 3309);
        assert_eq!(loaded.read_user, "reci_read");
        assert_eq!(loaded.write_user, "reci_write");
        // La contraseña de root NO debe estar en el config
        let config_content =
            std::fs::read_to_string(temp_dir.join("config.json")).unwrap();
        assert!(
            !config_content.contains("devrootpassword"),
            "La contraseña de root NUNCA debe guardarse en config.json"
        );

        // Limpiar
        let _ = std::fs::remove_dir_all(&temp_dir);
    }
}
