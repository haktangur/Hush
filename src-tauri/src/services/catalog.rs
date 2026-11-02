use crate::{domain::*, providers::*, storage::Storage};
use std::sync::{Arc, Mutex};
use tauri::Manager;
pub struct CatalogState(pub Mutex<Option<Arc<Catalog>>>);
pub async fn context(
    app: tauri::AppHandle,
    s: Arc<Storage>,
    require_enabled: bool,
) -> Result<(Arc<Catalog>, Option<String>)> {
    tauri::async_runtime::spawn_blocking(move || {
        let state = app.state::<CatalogState>();
        let mut state = state.0.lock().map_err(|_| AppError::new("INTERNAL"))?;
        let catalog = match state.as_ref() {
            Some(c) => c.clone(),
            None => {
                let c = Arc::new(Catalog::new(
                    app.path()
                        .app_cache_dir()
                        .map_err(|_| AppError::new("IO"))?
                        .join("catalog"),
                )?);
                *state = Some(c.clone());
                c
            }
        };
        let c = s.connection.lock().map_err(|_| AppError::new("INTERNAL"))?;
        let settings = super::settings::get(&c)?;
        if require_enabled && !settings.value.catalog.enabled {
            return Err(AppError::new("PROVIDER_NOT_CONFIGURED"));
        }
        Ok((catalog, settings.value.catalog.contact))
    })
    .await
    .map_err(|_| AppError::new("INTERNAL"))?
}
pub async fn annotate(s: Arc<Storage>, mut page: CatalogPage) -> Result<CatalogPage> {
    tauri::async_runtime::spawn_blocking(move||{use rusqlite::OptionalExtension;let c=s.connection.lock().map_err(|_|AppError::new("INTERNAL"))?;for item in &mut page.items {let found:Option<(String,Option<String>)>=c.query_row("SELECT i.id,i.deleted_at FROM library_items i JOIN catalog_refs r ON r.item_id=i.id WHERE r.provider=? AND r.entity_type=? AND r.external_id=?",rusqlite::params![item.key.provider,item.key.entity_type,item.key.external_id],|r|Ok((r.get(0)?,r.get(1)?))).optional()?;if let Some((id,deleted))=found{item.existing_item_id=Some(id);item.existing_state=Some(if deleted.is_some(){"trashed"}else{"active"}.into())}}Ok(page)}).await.map_err(|_|AppError::new("INTERNAL"))?
}
