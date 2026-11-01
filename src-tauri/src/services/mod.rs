pub mod settings;
use crate::{domain::*, repositories::library, storage::Storage};
use std::sync::Arc;
pub struct LibraryService(pub Arc<Storage>);
impl LibraryService {
    pub fn save(&self, mut request: SaveItemRequest) -> Result<LibraryItem> {
        request.item.validate()?;
        let mut c = self
            .0
            .connection
            .lock()
            .map_err(|_| AppError::new("INTERNAL"))?;
        library::save(&mut c, &request.item, request.expected_revision)
    }
    pub fn get(&self, id: &str) -> Result<LibraryItem> {
        valid_id(id)?;
        let c = self
            .0
            .connection
            .lock()
            .map_err(|_| AppError::new("INTERNAL"))?;
        library::get(&c, id, false)
    }
    pub fn list(&self, f: library::LibraryFilter) -> Result<library::Page<LibraryItem>> {
        let c = self
            .0
            .connection
            .lock()
            .map_err(|_| AppError::new("INTERNAL"))?;
        library::list(&c, &f)
    }
    pub fn trash(&self, id: &str, revision: i64, restore: bool) -> Result<()> {
        let c = self
            .0
            .connection
            .lock()
            .map_err(|_| AppError::new("INTERNAL"))?;
        library::trash(&c, id, revision, restore)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn book() -> ItemDraft {
        ItemDraft {
            id: uuid::Uuid::new_v4().to_string(),
            title: "IŞIK %_".into(),
            creator: "İpek".into(),
            release_year: Some(2020),
            cover_asset_id: None,
            rating: Some(0),
            favorite: false,
            notes: "Kişisel not".into(),
            tag_names: vec!["İstanbul".into(), "istanbul".into()],
            started_on: None,
            completed_on: None,
            provenance: None,
            content: ItemContent::Book {
                status: Status::Active,
                details: BookDetails {
                    isbn: None,
                    publisher: String::new(),
                    language: "tr".into(),
                    total_pages: Some(300),
                    current_page: 17,
                },
            },
        }
    }
    #[test]
    fn save_reopen_revision_retry_and_trash() {
        let dir = tempfile::tempdir().unwrap();
        let s = LibraryService(Arc::new(Storage::open(dir.path()).unwrap()));
        let draft = book();
        let saved = s
            .save(SaveItemRequest {
                item: draft.clone(),
                expected_revision: None,
            })
            .unwrap();
        assert_eq!(saved.item.tag_names.len(), 1);
        assert_eq!(saved.item.rating, Some(0));
        assert_eq!(
            s.save(SaveItemRequest {
                item: draft,
                expected_revision: None
            })
            .unwrap()
            .revision,
            1
        );
        let mut changed = saved.item.clone();
        changed.notes = "Yeni not".into();
        let updated = s
            .save(SaveItemRequest {
                item: changed.clone(),
                expected_revision: Some(1),
            })
            .unwrap();
        assert_eq!(updated.revision, 2);
        assert_eq!(
            s.save(SaveItemRequest {
                item: changed,
                expected_revision: Some(1)
            })
            .unwrap_err()
            .code,
            "CONFLICT"
        );
        drop(s);
        let s = LibraryService(Arc::new(Storage::open(dir.path()).unwrap()));
        assert_eq!(s.get(&saved.item.id).unwrap().item.notes, "Yeni not");
        s.trash(&saved.item.id, 2, false).unwrap();
        assert_eq!(s.get(&saved.item.id).unwrap_err().code, "NOT_FOUND");
        s.trash(&saved.item.id, 3, true).unwrap();
        assert_eq!(s.get(&saved.item.id).unwrap().revision, 4);
    }
    #[test]
    fn page_validation_and_kind_immutability() {
        let dir = tempfile::tempdir().unwrap();
        let s = LibraryService(Arc::new(Storage::open(dir.path()).unwrap()));
        let mut draft = book();
        if let ItemContent::Book { details, .. } = &mut draft.content {
            details.current_page = 301;
        }
        assert_eq!(
            s.save(SaveItemRequest {
                item: draft,
                expected_revision: None
            })
            .unwrap_err()
            .code,
            "VALIDATION"
        );
        let mut saved = s
            .save(SaveItemRequest {
                item: book(),
                expected_revision: None,
            })
            .unwrap();
        saved.item.content = ItemContent::Game {
            status: Status::Active,
            details: GameDetails {
                platform: String::new(),
                developer: String::new(),
                progress_percent: None,
                return_note: String::new(),
            },
        };
        assert_eq!(
            s.save(SaveItemRequest {
                item: saved.item,
                expected_revision: Some(1)
            })
            .unwrap_err()
            .code,
            "CONFLICT"
        );
    }
    #[test]
    fn turkish_and_literal_search() {
        assert_eq!(fold("Iİıi"), "ıiıi");
        let dir = tempfile::tempdir().unwrap();
        let s = LibraryService(Arc::new(Storage::open(dir.path()).unwrap()));
        s.save(SaveItemRequest {
            item: book(),
            expected_revision: None,
        })
        .unwrap();
        let f = |q: &str| library::LibraryFilter {
            kind: None,
            query: Some(q.into()),
            status: None,
            favorite: None,
            sort: "title_asc".into(),
            offset: 0,
            limit: 60,
            trashed: None,
        };
        assert_eq!(s.list(f("ışık %_")).unwrap().total, 1);
        assert_eq!(s.list(f("%X")).unwrap().total, 0);
    }
    #[test]
    fn failed_foreign_key_save_is_atomic() {
        let dir = tempfile::tempdir().unwrap();
        let s = LibraryService(Arc::new(Storage::open(dir.path()).unwrap()));
        let mut b = book();
        b.cover_asset_id = Some(uuid::Uuid::new_v4().to_string());
        let id = b.id.clone();
        assert!(s
            .save(SaveItemRequest {
                item: b,
                expected_revision: None
            })
            .is_err());
        assert_eq!(s.get(&id).unwrap_err().code, "NOT_FOUND");
    }
    #[test]
    fn dto_roundtrip() {
        let b = book();
        let text = serde_json::to_string(&b).unwrap();
        let decoded: ItemDraft = serde_json::from_str(&text).unwrap();
        assert_eq!(decoded, b);
    }
}
