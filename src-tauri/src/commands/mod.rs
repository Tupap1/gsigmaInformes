use crate::db::{AppDb, AppState, get_db};
use serde::Serialize;
use tauri::State;

pub mod informes;
pub mod proveedores;
pub mod setup;

#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
pub struct ConnectionStatus {
    pub read: bool,
    pub write: bool,
}

pub async fn perform_test_connection(db: &AppDb) -> ConnectionStatus {
    let read_ok = sqlx::query_scalar("SELECT 1")
        .fetch_one(&db.read_pool)
        .await
        .map(|_: i32| true)
        .unwrap_or(false);

    let write_ok = sqlx::query_scalar("SELECT 1")
        .fetch_one(&db.write_pool)
        .await
        .map(|_: i32| true)
        .unwrap_or(false);

    ConnectionStatus {
        read: read_ok,
        write: write_ok,
    }
}

#[tauri::command]
pub async fn test_connection(app_state: State<'_, AppState>) -> Result<ConnectionStatus, String> {
    let db = get_db(&app_state).await?;
    Ok(perform_test_connection(&db).await)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;

    #[tokio::test]
    async fn test_perform_connection() {
        let db_state = db::create_pools().await.expect("Failed to create pools");
        let result = perform_test_connection(&db_state).await;
        assert!(result.read, "Read pool connection failed");
        assert!(result.write, "Write pool connection failed");
        println!("Test connection result: {:?}", result);
    }
}
