mod commands;
mod domain;
mod providers;
mod repositories;
mod services;
mod storage;
use tauri::Manager;
pub fn run() {
    let builder = tauri::Builder::default();
    #[cfg(feature = "e2e")]
    let builder = builder
        .plugin(tauri_plugin_wdio::init())
        .plugin(tauri_plugin_wdio_webdriver::init());
    builder
        .plugin(tauri_plugin_dialog::init())
        .manage(services::catalog::CatalogState(std::sync::Mutex::new(None)))
        .manage(commands::StorageGate::default())
        .manage(commands::BackupState::default())
        .manage(commands::AppState(std::sync::Mutex::new(None)))
        .plugin(tauri_plugin_single_instance::init(|app, _, _| {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
                let _ = window.set_focus();
            }
        }))
        .invoke_handler(tauri::generate_handler![
            commands::app_bootstrap,
            commands::library_save,
            commands::library_get,
            commands::library_list,
            commands::library_trash,
            commands::library_restore,
            commands::asset_choose_import,
            commands::asset_resolve,
            commands::settings_save,
            commands::room_slot_set,
            commands::library_extras,
            commands::session_save,
            commands::session_delete,
            commands::library_children_replace,
            commands::collection_list,
            commands::collection_save,
            commands::collection_delete,
            commands::trash_purge,
            commands::journal_list,
            commands::journal_save,
            commands::draft_get,
            commands::draft_put,
            commands::draft_delete,
            commands::journal_trash,
            commands::journal_restore,
            commands::calendar_month,
            commands::search_local,
            commands::catalog_search,
            commands::catalog_prepare,
            commands::catalog_editions,
            commands::provider_status,
            commands::provider_igdb_configure,
            commands::provider_disconnect,
            commands::backup_export,
            commands::backup_inspect,
            commands::backup_restore
        ])
        .run(tauri::generate_context!())
        .expect("Hush could not start");
}
