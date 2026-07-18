use crate::config::AppConfig;
use sqlx::{mysql::MySqlPoolOptions, MySqlPool};
use std::env;
use std::sync::atomic::AtomicBool;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct AppDb {
    pub read_pool: MySqlPool,
    pub write_pool: MySqlPool,
}

/// Estado global de la aplicación que soporta "Modo Inicialización".
/// Al iniciar sin config.json, `db` es None y `configured` es false.
/// Tras ejecutar setup_db_connection, se llenan los pools y el flag cambia.
pub struct AppState {
    pub db: RwLock<Option<AppDb>>,
    pub configured: AtomicBool,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            db: RwLock::new(None),
            configured: AtomicBool::new(false),
        }
    }

    pub fn with_db(db: AppDb) -> Self {
        Self {
            db: RwLock::new(Some(db)),
            configured: AtomicBool::new(true),
        }
    }
}

/// Extrae el AppDb del AppState, retornando un error claro si no está configurado.
pub async fn get_db(state: &AppState) -> Result<AppDb, String> {
    state
        .db
        .read()
        .await
        .clone()
        .ok_or_else(|| {
            "La aplicación no está configurada. Complete el asistente de configuración inicial."
                .to_string()
        })
}

/// Crea los pools de conexión a partir de un AppConfig (post-setup).
pub async fn create_pools_from_config(config: &AppConfig) -> Result<AppDb, sqlx::Error> {
    let read_url = format!(
        "mysql://{}:{}@{}:{}/?ssl-mode=disabled",
        config.read_user, config.read_password, config.host, config.port
    );
    let write_url = format!(
        "mysql://{}:{}@{}:{}/?ssl-mode=disabled",
        config.write_user, config.write_password, config.host, config.port
    );

    let read_pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&read_url)
        .await?;

    let write_pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&write_url)
        .await?;

    Ok(AppDb {
        read_pool,
        write_pool,
    })
}

pub fn load_env() {
    let prefer_test = cfg!(test) || env::var("NODE_ENV").map(|v| v == "test").unwrap_or(false);
    let primary_file = if prefer_test { ".env.test" } else { ".env" };
    let secondary_file = if prefer_test { ".env" } else { ".env.test" };

    if let Ok(current_dir) = env::current_dir() {
        let mut dir = current_dir;
        loop {
            let primary_path = dir.join(primary_file);
            if primary_path.exists() {
                dotenvy::from_path(&primary_path).ok();
                println!("Loaded env from: {:?}", primary_path);
                return;
            }
            let secondary_path = dir.join(secondary_file);
            if secondary_path.exists() {
                dotenvy::from_path(&secondary_path).ok();
                println!("Loaded env from: {:?}", secondary_path);
                return;
            }
            if !dir.pop() {
                break;
            }
        }
    }
    // Fallback
    let _ = dotenvy::dotenv();
}

pub async fn create_pools() -> Result<AppDb, sqlx::Error> {
    load_env();

    let host = env::var("DB_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("DB_PORT").unwrap_or_else(|_| "3306".to_string());

    let read_user = env::var("DB_READ_USER").expect("DB_READ_USER must be set");
    let read_password = env::var("DB_READ_PASSWORD").expect("DB_READ_PASSWORD must be set");

    let write_user = env::var("DB_WRITE_USER").expect("DB_WRITE_USER must be set");
    let write_password = env::var("DB_WRITE_PASSWORD").expect("DB_WRITE_PASSWORD must be set");

    // Construir URLs de conexión desactivando SSL para compatibilidad con MySQL 5.5 local
    let read_url = format!(
        "mysql://{}:{}@{}:{}/?ssl-mode=disabled",
        read_user, read_password, host, port
    );
    let write_url = format!(
        "mysql://{}:{}@{}:{}/?ssl-mode=disabled",
        write_user, write_password, host, port
    );

    println!(
        "Connecting to Read Pool using URL: mysql://{}@{}:{}/",
        read_user, host, port
    );
    let read_pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&read_url)
        .await?;

    println!(
        "Connecting to Write Pool using URL: mysql://{}@{}:{}/",
        write_user, host, port
    );
    let write_pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&write_url)
        .await?;

    Ok(AppDb {
        read_pool,
        write_pool,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_pools() {
        let db = create_pools().await.expect("Failed to create pools");

        // Test read pool with SELECT 1
        let read_row: i32 = sqlx::query_scalar("SELECT 1")
            .fetch_one(&db.read_pool)
            .await
            .expect("Failed to query read pool");
        assert_eq!(read_row, 1);
        println!("Read pool query successful: {}", read_row);

        // Test write pool with SELECT 1
        let write_row: i32 = sqlx::query_scalar("SELECT 1")
            .fetch_one(&db.write_pool)
            .await
            .expect("Failed to query write pool");
        assert_eq!(write_row, 1);
        println!("Write pool query successful: {}", write_row);
    }
}
