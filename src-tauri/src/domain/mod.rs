use serde::{Deserialize, Serialize};
use unicode_normalization::UnicodeNormalization;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ItemDraft {
    pub id: String,
    pub title: String,
    pub creator: String,
    pub release_year: Option<i32>,
    pub cover_asset_id: Option<String>,
    pub rating: Option<i32>,
    pub favorite: bool,
    pub notes: String,
    pub tag_names: Vec<String>,
    pub started_on: Option<String>,
    pub completed_on: Option<String>,
    pub provenance: Option<Provenance>,
    #[serde(flatten)]
    pub content: ItemContent,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum ItemContent {
    Game {
        status: Status,
        details: GameDetails,
    },
    Album {
        status: AlbumStatus,
        details: AlbumDetails,
    },
    Book {
        status: Status,
        details: BookDetails,
    },
}
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Planned,
    Active,
    Completed,
    Paused,
    Dropped,
}
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum AlbumStatus {
    Planned,
    Active,
    Completed,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GameDetails {
    pub platform: String,
    pub developer: String,
    pub progress_percent: Option<i32>,
    pub return_note: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AlbumDetails {
    pub first_listened_on: Option<String>,
    pub spotify_url: Option<String>,
    pub apple_music_url: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct BookDetails {
    pub isbn: Option<String>,
    pub publisher: String,
    pub language: String,
    pub total_pages: Option<i32>,
    pub current_page: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Provenance {
    pub provider: String,
    pub entity_type: String,
    pub external_id: String,
    pub edition_id: Option<String>,
    pub source_url: String,
    pub fetched_at: String,
    pub attribution: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LibraryItem {
    #[serde(flatten)]
    pub item: ItemDraft,
    pub revision: i64,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SaveItemRequest {
    pub initial_tracks: Option<Vec<activity::Track>>,
    pub item: ItemDraft,
    pub expected_revision: Option<i64>,
}
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AppError {
    pub code: &'static str,
    pub message: &'static str,
    pub retryable: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_after_seconds: Option<u64>,
}
impl AppError {
    pub fn new(code: &'static str) -> Self {
        Self {
            code,
            retry_after_seconds: None,
            message: code,
            retryable: matches!(code, "DB_BUSY" | "IO"),
        }
    }
}
impl From<rusqlite::Error> for AppError {
    fn from(e: rusqlite::Error) -> Self {
        use rusqlite::{Error, ErrorCode};
        Self::new(match e {
            Error::QueryReturnedNoRows => "NOT_FOUND",
            Error::SqliteFailure(ref err, _) => match err.code {
                ErrorCode::DatabaseBusy | ErrorCode::DatabaseLocked => "DB_BUSY",
                ErrorCode::DiskFull => "DISK_FULL",
                ErrorCode::ConstraintViolation => "CONFLICT",
                _ => "IO",
            },
            _ => "INTERNAL",
        })
    }
}
impl From<std::io::Error> for AppError {
    fn from(error: std::io::Error) -> Self {
        Self::new(if error.kind() == std::io::ErrorKind::StorageFull {
            "DISK_FULL"
        } else {
            "IO"
        })
    }
}
pub type Result<T> = std::result::Result<T, AppError>;
pub fn fold(text: &str) -> String {
    text.replace('I', "ı")
        .replace('İ', "i")
        .to_lowercase()
        .nfc()
        .collect()
}
pub fn now() -> String {
    chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true)
}
pub fn valid_id(id: &str) -> Result<()> {
    uuid::Uuid::parse_str(id)
        .map(|_| ())
        .map_err(|_| AppError::new("VALIDATION"))
}
pub fn valid_day(day: &str) -> Result<()> {
    if day.len() != 10 {
        return Err(AppError::new("VALIDATION"));
    }
    chrono::NaiveDate::parse_from_str(day, "%Y-%m-%d")
        .map(|_| ())
        .map_err(|_| AppError::new("VALIDATION"))
}
impl ItemDraft {
    pub fn kind(&self) -> &'static str {
        match self.content {
            ItemContent::Game { .. } => "game",
            ItemContent::Album { .. } => "album",
            ItemContent::Book { .. } => "book",
        }
    }
    pub fn status(&self) -> &'static str {
        match &self.content {
            ItemContent::Game { status, .. } | ItemContent::Book { status, .. } => match status {
                Status::Planned => "planned",
                Status::Active => "active",
                Status::Completed => "completed",
                Status::Paused => "paused",
                Status::Dropped => "dropped",
            },
            ItemContent::Album { status, .. } => match status {
                AlbumStatus::Planned => "planned",
                AlbumStatus::Active => "active",
                AlbumStatus::Completed => "completed",
            },
        }
    }
    pub fn validate(&mut self) -> Result<()> {
        valid_id(&self.id)?;
        self.title = self.title.trim().to_owned();
        self.creator = self.creator.trim().to_owned();
        self.notes = self.notes.trim().to_owned();
        let invalid = || AppError::new("VALIDATION");
        if self.title.is_empty()
            || self.title.chars().count() > 300
            || self.creator.chars().count() > 500
            || self.notes.chars().count() > 20000
            || self.rating.is_some_and(|x| !(0..=10).contains(&x))
            || self.release_year.is_some_and(|x| !(1..=9999).contains(&x))
            || self.tag_names.len() > 30
        {
            return Err(invalid());
        }
        if let Some(id) = &self.cover_asset_id {
            valid_id(id)?;
        }
        for day in [&self.started_on, &self.completed_on].into_iter().flatten() {
            valid_day(day)?;
        }
        if let (Some(a), Some(b)) = (&self.started_on, &self.completed_on) {
            if b < a {
                return Err(invalid());
            }
        }
        let mut seen = std::collections::HashSet::new();
        self.tag_names = self
            .tag_names
            .iter()
            .map(|s| s.trim().to_owned())
            .filter(|s| seen.insert(fold(s)))
            .collect();
        if self
            .tag_names
            .iter()
            .any(|s| s.is_empty() || s.chars().count() > 60)
        {
            return Err(invalid());
        }
        match &mut self.content {
            ItemContent::Book { status, details } => {
                if details.current_page < 0
                    || details
                        .total_pages
                        .is_some_and(|n| n <= 0 || details.current_page > n)
                {
                    return Err(invalid());
                }
                if *status == Status::Completed {
                    if let Some(n) = details.total_pages {
                        details.current_page = n;
                    }
                }
                if details.publisher.chars().count() > 500
                    || details.language.len() > 100
                    || details.isbn.as_ref().is_some_and(|s| s.len() > 32)
                {
                    return Err(invalid());
                }
            }
            ItemContent::Game { status, details } => {
                if details
                    .progress_percent
                    .is_some_and(|n| !(0..=100).contains(&n))
                    || details.return_note.chars().count() > 2000
                    || details.platform.chars().count() > 500
                    || details.developer.chars().count() > 500
                {
                    return Err(invalid());
                }
                if *status == Status::Completed {
                    details.progress_percent = Some(100);
                    if self.completed_on.is_none() {
                        self.completed_on =
                            Some(chrono::Local::now().format("%Y-%m-%d").to_string());
                    }
                }
            }
            ItemContent::Album { details, .. } => {
                if let Some(day) = &details.first_listened_on {
                    valid_day(day)?;
                }
                for link in [&details.spotify_url, &details.apple_music_url]
                    .into_iter()
                    .flatten()
                {
                    let u = url::Url::parse(link).map_err(|_| invalid())?;
                    if u.scheme() != "https"
                        || u.host_str().is_none()
                        || !u.username().is_empty()
                        || u.password().is_some()
                    {
                        return Err(invalid());
                    }
                }
            }
        }
        if let Some(p) = &self.provenance {
            if !matches!(
                (self.kind(), p.provider.as_str(), p.entity_type.as_str()),
                ("book", "open_library", "work" | "edition")
                    | ("game", "igdb", "game")
                    | ("album", "musicbrainz", "release_group")
            ) || p.external_id.is_empty()
                || p.external_id.len() > 100
            {
                return Err(invalid());
            }
            let u = url::Url::parse(&p.source_url).map_err(|_| invalid())?;
            if u.scheme() != "https" {
                return Err(invalid());
            }
            chrono::DateTime::parse_from_rfc3339(&p.fetched_at).map_err(|_| invalid())?;
        }
        Ok(())
    }
}
pub mod activity;
pub mod collection;
pub mod journal;
