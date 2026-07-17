use crate::db::AppDb;
use serde::Serialize;
use tauri::State;

pub mod informes;
pub mod proveedores;

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
pub async fn test_connection(db: State<'_, AppDb>) -> Result<ConnectionStatus, String> {
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
