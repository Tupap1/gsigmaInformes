pub mod commands;
pub mod config;
pub mod db;
pub mod models;
pub mod utils;

use db::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Intentar cargar config.json y conectar automáticamente.
    // Si no existe, arrancar en "Modo Inicialización".
    let app_state = tauri::async_runtime::block_on(async {
        match config::load_config(None) {
            Some(cfg) => {
                // Config existe: intentar conectar con usuarios limitados
                match db::create_pools_from_config(&cfg).await {
                    Ok(db) => {
                        println!("✅ Configuración cargada. Pools inicializados.");
                        AppState::with_db(db)
                    }
                    Err(e) => {
                        eprintln!(
                            "⚠️ Config existe pero no se pudo conectar: {}. Entrando en Modo Inicialización.",
                            e
                        );
                        AppState::new()
                    }
                }
            }
            None => {
                println!("🔧 No se encontró config.json. Entrando en Modo Inicialización.");
                AppState::new()
            }
        }
    });

    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            commands::test_connection,
            commands::setup::setup_db_connection,
            commands::setup::check_configured,
            commands::proveedores::list_proveedores,
            commands::proveedores::get_proveedor,
            commands::proveedores::create_proveedor,
            commands::proveedores::update_proveedor,
            commands::proveedores::delete_proveedor,
            commands::informes::get_productos,
            commands::informes::get_compras_acumuladas,
            commands::informes::get_resumen_caja,
            commands::informes::save_pdf_file
        ])
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
