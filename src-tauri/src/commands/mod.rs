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

// The test selector only supplies a dialog result; the command and storage path remain real.
fn backup_path(app: &tauri::AppHandle, save: bool) -> Result<Option<std::path::PathBuf>> {
    #[cfg(feature = "e2e")]
    if let Some(root) = std::env::var_os("HUSH_TEST_DATA_DIR") {
        let selector = std::path::PathBuf::from(root).join(if save {
            "backup-save-selection"
        } else {
            "backup-open-selection"
        });
        if selector.exists() {
            let name = std::fs::read_to_string(&selector)?;
            if name.is_empty() {
                return Ok(None);
            }
            // Fixture names cannot escape the isolated test directory.
            if name.contains(['/', '\\']) || name == ".." {
                return Err(AppError::new("VALIDATION"));
            }
            return Ok(Some(selector.parent().unwrap().join(name)));
        }
    }
    use tauri_plugin_dialog::DialogExt;
    let window = app
        .get_webview_window("main")
        .ok_or_else(|| AppError::new("IO"))?;
    let dialog = app
        .dialog()
        .file()
        .set_parent(&window)
        .add_filter("Hush", &["hushbackup"]);
    let selected = if save {
        dialog.set_file_name("Hush.hushbackup").blocking_save_file()
    } else {
        dialog.blocking_pick_file()
    };
    selected
        .map(|p| p.into_path().map_err(|_| AppError::new("IO")))
        .transpose()
}

#[tauri::command]
pub async fn backup_export(
    app: tauri::AppHandle,
    request: EmptyRequest,
) -> ApiResult<Option<bool>> {
    let _ = request;
    let result = async {
        let a = app.clone();
        let selected = tauri::async_runtime::spawn_blocking(move || backup_path(&a, true))
            .await
            .map_err(|_| AppError::new("INTERNAL"))??;
        let Some(path) = selected else {
            return Ok(None);
        };
        let gate = app.state::<StorageGate>().0.clone().write_owned().await;
        tauri::async_runtime::spawn_blocking(move || {
            let _gate = gate;
            let s = storage(&app)?;
            crate::storage::backup::export(&s, &path)?;
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
        // A new selection (including cancellation/failure) revokes the previous preview.
        *app.state::<BackupState>()
            .0
            .lock()
            .map_err(|_| AppError::new("INTERNAL"))? = None;
        let Some(path) = backup_path(&app, false)? else {
            return Ok(None);
        };
        let root = s
            .generation
            .parent()
            .and_then(|p| p.parent())
            .ok_or_else(|| AppError::new("IO"))?;
        let (inspection, preview) = crate::storage::backup::inspect(root, &path)?;
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
        #[cfg(feature = "e2e")]
        app.asset_protocol_scope()
            .allow_directory(new.generation.join("assets"), true)
            .map_err(|_| AppError::new("IO"))?;
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

#[tauri::command]
pub async fn catalog_refresh_preview(
    app: tauri::AppHandle,
    request: IdRequest,
) -> ApiResult<crate::services::metadata::Diff> {
    let _gate = app.state::<StorageGate>().0.clone().read_owned().await;
    let result = async {
        let a = app.clone();
        let (s, item) = tauri::async_runtime::spawn_blocking(move || {
            let s = storage(&a)?;
            let item = crate::repositories::library::get(
                &*s.connection.lock().map_err(|_| AppError::new("INTERNAL"))?,
                &request.id,
                false,
            )?;
            Ok::<_, AppError>((s, item))
        })
        .await
        .map_err(|_| AppError::new("INTERNAL"))??;
        let p = item
            .item
            .provenance
            .as_ref()
            .ok_or(AppError::new("VALIDATION"))?;
        let request = crate::providers::Prepare {
            candidate_key: crate::providers::CandidateKey {
                provider: p.provider.clone(),
                entity_type: p.entity_type.clone(),
                external_id: p.external_id.clone(),
            },
            edition_id: p.edition_id.clone(),
        };
        let (c, contact) = crate::services::catalog::context(app.clone(), s.clone(), true).await?;
        let epoch = s.epoch.clone();
        let prepared = c
            .prepare_with_ttl(request, s, contact.as_deref(), 0)
            .await?;
        crate::services::metadata::preview(
            item,
            prepared,
            &app.state::<crate::services::metadata::RefreshState>(),
            epoch,
        )
    }
    .await;
    result.into()
}
#[tauri::command]
pub async fn catalog_refresh_apply(
    app: tauri::AppHandle,
    request: crate::services::metadata::Apply,
) -> ApiResult<LibraryItem> {
    work(app.clone(), move |s| {
        let state = app.state::<crate::services::metadata::RefreshState>();
        let mut pending = state.0.lock().map_err(|_| AppError::new("INTERNAL"))?;
        let p = pending
            .get(&request.token)
            .filter(|p| p.expires > std::time::Instant::now() && p.epoch == s.epoch)
            .ok_or(AppError::new("CONFLICT"))?;
        let mut c = s.connection.lock().map_err(|_| AppError::new("INTERNAL"))?;
        let current = crate::repositories::library::get(&c, &p.item.item.id, false)?;
        if current.revision != p.item.revision {
            return Err(AppError::new("CONFLICT"));
        }
        let draft = crate::services::metadata::merge(&current, &p.prepared, &request.fields)?;
        let result =
            crate::repositories::library::save(&mut c, &draft, Some(current.revision), None)?;
        pending.remove(&request.token);
        Ok(result)
    })
    .await
}

/// Set once the UI has approved quitting, so `RunEvent::ExitRequested` lets the exit through.
#[derive(Default)]
pub struct QuitApproved(pub std::sync::atomic::AtomicBool);

// Called only after the UI has no unsaved form work. Waits for any running storage
// operation (save, import, restore) to release the gate before quitting or hiding.
#[tauri::command]
pub async fn app_finish_close(
    app: tauri::AppHandle,
    quit: bool,
    gate: tauri::State<'_, StorageGate>,
) -> std::result::Result<(), String> {
    let _guard = gate.0.write().await;
    if quit {
        app.state::<QuitApproved>()
            .0
            .store(true, std::sync::atomic::Ordering::SeqCst);
        app.exit(0);
    } else if let Some(window) = app.get_webview_window("main") {
        window.hide().map_err(|_| "UNAVAILABLE".to_string())?;
    }
    Ok(())
}
