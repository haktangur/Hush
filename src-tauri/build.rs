fn main() {
    tauri_build::try_build(tauri_build::Attributes::new().app_manifest(
        tauri_build::AppManifest::new().commands(&[
            "app_bootstrap",
            "library_save",
            "library_get",
            "library_list",
            "library_trash",
            "library_restore",
            "asset_choose_import",
            "asset_resolve",
            "settings_save",
            "room_slot_set",
        ]),
    ))
    .expect("Tauri build configuration failed");
}
