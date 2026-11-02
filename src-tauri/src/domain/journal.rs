use serde::{Deserialize, Serialize};
#[derive(Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Mood {
    Calm,
    Happy,
    Focused,
    Tired,
    Melancholic,
    Excited,
}
#[derive(Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct JournalDraft {
    pub id: String,
    pub title: String,
    pub entry_date: String,
    pub mood: Option<Mood>,
    pub body: String,
    pub item_ids: Vec<String>,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JournalEntry {
    #[serde(flatten)]
    pub draft: JournalDraft,
    pub revision: i64,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SaveJournal {
    pub entry: JournalDraft,
    pub expected_revision: Option<i64>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct JournalFilter {
    pub query: Option<String>,
    pub start: Option<String>,
    pub end: Option<String>,
    pub offset: i64,
    pub limit: i64,
    pub trashed: Option<bool>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CalendarRequest {
    pub year: i32,
    pub month: u32,
    pub time_zone: String,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CalendarEvent {
    pub id: String,
    pub title: String,
    pub day: String,
    pub kind: String,
    pub duration_minutes: Option<i32>,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CalendarMonth {
    pub events: Vec<CalendarEvent>,
    pub added_items: i64,
    pub journal_days: usize,
    pub game_minutes: i64,
    pub book_minutes: i64,
    pub album_minutes: i64,
}
