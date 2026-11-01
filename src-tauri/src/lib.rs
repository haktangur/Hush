mod commands;
mod domain;
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
            commands::room_slot_set
        ])
        .run(tauri::generate_context!())
        .expect("Hush could not start");
}
