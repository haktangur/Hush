use crate::{
    domain::{AppError, LibraryItem, Result, SaveItemRequest},
    repositories::library::{LibraryFilter, Page},
    services::LibraryService,
    storage::Storage,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tauri::Manager;
pub struct AppState(pub Mutex<Option<Arc<Storage>>>);
#[derive(Deserialize)]
pub struct EmptyRequest {}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct IdRequest {
    pub id: String,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RevisionRequest {
    pub id: String,
    pub expected_revision: i64,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Bootstrap {
    version: &'static str,
    ready: bool,
    storage_epoch: String,
    settings: crate::services::settings::Snapshot,
    slots: crate::services::settings::Slots,
}
#[derive(Serialize)]
#[serde(untagged)]
pub enum ApiResult<T> {
    Success { ok: bool, value: T },
    Failure { ok: bool, error: AppError },
}
impl<T> From<Result<T>> for ApiResult<T> {
    fn from(value: Result<T>) -> Self {
        match value {
            Ok(value) => Self::Success { ok: true, value },
            Err(error) => Self::Failure { ok: false, error },
        }
    }
}
fn storage(app: &tauri::AppHandle) -> Result<Arc<Storage>> {
    let state = app.state::<AppState>();
    let mut guard = state.0.lock().map_err(|_| AppError::new("INTERNAL"))?;
    if let Some(s) = guard.as_ref() {
        return Ok(s.clone());
    }
    let path = app.path().app_data_dir().map_err(|_| AppError::new("IO"))?;
    #[cfg(feature = "e2e")]
    let path = std::env::var_os("HUSH_TEST_DATA_DIR")
        .map(std::path::PathBuf::from)
        .unwrap_or(path);
    let s = Arc::new(Storage::open(&path)?);
    *guard = Some(s.clone());
    Ok(s)
}
async fn work<T: Send + 'static>(
    app: tauri::AppHandle,
    f: impl FnOnce(Arc<Storage>) -> Result<T> + Send + 'static,
) -> ApiResult<T> {
    tauri::async_runtime::spawn_blocking(move || f(storage(&app)?))
        .await
        .unwrap_or_else(|_| Err(AppError::new("INTERNAL")))
        .into()
}
#[tauri::command]
pub async fn app_bootstrap(app: tauri::AppHandle, request: EmptyRequest) -> ApiResult<Bootstrap> {
    let _ = request;
    work(app, move |s| {
        let c = s.connection.lock().map_err(|_| AppError::new("INTERNAL"))?;
        Ok(Bootstrap {
            version: env!("CARGO_PKG_VERSION"),
            ready: true,
            storage_epoch: s.epoch.clone(),
            settings: crate::services::settings::get(&c)?,
            slots: crate::services::settings::slots(&c)?,
        })
    })
    .await
}
#[tauri::command]
pub async fn library_save(
    app: tauri::AppHandle,
    request: SaveItemRequest,
) -> ApiResult<LibraryItem> {
    work(app, move |s| LibraryService(s).save(request)).await
}
#[tauri::command]
pub async fn library_get(app: tauri::AppHandle, request: IdRequest) -> ApiResult<LibraryItem> {
    work(app, move |s| LibraryService(s).get(&request.id)).await
}
#[tauri::command]
pub async fn library_list(
    app: tauri::AppHandle,
    request: LibraryFilter,
) -> ApiResult<Page<LibraryItem>> {
    work(app, move |s| LibraryService(s).list(request)).await
}
#[tauri::command]
pub async fn library_trash(app: tauri::AppHandle, request: RevisionRequest) -> ApiResult<()> {
    work(app, move |s| {
        LibraryService(s).trash(&request.id, request.expected_revision, false)
    })
    .await
}
#[tauri::command]
pub async fn library_restore(app: tauri::AppHandle, request: RevisionRequest) -> ApiResult<()> {
    work(app, move |s| {
        LibraryService(s).trash(&request.id, request.expected_revision, true)
    })
    .await
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ImportRequest {
    pub purpose: String,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ResolveRequest {
    pub asset_ids: Vec<String>,
}
#[tauri::command]
pub async fn asset_choose_import(
    app: tauri::AppHandle,
    request: ImportRequest,
) -> ApiResult<Option<crate::storage::assets::Asset>> {
    work(app.clone(), move |s| {
        use tauri_plugin_dialog::DialogExt;
        let path = app
            .dialog()
            .file()
            .add_filter("PNG / JPEG / WebP", &["png", "jpg", "jpeg", "webp"])
            .blocking_pick_file();
        match path {
            None => Ok(None),
            Some(path) => {
                let path = path.into_path().map_err(|_| AppError::new("VALIDATION"))?;
                crate::storage::assets::import(&s, &path, &request.purpose).map(Some)
            }
        }
    })
    .await
}
#[tauri::command]
pub async fn asset_resolve(
    app: tauri::AppHandle,
    request: ResolveRequest,
) -> ApiResult<Vec<crate::storage::assets::ResolvedAsset>> {
    work(app, move |s| {
        crate::storage::assets::resolve(&s, &request.asset_ids)
    })
    .await
}

#[tauri::command]
pub async fn settings_save(
    app: tauri::AppHandle,
    request: crate::services::settings::SaveSettings,
) -> ApiResult<crate::services::settings::Snapshot> {
    work(app, move |s| {
        let c = s.connection.lock().map_err(|_| AppError::new("INTERNAL"))?;
        crate::services::settings::save(&c, request)
    })
    .await
}
#[tauri::command]
pub async fn room_slot_set(
    app: tauri::AppHandle,
    request: crate::services::settings::SetSlot,
) -> ApiResult<crate::services::settings::Slots> {
    work(app, move |s| {
        let c = s.connection.lock().map_err(|_| AppError::new("INTERNAL"))?;
        crate::services::settings::set_slot(&c, request)
    })
    .await
}
