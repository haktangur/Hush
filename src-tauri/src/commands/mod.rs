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
#[derive(Default)]
pub struct StorageGate(pub Arc<tokio::sync::RwLock<()>>);
#[derive(Default)]
pub struct BackupState(pub Mutex<Option<crate::storage::backup::Inspection>>);
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
    // Test data lives outside APPDATA. Only this generation's images are exposed;
    // release builds keep the static APPDATA asset scope unchanged.
    #[cfg(feature = "e2e")]
    app.asset_protocol_scope()
        .allow_directory(s.generation.join("assets"), true)
        .map_err(|_| AppError::new("IO"))?;
    *guard = Some(s.clone());
    Ok(s)
}
async fn work<T: Send + 'static>(
    app: tauri::AppHandle,
    f: impl FnOnce(Arc<Storage>) -> Result<T> + Send + 'static,
) -> ApiResult<T> {
    let gate = app.state::<StorageGate>().0.clone().read_owned().await;
    tauri::async_runtime::spawn_blocking(move || {
        let _gate = gate;
        f(storage(&app)?)
    })
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
        let window = app
            .get_webview_window("main")
            .ok_or_else(|| AppError::new("UNAVAILABLE"))?;
        window.show().map_err(|_| AppError::new("UNAVAILABLE"))?;
        window
            .set_focus()
            .map_err(|_| AppError::new("UNAVAILABLE"))?;
        let path = app
            .dialog()
            .file()
            .set_parent(&window)
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
#[tauri::command]
pub async fn library_extras(
    app: tauri::AppHandle,
    request: IdRequest,
) -> ApiResult<crate::domain::activity::Extras> {
    work(app, move |s| LibraryService(s).extras(&request.id)).await
}
#[tauri::command]
pub async fn session_save(
    app: tauri::AppHandle,
    request: crate::domain::activity::SaveSession,
) -> ApiResult<crate::domain::activity::Session> {
    work(app, move |s| LibraryService(s).save_session(request)).await
}
#[tauri::command]
pub async fn session_delete(app: tauri::AppHandle, request: RevisionRequest) -> ApiResult<()> {
    work(app, move |s| {
        LibraryService(s).delete_session(&request.id, request.expected_revision)
    })
    .await
}
#[tauri::command]
pub async fn library_children_replace(
    app: tauri::AppHandle,
    request: crate::domain::activity::ReplaceChildren,
) -> ApiResult<LibraryItem> {
    work(app, move |s| LibraryService(s).replace_children(request)).await
}
#[tauri::command]
pub async fn collection_list(
    app: tauri::AppHandle,
    request: EmptyRequest,
) -> ApiResult<Vec<crate::domain::collection::Collection>> {
    let _ = request;
    work(app, move |s| LibraryService(s).collections()).await
}
#[tauri::command]
pub async fn collection_save(
    app: tauri::AppHandle,
    request: crate::domain::collection::SaveCollection,
) -> ApiResult<crate::domain::collection::Collection> {
    work(app, move |s| LibraryService(s).save_collection(request)).await
}
#[tauri::command]
pub async fn collection_delete(app: tauri::AppHandle, request: RevisionRequest) -> ApiResult<()> {
    work(app, move |s| {
        LibraryService(s).delete_collection(&request.id, request.expected_revision)
    })
    .await
}
#[tauri::command]
pub async fn trash_purge(app: tauri::AppHandle, request: IdRequest) -> ApiResult<()> {
    work(app, move |s| LibraryService(s).purge(&request.id)).await
}
#[tauri::command]
pub async fn journal_list(
    app: tauri::AppHandle,
    request: crate::domain::journal::JournalFilter,
) -> ApiResult<Page<crate::domain::journal::JournalEntry>> {
    work(app, move |s| {
        crate::services::journal::JournalService(s).list(request)
    })
    .await
}
#[tauri::command]
pub async fn journal_save(
    app: tauri::AppHandle,
    request: crate::domain::journal::SaveJournal,
) -> ApiResult<crate::domain::journal::JournalEntry> {
    work(app, move |s| {
        crate::services::journal::JournalService(s).save(request)
    })
    .await
}
#[tauri::command]
pub async fn draft_get(
    app: tauri::AppHandle,
    request: EmptyRequest,
) -> ApiResult<Option<crate::domain::journal::SaveJournal>> {
    let _ = request;
    work(app, move |s| {
        crate::services::journal::JournalService(s).draft_get()
    })
    .await
}
#[tauri::command]
pub async fn draft_put(
    app: tauri::AppHandle,
    request: crate::domain::journal::SaveJournal,
) -> ApiResult<()> {
    work(app, move |s| {
        crate::services::journal::JournalService(s).draft_put(request)
    })
    .await
}
#[tauri::command]
pub async fn draft_delete(app: tauri::AppHandle, request: EmptyRequest) -> ApiResult<()> {
    let _ = request;
    work(app, move |s| {
        crate::services::journal::JournalService(s).draft_delete()
    })
    .await
}
#[tauri::command]
pub async fn journal_trash(app: tauri::AppHandle, request: RevisionRequest) -> ApiResult<()> {
    work(app, move |s| {
        crate::services::journal::JournalService(s).trash(
            &request.id,
            request.expected_revision,
            false,
        )
    })
    .await
}
#[tauri::command]
pub async fn journal_restore(app: tauri::AppHandle, request: RevisionRequest) -> ApiResult<()> {
    work(app, move |s| {
        crate::services::journal::JournalService(s).trash(
            &request.id,
            request.expected_revision,
            true,
        )
    })
    .await
}
#[tauri::command]
pub async fn calendar_month(
    app: tauri::AppHandle,
    request: crate::domain::journal::CalendarRequest,
) -> ApiResult<crate::domain::journal::CalendarMonth> {
    work(app, move |s| {
        crate::services::journal::JournalService(s).calendar(request)
    })
    .await
}
#[tauri::command]
pub async fn search_local(
    app: tauri::AppHandle,
    request: crate::services::journal::SearchRequest,
) -> ApiResult<crate::services::journal::SearchResults> {
    work(app, move |s| {
        crate::services::journal::JournalService(s).search(request)
    })
    .await
}
#[tauri::command]
pub async fn catalog_search(
    app: tauri::AppHandle,
    request: crate::providers::Search,
) -> ApiResult<crate::providers::CatalogPage> {
    let _gate = app.state::<StorageGate>().0.clone().read_owned().await;
    let result = async {
        let a = app.clone();
        let s = tauri::async_runtime::spawn_blocking(move || storage(&a))
            .await
            .map_err(|_| AppError::new("INTERNAL"))??;
        let (c, contact) = crate::services::catalog::context(app, s.clone(), true).await?;
        crate::services::catalog::annotate(s, c.search(request, contact.as_deref()).await?).await
    }
    .await;
    result.into()
}
#[tauri::command]
pub async fn catalog_prepare(
    app: tauri::AppHandle,
    request: crate::providers::Prepare,
) -> ApiResult<crate::providers::Prepared> {
    let _gate = app.state::<StorageGate>().0.clone().read_owned().await;
    let result = async {
        let a = app.clone();
        let s = tauri::async_runtime::spawn_blocking(move || storage(&a))
            .await
            .map_err(|_| AppError::new("INTERNAL"))??;
        let (c, contact) = crate::services::catalog::context(app, s.clone(), true).await?;
        c.prepare(request, s, contact.as_deref()).await
    }
    .await;
    result.into()
}
#[tauri::command]
pub async fn catalog_editions(
    app: tauri::AppHandle,
    request: crate::providers::CandidateKey,
) -> ApiResult<Vec<crate::providers::Edition>> {
    let _gate = app.state::<StorageGate>().0.clone().read_owned().await;
    let result = async {
        let a = app.clone();
        let s = tauri::async_runtime::spawn_blocking(move || storage(&a))
            .await
            .map_err(|_| AppError::new("INTERNAL"))??;
        let (c, contact) = crate::services::catalog::context(app, s, true).await?;
        c.editions(request, contact.as_deref()).await
    }
    .await;
    result.into()
}
#[tauri::command]
pub async fn provider_status(request: EmptyRequest) -> ApiResult<crate::providers::ProviderState> {
    let _ = request;
    tauri::async_runtime::spawn_blocking(|| Ok(crate::providers::Catalog::status()))
        .await
        .unwrap_or_else(|_| Err(AppError::new("INTERNAL")))
        .into()
}
#[tauri::command]
pub async fn provider_igdb_configure(
    app: tauri::AppHandle,
    request: crate::providers::Credentials,
) -> ApiResult<crate::providers::ProviderState> {
    let _gate = app.state::<StorageGate>().0.clone().read_owned().await;
    let result = async {
        let a = app.clone();
        let s = tauri::async_runtime::spawn_blocking(move || storage(&a))
            .await
            .map_err(|_| AppError::new("INTERNAL"))??;
        let (c, _) = crate::services::catalog::context(app, s, false).await?;
        c.configure(request).await
    }
    .await;
    result.into()
}
#[tauri::command]
pub async fn provider_disconnect(app: tauri::AppHandle, request: EmptyRequest) -> ApiResult<()> {
    let _ = request;
    let _gate = app.state::<StorageGate>().0.clone().read_owned().await;
    let result = async {
        let a = app.clone();
        let s = tauri::async_runtime::spawn_blocking(move || storage(&a))
            .await
            .map_err(|_| AppError::new("INTERNAL"))??;
        let (c, _) = crate::services::catalog::context(app, s, false).await?;
        c.disconnect().await
    }
    .await;
    result.into()
}

#[tauri::command]
pub async fn backup_export(
    app: tauri::AppHandle,
    request: EmptyRequest,
) -> ApiResult<Option<bool>> {
    let _ = request;
    let result = async {
        use tauri_plugin_dialog::DialogExt;
        let a = app.clone();
        let selected = tauri::async_runtime::spawn_blocking(move || {
            a.dialog()
                .file()
                .add_filter("Hush", &["hushbackup"])
                .set_file_name("Hush.hushbackup")
                .blocking_save_file()
        })
        .await
        .map_err(|_| AppError::new("INTERNAL"))?;
        let Some(path) = selected else {
            return Ok(None);
        };
        let gate = app.state::<StorageGate>().0.clone().write_owned().await;
        tauri::async_runtime::spawn_blocking(move || {
            let _gate = gate;
            let s = storage(&app)?;
            crate::storage::backup::export(
                &s,
                &path.into_path().map_err(|_| AppError::new("IO"))?,
            )?;
            Ok(Some(true))
        })
        .await
        .map_err(|_| AppError::new("INTERNAL"))?
    }
    .await;
    result.into()
}
#[tauri::command]
pub async fn backup_inspect(
    app: tauri::AppHandle,
    request: EmptyRequest,
) -> ApiResult<Option<crate::storage::backup::Preview>> {
    let _ = request;
    work(app.clone(), move |s| {
        use tauri_plugin_dialog::DialogExt;
        let Some(path) = app
            .dialog()
            .file()
            .add_filter("Hush", &["hushbackup"])
            .blocking_pick_file()
        else {
            return Ok(None);
        };
        let root = s
            .generation
            .parent()
            .and_then(|p| p.parent())
            .ok_or_else(|| AppError::new("IO"))?;
        let (inspection, preview) = crate::storage::backup::inspect(
            root,
            &path.into_path().map_err(|_| AppError::new("IO"))?,
        )?;
        *app.state::<BackupState>()
            .0
            .lock()
            .map_err(|_| AppError::new("INTERNAL"))? = Some(inspection);
        Ok(Some(preview))
    })
    .await
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RestoreRequest {
    inspection_token: String,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RestoreResult {
    storage_epoch: String,
    restored_at: String,
}
#[tauri::command]
pub async fn backup_restore(
    app: tauri::AppHandle,
    request: RestoreRequest,
) -> ApiResult<RestoreResult> {
    let gate = app.state::<StorageGate>().0.clone().write_owned().await;
    tauri::async_runtime::spawn_blocking(move || {
        let _gate = gate;
        let old = storage(&app)?;
        let state = app.state::<BackupState>();
        let mut pending = state.0.lock().map_err(|_| AppError::new("INTERNAL"))?;
        if pending
            .as_ref()
            .is_none_or(|i| i.token != request.inspection_token)
        {
            return Err(AppError::new("EXPIRED_BACKUP"));
        }
        let inspection = pending
            .take()
            .ok_or_else(|| AppError::new("EXPIRED_BACKUP"))?;
        let root = old
            .generation
            .parent()
            .and_then(|p| p.parent())
            .ok_or_else(|| AppError::new("IO"))?;
        let new = crate::storage::backup::restore(root, &old, inspection)?;
        let result = RestoreResult {
            storage_epoch: new.epoch.clone(),
            restored_at: crate::domain::now(),
        };
        *app.state::<AppState>()
            .0
            .lock()
            .map_err(|_| AppError::new("INTERNAL"))? = Some(Arc::new(new));
        Ok(result)
    })
    .await
    .unwrap_or_else(|_| Err(AppError::new("INTERNAL")))
    .into()
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExternalRequest {
    pub url: String,
}
#[tauri::command]
pub async fn external_open(request: ExternalRequest) -> ApiResult<()> {
    tauri::async_runtime::spawn_blocking(move || crate::services::external::open(&request.url))
        .await
        .unwrap_or_else(|_| Err(AppError::new("INTERNAL")))
        .into()
}
