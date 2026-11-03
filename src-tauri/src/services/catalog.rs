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
                let cache = app
                    .path()
                    .app_cache_dir()
                    .map_err(|_| AppError::new("IO"))?
                    .join("catalog");
                #[cfg(feature = "e2e")]
                let cache = std::env::var_os("HUSH_TEST_DATA_DIR")
                    .map(std::path::PathBuf::from)
                    .map(|p| p.join("catalog"))
                    .unwrap_or(cache);
                let c = Arc::new(Catalog::new(cache)?);
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
    tauri::async_runtime::spawn_blocking(move || {
        let c = s.connection.lock().map_err(|_| AppError::new("INTERNAL"))?;
        for item in &mut page.items {
            if let Some((id, trashed)) = crate::repositories::library::catalog_existing(
                &c,
                &item.key.provider,
                &item.key.entity_type,
                &item.key.external_id,
            )? {
                item.existing_item_id = Some(id);
                item.existing_state = Some(if trashed { "trashed" } else { "active" }.into());
            }
        }
        Ok(page)
    })
    .await
    .map_err(|_| AppError::new("INTERNAL"))?
}
