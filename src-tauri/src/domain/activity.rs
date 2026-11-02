use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SessionDraft {
    pub id: String,
    pub item_id: String,
    pub occurred_at: String,
    pub duration_minutes: i32,
    pub notes: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Session {
    #[serde(flatten)]
    pub draft: SessionDraft,
    pub revision: i64,
    pub created_at: String,
    pub updated_at: String,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SaveSession {
    pub draft: SessionDraft,
    pub expected_revision: Option<i64>,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Track {
    pub id: String,
    pub disc_number: i32,
    pub track_number: i32,
    pub title: String,
    pub duration_seconds: Option<i32>,
    pub favorite: bool,
    pub notes: String,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Quote {
    pub id: String,
    pub body: String,
    pub page_number: Option<i32>,
    pub comment: String,
    pub created_at: String,
    pub updated_at: String,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Screenshot {
    pub id: String,
    pub asset_id: String,
    pub caption: String,
    pub sort_order: i32,
    pub created_at: String,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Extras {
    pub sessions: Vec<Session>,
    pub total_minutes: i64,
    pub tracks: Vec<Track>,
    pub quotes: Vec<Quote>,
    pub screenshots: Vec<Screenshot>,
}
#[derive(Deserialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum Children {
    Album { tracks: Vec<Track> },
    Book { quotes: Vec<Quote> },
    Game { screenshots: Vec<Screenshot> },
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ReplaceChildren {
    pub item_id: String,
    pub expected_revision: i64,
    pub children: Children,
}
