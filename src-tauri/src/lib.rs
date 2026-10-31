mod commands;
use tauri::Manager;
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _, _| {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
                let _ = window.set_focus();
            }
        }))
        .invoke_handler(tauri::generate_handler![commands::app_bootstrap])
        .run(tauri::generate_context!())
        .expect("Hush could not start");
}
