pub mod commands;
pub mod db;
pub mod models;
pub mod utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Pre-initialize connection pools using block_on
    let db_state = tauri::async_runtime::block_on(async {
        db::create_pools()
            .await
            .expect("Failed to initialize database pools")
    });

    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .manage(db_state) // Manage AppDb state
        .invoke_handler(tauri::generate_handler![
            commands::test_connection,
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
