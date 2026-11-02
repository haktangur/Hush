use crate::{
    domain::{journal::*, *},
    repositories::{journal, library::Page},
    storage::Storage,
};
use std::sync::Arc;
pub struct JournalService(pub Arc<Storage>);
impl JournalService {
    pub fn list(&self, r: JournalFilter) -> Result<Page<JournalEntry>> {
        let c = self
            .0
            .connection
            .lock()
            .map_err(|_| AppError::new("INTERNAL"))?;
        journal::list(&c, r)
    }
    pub fn save(&self, r: SaveJournal) -> Result<JournalEntry> {
        let mut c = self
            .0
            .connection
            .lock()
            .map_err(|_| AppError::new("INTERNAL"))?;
        journal::save(&mut c, r)
    }
    pub fn draft_get(&self) -> Result<Option<SaveJournal>> {
        let c = self
            .0
            .connection
            .lock()
            .map_err(|_| AppError::new("INTERNAL"))?;
        journal::draft_get(&c)
    }
    pub fn draft_put(&self, r: SaveJournal) -> Result<()> {
        let c = self
            .0
            .connection
            .lock()
            .map_err(|_| AppError::new("INTERNAL"))?;
        journal::draft_put(&c, r)
    }
    pub fn draft_delete(&self) -> Result<()> {
        let c = self
            .0
            .connection
            .lock()
            .map_err(|_| AppError::new("INTERNAL"))?;
        journal::draft_delete(&c)
    }
    pub fn trash(&self, id: &str, rev: i64, restore: bool) -> Result<()> {
        let c = self
            .0
            .connection
            .lock()
            .map_err(|_| AppError::new("INTERNAL"))?;
        journal::trash(&c, id, rev, restore)
    }
    pub fn calendar(&self, r: CalendarRequest) -> Result<CalendarMonth> {
        let c = self
            .0
            .connection
            .lock()
            .map_err(|_| AppError::new("INTERNAL"))?;
        journal::calendar(&c, r)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn draft() -> SaveJournal {
        SaveJournal {
            entry: JournalDraft {
                id: uuid::Uuid::new_v4().to_string(),
                title: "İçimdeki Işık %_".into(),
                entry_date: "2026-11-01".into(),
                mood: Some(Mood::Calm),
                body: "Sentetik günlük gövdesi".into(),
                item_ids: vec![],
            },
            expected_revision: None,
        }
    }
    #[test]
    fn draft_reopen_save_clears_draft_and_calendar_preserves_day() {
        let d = tempfile::tempdir().unwrap();
        let r = draft();
        {
            let s = JournalService(Arc::new(Storage::open(d.path()).unwrap()));
            s.draft_put(r.clone()).unwrap();
        }
        let s = JournalService(Arc::new(Storage::open(d.path()).unwrap()));
        assert_eq!(s.draft_get().unwrap().unwrap().entry.body, r.entry.body);
        let saved = s.save(r).unwrap();
        assert!(s.draft_get().unwrap().is_none());
        let found = s
            .list(JournalFilter {
                id: None,
                query: Some("ışık %_".into()),
                start: None,
                end: None,
                offset: 0,
                limit: 60,
                trashed: None,
            })
            .unwrap();
        assert_eq!(found.total, 1);
        for zone in ["Europe/Istanbul", "America/Los_Angeles"] {
            let month = s
                .calendar(CalendarRequest {
                    year: 2026,
                    month: 11,
                    time_zone: zone.into(),
                })
                .unwrap();
            assert_eq!(month.events[0].day, "2026-11-01");
            assert_eq!(month.journal_days, 1);
        }
        s.trash(&saved.draft.id, 1, false).unwrap();
        s.trash(&saved.draft.id, 2, true).unwrap();
        assert_eq!(
            s.list(JournalFilter {
                id: None,
                query: None,
                start: None,
                end: None,
                offset: 0,
                limit: 60,
                trashed: None
            })
            .unwrap()
            .total,
            1
        );
    }
    #[test]
    fn invalid_day_and_large_body_rejected() {
        let d = tempfile::tempdir().unwrap();
        let s = JournalService(Arc::new(Storage::open(d.path()).unwrap()));
        let mut r = draft();
        r.entry.entry_date = "2026-02-30".into();
        assert!(s.save(r).is_err());
        let mut r = draft();
        r.entry.body = "x".repeat(50001);
        assert!(s.draft_put(r).is_err());
    }
}
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SearchRequest {
    pub query: String,
    pub limit: i64,
}
#[derive(serde::Serialize)]
pub struct SearchResults {
    pub items: Vec<crate::domain::LibraryItem>,
    pub journals: Vec<JournalEntry>,
}
impl JournalService {
    pub fn search(&self, r: SearchRequest) -> Result<SearchResults> {
        if r.query.chars().count() > 300 || !(1..=60).contains(&r.limit) {
            return Err(AppError::new("VALIDATION"));
        }
        let c = self
            .0
            .connection
            .lock()
            .map_err(|_| AppError::new("INTERNAL"))?;
        let items = crate::repositories::library::list(
            &c,
            &crate::repositories::library::LibraryFilter {
                kind: None,
                query: Some(r.query.clone()),
                status: None,
                favorite: None,
                sort: "updated_desc".into(),
                offset: 0,
                limit: r.limit,
                trashed: None,
                collection_id: None,
                tags: None,
            },
        )?
        .items;
        let journals = journal::list(
            &c,
            JournalFilter {
                id: None,
                query: Some(r.query),
                start: None,
                end: None,
                offset: 0,
                limit: r.limit,
                trashed: None,
            },
        )?
        .items;
        Ok(SearchResults { items, journals })
    }
}
