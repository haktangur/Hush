use crate::{domain::*, providers::Prepared};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    collections::HashMap,
    sync::Mutex,
    time::{Duration, Instant},
};
#[derive(Default)]
pub struct RefreshState(pub Mutex<HashMap<String, Pending>>);
pub struct Pending {
    pub epoch: String,
    pub item: LibraryItem,
    pub prepared: Prepared,
    pub expires: Instant,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Diff {
    pub token: String,
    pub fields: Vec<Field>,
    pub warnings: Vec<String>,
}
#[derive(Serialize)]
pub struct Field {
    pub field: String,
    pub before: Value,
    pub after: Value,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Apply {
    pub token: String,
    pub fields: Vec<String>,
}
fn paths(kind: &str) -> Vec<&'static str> {
    let mut fields = vec!["title", "creator", "releaseYear", "coverAssetId"];
    match kind {
        "book" => fields.extend(["isbn", "publisher", "language", "totalPages"]),
        "game" => fields.extend(["platform", "developer"]),
        _ => (),
    }
    fields
}
fn value(item: &Value, field: &str) -> Value {
    item.get(field)
        .or_else(|| item["details"].get(field))
        .cloned()
        .unwrap_or(Value::Null)
}
pub fn preview(
    item: LibraryItem,
    prepared: Prepared,
    state: &RefreshState,
    epoch: String,
) -> Result<Diff> {
    let a = serde_json::to_value(&item.item).map_err(|_| AppError::new("INTERNAL"))?;
    let b = serde_json::to_value(&prepared.item).map_err(|_| AppError::new("INTERNAL"))?;
    let fields = paths(item.item.kind())
        .into_iter()
        .filter_map(|field| {
            let before = value(&a, field);
            let after = value(&b, field);
            if before == after || (field == "coverAssetId" && after.is_null()) {
                None
            } else {
                Some(Field {
                    field: field.into(),
                    before,
                    after,
                })
            }
        })
        .collect();
    let token = uuid::Uuid::new_v4().to_string();
    let warnings = prepared.warnings.clone();
    let mut pending = state.0.lock().map_err(|_| AppError::new("INTERNAL"))?;
    pending.retain(|_, p| p.expires > Instant::now());
    if pending.len() >= 20 {
        return Err(AppError::new("RATE_LIMITED"));
    }
    pending.insert(
        token.clone(),
        Pending {
            epoch,
            item,
            prepared,
            expires: Instant::now() + Duration::from_secs(600),
        },
    );
    Ok(Diff {
        token,
        fields,
        warnings,
    })
}
pub fn merge(current: &LibraryItem, prepared: &Prepared, fields: &[String]) -> Result<ItemDraft> {
    let allowed = paths(current.item.kind());
    if fields.is_empty() || fields.iter().any(|f| !allowed.contains(&f.as_str())) {
        return Err(AppError::new("VALIDATION"));
    }
    let mut target = serde_json::to_value(&current.item).map_err(|_| AppError::new("INTERNAL"))?;
    let source = serde_json::to_value(&prepared.item).map_err(|_| AppError::new("INTERNAL"))?;
    for field in fields {
        let new = value(&source, field);
        if field == "coverAssetId" && new.is_null() {
            continue;
        }
        if target.get(field).is_some() {
            target[field] = new;
        } else {
            target["details"][field] = new;
        }
    }
    let mut draft: ItemDraft =
        serde_json::from_value(target).map_err(|_| AppError::new("VALIDATION"))?;
    // Validation of a completed book normally advances its page. Refresh must never do so.
    let progress = match &draft.content {
        ItemContent::Book { details, .. } => Some(details.current_page),
        _ => None,
    };
    draft.validate()?;
    if let (Some(page), ItemContent::Book { details, .. }) = (progress, &mut draft.content) {
        details.current_page = page;
    }
    Ok(draft)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn refresh_preserves_personal_fields_and_rejects_progress_overflow() {
        let item: ItemDraft =
            serde_json::from_str(include_str!("../../../tests/fixtures/book.json")).unwrap();
        let local = LibraryItem {
            item: item.clone(),
            revision: 1,
            created_at: now(),
            updated_at: now(),
            deleted_at: None,
        };
        let mut incoming = item.clone();
        incoming.title = "Yeni katalog başlığı".into();
        incoming.notes = "Asla uygulanmaz".into();
        incoming.rating = Some(9);
        if let ItemContent::Book { details, .. } = &mut incoming.content {
            details.total_pages = Some(400);
            details.current_page = 0;
        }
        let prepared = Prepared {
            existing_item_id: None,
            existing_state: None,
            item: incoming,
            tracks: vec![],
            cover_lease_id: None,
            warnings: vec![],
        };
        let merged = merge(&local, &prepared, &["title".into(), "totalPages".into()]).unwrap();
        assert_eq!(merged.notes, item.notes);
        assert_eq!(merged.rating, item.rating);
        assert_eq!(merged.tag_names, item.tag_names);
        assert_eq!(merged.status(), item.status());
        if let ItemContent::Book { details, .. } = &merged.content {
            assert_eq!(details.current_page, 37);
            assert_eq!(details.total_pages, Some(400));
        } else {
            panic!()
        }
        assert!(merge(&local, &prepared, &["notes".into()]).is_err());
        let mut prepared = prepared;
        if let ItemContent::Book { details, .. } = &mut prepared.item.content {
            details.total_pages = Some(20);
        }
        assert!(merge(&local, &prepared, &["totalPages".into()]).is_err());
    }
}

#[cfg(test)]
mod persistence_tests {
    use super::*;
    use crate::{
        domain::activity::*,
        repositories::{activity, library},
        storage::Storage,
    };
    #[test]
    fn all_kinds_refresh_preserves_sessions_children_and_personal_data() {
        for kind in ["book", "album", "game"] {
            let root = tempfile::tempdir().unwrap();
            let storage = Storage::open(root.path()).unwrap();
            let mut c = storage.connection.lock().unwrap();
            let mut draft: ItemDraft =
                serde_json::from_str(include_str!("../../../tests/fixtures/book.json")).unwrap();
            if kind == "album" {
                draft.content = ItemContent::Album {
                    status: AlbumStatus::Active,
                    details: AlbumDetails {
                        first_listened_on: Some("2026-01-01".into()),
                        spotify_url: None,
                        apple_music_url: None,
                    },
                };
            }
            if kind == "game" {
                draft.content = ItemContent::Game {
                    status: Status::Paused,
                    details: GameDetails {
                        platform: "Mac".into(),
                        developer: "Sentetik".into(),
                        progress_percent: Some(42),
                        return_note: "Kişisel dönüş notu".into(),
                    },
                };
            }
            let mut local = library::save(&mut c, &draft, None, None).unwrap();
            activity::save_session(
                &mut c,
                SaveSession {
                    draft: SessionDraft {
                        id: uuid::Uuid::new_v4().to_string(),
                        item_id: draft.id.clone(),
                        occurred_at: now(),
                        duration_minutes: 23,
                        notes: "Korunan seans".into(),
                    },
                    expected_revision: None,
                },
            )
            .unwrap();
            let children = match kind {
                "album" => Children::Album {
                    tracks: vec![Track {
                        id: uuid::Uuid::new_v4().to_string(),
                        disc_number: 1,
                        track_number: 1,
                        title: "Kişisel parça".into(),
                        duration_seconds: Some(123),
                        favorite: true,
                        notes: "Korunan yorum".into(),
                    }],
                },
                "book" => Children::Book {
                    quotes: vec![Quote {
                        id: uuid::Uuid::new_v4().to_string(),
                        body: "Sentetik alıntı".into(),
                        page_number: Some(20),
                        comment: "Korunan yorum".into(),
                        created_at: now(),
                        updated_at: now(),
                    }],
                },
                _ => Children::Game {
                    screenshots: vec![],
                },
            };
            local = activity::replace(
                &mut c,
                ReplaceChildren {
                    item_id: draft.id.clone(),
                    expected_revision: local.revision,
                    children,
                },
            )
            .unwrap();
            let extras = serde_json::to_value(activity::extras(&c, &draft.id).unwrap()).unwrap();
            let mut incoming = draft.clone();
            incoming.title = "Katalog yeni başlık".into();
            incoming.notes = "Uygulanmayan sağlayıcı notu".into();
            incoming.rating = Some(10);
            let prepared = Prepared {
                existing_item_id: None,
                existing_state: None,
                item: incoming,
                tracks: vec![],
                warnings: vec![],
                cover_lease_id: None,
            };
            let merged = merge(&local, &prepared, &["title".into()]).unwrap();
            let saved = library::save(&mut c, &merged, Some(local.revision), None).unwrap();
            let mut expected = local.item.clone();
            expected.title = "Katalog yeni başlık".into();
            assert_eq!(saved.item, expected);
            assert_eq!(
                serde_json::to_value(activity::extras(&c, &draft.id).unwrap()).unwrap(),
                extras
            );
            assert_eq!(
                library::save(&mut c, &merged, Some(local.revision), None)
                    .unwrap_err()
                    .code,
                "CONFLICT"
            );
        }
    }
}
