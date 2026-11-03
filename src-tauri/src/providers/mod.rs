use crate::domain::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::{
    collections::HashMap,
    path::PathBuf,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::sync::Mutex;
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CandidateKey {
    pub provider: String,
    pub entity_type: String,
    pub external_id: String,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Candidate {
    pub key: CandidateKey,
    pub title: String,
    pub creator: String,
    pub year: Option<i32>,
    pub provider_page_url: String,
    pub cover_available: bool,
    pub existing_item_id: Option<String>,
    pub existing_state: Option<String>,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CatalogPage {
    pub items: Vec<Candidate>,
    pub page: u32,
    pub has_more: bool,
    pub from_cache: bool,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Search {
    pub kind: String,
    pub query: String,
    pub page: u32,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Prepare {
    pub candidate_key: CandidateKey,
    pub edition_id: Option<String>,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Prepared {
    pub existing_item_id: Option<String>,
    pub existing_state: Option<String>,
    pub item: ItemDraft,
    pub tracks: Vec<crate::domain::activity::Track>,
    pub cover_lease_id: Option<String>,
    pub warnings: Vec<String>,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Edition {
    pub id: String,
    pub title: String,
    pub description: String,
}
#[derive(Serialize)]
pub struct ProviderState {
    pub provider: String,
    pub state: String,
    pub message: Option<String>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Credentials {
    pub client_id: String,
    pub client_secret: String,
}
struct Token {
    value: String,
    client_id: String,
    expires: Instant,
}
pub struct Catalog {
    client: reqwest::Client,
    queues: HashMap<&'static str, Mutex<Instant>>,
    token: Mutex<Option<Token>>,
    pub cache: PathBuf,
    candidates: Mutex<HashMap<CandidateKey, Candidate>>,
}
fn cache_read(path: &std::path::Path, ttl: u64, max: u64) -> Option<Vec<u8>> {
    let meta = std::fs::metadata(path).ok()?;
    if meta.len() > max || meta.modified().ok()?.elapsed().ok()?.as_secs() >= ttl {
        return None;
    }
    let bytes = std::fs::read(path).ok()?;
    let _ = std::fs::File::open(path).and_then(|f| {
        f.set_times(std::fs::FileTimes::new().set_accessed(std::time::SystemTime::now()))
    });
    Some(bytes)
}
fn credential_service() -> String {
    #[cfg(feature = "e2e")]
    if let Some(root) = std::env::var_os("HUSH_TEST_DATA_DIR") {
        let hash = Sha256::digest(root.to_string_lossy().as_bytes())
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect::<String>();
        return format!("app.hush.desktop.e2e.{hash}");
    }
    "app.hush.desktop".into()
}
fn retry_after(value: &str) -> Option<Duration> {
    if let Ok(seconds) = value.trim().parse::<u64>() {
        return Some(Duration::from_secs(seconds));
    }
    let date = chrono::DateTime::parse_from_rfc2822(value).ok()?;
    Some(
        (date.with_timezone(&chrono::Utc) - chrono::Utc::now())
            .to_std()
            .unwrap_or_default(),
    )
}
fn failure() -> AppError {
    AppError::new("PROVIDER_UNAVAILABLE")
}
pub fn allowed(url: &url::Url) -> bool {
    if url.scheme() != "https"
        || url.port().is_some()
        || !url.username().is_empty()
        || url.password().is_some()
    {
        return false;
    }
    let Some(host) = url.host_str() else {
        return false;
    };
    matches!(
        host,
        "openlibrary.org"
            | "covers.openlibrary.org"
            | "musicbrainz.org"
            | "coverartarchive.org"
            | "archive.org"
            | "api.igdb.com"
            | "images.igdb.com"
            | "id.twitch.tv"
    ) || host.ends_with(".archive.org")
}
pub fn escape_query(s: &str) -> String {
    s.chars()
        .flat_map(|c| {
            if "+-!(){}[]^\"~*?:\\/&|".contains(c) {
                vec!['\\', c]
            } else {
                vec![c]
            }
        })
        .collect()
}
fn string(v: &Value, key: &str) -> String {
    v[key].as_str().unwrap_or("").to_owned()
}
fn year(v: &str) -> Option<i32> {
    v.get(..4)?.parse().ok().filter(|n| *n > 0 && *n < 10000)
}
fn publication_year(date: &str) -> Option<i32> {
    date.split(|c: char| !c.is_ascii_digit())
        .find_map(|s| if s.len() == 4 { year(s) } else { None })
}
fn credit(v: &Value) -> String {
    v.as_array()
        .map(|a| {
            a.iter()
                .map(|x| {
                    format!(
                        "{}{}",
                        x["name"]
                            .as_str()
                            .or(x["artist"]["name"].as_str())
                            .unwrap_or(""),
                        x["joinphrase"].as_str().unwrap_or("")
                    )
                })
                .collect()
        })
        .unwrap_or_default()
}
fn ol_id(id: &str, suffix: char) -> bool {
    id.starts_with("OL")
        && id.ends_with(suffix)
        && id.len() > 3
        && id.len() < 32
        && id[2..id.len() - 1].chars().all(|c| c.is_ascii_digit())
}
impl Catalog {
    pub fn new(cache: PathBuf) -> Result<Self> {
        let client = reqwest::Client::builder()
            .connect_timeout(Duration::from_secs(5))
            .timeout(Duration::from_secs(15))
            .redirect(reqwest::redirect::Policy::custom(|a| {
                if a.previous().len() >= 5
                    || !allowed(a.url())
                    || a.previous().first().is_some_and(|u| {
                        matches!(u.host_str(), Some("id.twitch.tv" | "api.igdb.com"))
                            && u.host_str() != a.url().host_str()
                    })
                {
                    a.stop()
                } else {
                    a.follow()
                }
            }))
            .build()
            .map_err(|_| failure())?;
        Ok(Self {
            client,
            queues: ["book", "album", "game"]
                .into_iter()
                .map(|s| (s, Mutex::new(Instant::now() - Duration::from_secs(5))))
                .collect(),
            token: Mutex::new(None),
            cache,
            candidates: Mutex::new(HashMap::new()),
        })
    }
    async fn throttle(&self, kind: &str) -> Result<()> {
        let queue = self.queues.get(kind).ok_or(AppError::new("VALIDATION"))?;
        let mut last = queue.lock().await;
        let delay = Duration::from_millis(if kind == "game" { 500 } else { 1100 });
        if let Some(wait) = (*last + delay).checked_duration_since(Instant::now()) {
            if wait > Duration::from_secs(15) {
                let mut error = AppError::new("RATE_LIMITED");
                error.retry_after_seconds = Some(wait.as_secs() + 1);
                return Err(error);
            }
            tokio::time::sleep(wait).await;
        }
        *last = Instant::now();
        Ok(())
    }
    fn agent(contact: Option<&str>) -> String {
        match contact {
            Some(c) => format!("Hush/{} ({c})", env!("CARGO_PKG_VERSION")),
            None => format!("Hush/{}", env!("CARGO_PKG_VERSION")),
        }
    }
    async fn body(
        &self,
        request: reqwest::RequestBuilder,
        kind: &str,
        max: usize,
    ) -> Result<Vec<u8>> {
        'attempts: for attempt in 0..3 {
            self.throttle(kind).await?;
            #[cfg(feature = "e2e")]
            if let Some(root) = std::env::var_os("HUSH_TEST_DATA_DIR") {
                use std::io::Write;
                let root = std::path::PathBuf::from(root);
                if root.join("offline").exists() {
                    return Err(AppError::new("OFFLINE"));
                }
                std::fs::OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(root.join("http-attempts.txt"))?
                    .write_all(b"send\n")?;
            }
            let response = request.try_clone().ok_or_else(failure)?.send().await;
            let mut response = match response {
                Ok(r) => r,
                Err(e) if e.is_timeout() && attempt < 2 => {
                    self.backoff(kind, attempt, None).await?;
                    continue;
                }
                Err(e) => {
                    return Err(AppError::new(if e.is_timeout() {
                        "TIMEOUT"
                    } else {
                        "OFFLINE"
                    }))
                }
            };
            let status = response.status().as_u16();
            if status == 429 || status == 503 {
                let delay = response
                    .headers()
                    .get("Retry-After")
                    .and_then(|h| h.to_str().ok())
                    .and_then(retry_after);
                self.backoff(kind, attempt, delay).await?;
                if attempt < 2 {
                    continue;
                }
            }
            if !response.status().is_success() {
                return Err(AppError::new(match status {
                    429 => "RATE_LIMITED",
                    401 => "PROVIDER_UNAUTHORIZED",
                    403 => "PROVIDER_FORBIDDEN",
                    404 => "PROVIDER_NOT_FOUND",
                    _ => "PROVIDER_UNAVAILABLE",
                }));
            }
            if response.content_length().is_some_and(|n| n > max as u64) {
                return Err(failure());
            }
            let mut out = Vec::new();
            loop {
                let chunk = match response.chunk().await {
                    Ok(Some(chunk)) => chunk,
                    Ok(None) => break,
                    Err(e) if e.is_timeout() && attempt < 2 => {
                        self.backoff(kind, attempt, None).await?;
                        continue 'attempts;
                    }
                    Err(e) => {
                        return Err(AppError::new(if e.is_timeout() {
                            "TIMEOUT"
                        } else {
                            "PROVIDER_UNAVAILABLE"
                        }))
                    }
                };
                if out.len() + chunk.len() > max {
                    return Err(failure());
                }
                out.extend_from_slice(&chunk);
            }
            return Ok(out);
        }
        Err(failure())
    }
    async fn backoff(&self, kind: &str, attempt: u64, server: Option<Duration>) -> Result<()> {
        let jitter = u64::from(uuid::Uuid::new_v4().as_bytes()[0]);
        let wait = server
            .unwrap_or_default()
            .max(Duration::from_millis(1200 * (attempt + 1) + jitter));
        let mut next = self
            .queues
            .get(kind)
            .ok_or(AppError::new("VALIDATION"))?
            .lock()
            .await;
        *next = (*next).max(
            Instant::now()
                .checked_add(wait)
                .ok_or(AppError::new("RATE_LIMITED"))?,
        );
        Ok(())
    }
    async fn json(
        &self,
        url: url::Url,
        kind: &str,
        contact: Option<&str>,
        ttl: u64,
    ) -> Result<(Value, bool)> {
        if !allowed(&url) {
            return Err(AppError::new("VALIDATION"));
        }
        let hash: String = Sha256::digest(url.as_str().as_bytes())
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect();
        let file = self.cache.join(format!("{hash}.json"));
        if let Some(bytes) = cache_read(&file, ttl, 5 * 1024 * 1024) {
            if let Ok(value) = serde_json::from_slice(&bytes) {
                return Ok((value, true));
            }
        }
        if url.host_str() == Some("musicbrainz.org") && contact.is_none() {
            return Err(AppError::new("PROVIDER_NOT_CONFIGURED"));
        }
        let bytes = self
            .body(
                self.client
                    .get(url)
                    .header("User-Agent", Self::agent(contact)),
                kind,
                5 * 1024 * 1024,
            )
            .await?;
        let value = serde_json::from_slice(&bytes).map_err(|_| failure())?;
        std::fs::create_dir_all(&self.cache)?;
        self.trim_cache(bytes.len())?;
        std::fs::write(file, bytes)?;
        Ok((value, false))
    }
    fn trim_cache(&self, incoming: usize) -> Result<()> {
        let mut entries = std::fs::read_dir(&self.cache)?
            .filter_map(|e| e.ok())
            .filter_map(|e| {
                e.metadata()
                    .ok()
                    .map(|m| (e.path(), m.len(), m.accessed().ok()))
            })
            .collect::<Vec<_>>();
        let mut total: u64 = entries.iter().map(|e| e.1).sum::<u64>() + incoming as u64;
        let mut count = entries.len();
        entries.sort_by_key(|e| e.2);
        for (path, size, _) in entries {
            if total <= 100 * 1024 * 1024 && count < 2000 {
                break;
            }
            std::fs::remove_file(path)?;
            total = total.saturating_sub(size);
            count = count.saturating_sub(1);
        }
        Ok(())
    }
    async fn cover_bytes(
        &self,
        url: url::Url,
        kind: &str,
        contact: Option<&str>,
        ttl: u64,
    ) -> Result<Vec<u8>> {
        let hash = Sha256::digest(url.as_str().as_bytes())
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect::<String>();
        let file = self.cache.join(format!("{hash}.cover"));
        if let Some(bytes) = cache_read(&file, ttl, 20 * 1024 * 1024) {
            return Ok(bytes);
        }
        let bytes = self
            .body(
                self.client
                    .get(url)
                    .header("User-Agent", Self::agent(contact))
                    .timeout(Duration::from_secs(20)),
                kind,
                20 * 1024 * 1024,
            )
            .await?;
        std::fs::create_dir_all(&self.cache)?;
        self.trim_cache(bytes.len())?;
        std::fs::write(file, &bytes)?;
        Ok(bytes)
    }
    pub async fn token(&self) -> Result<(String, String)> {
        let mut cached = self.token.lock().await;
        if let Some(t) = cached.as_ref() {
            if t.expires > Instant::now() + Duration::from_secs(60) {
                return Ok((t.client_id.clone(), t.value.clone()));
            }
        }
        let (id, secret) = tauri::async_runtime::spawn_blocking(|| {
            let id = keyring::Entry::new(&credential_service(), "igdb-client-id")
                .map_err(|_| AppError::new("PROVIDER_NOT_CONFIGURED"))?
                .get_password()
                .map_err(|_| AppError::new("PROVIDER_NOT_CONFIGURED"))?;
            let secret = keyring::Entry::new(&credential_service(), "igdb-client-secret")
                .map_err(|_| AppError::new("PROVIDER_NOT_CONFIGURED"))?
                .get_password()
                .map_err(|_| AppError::new("PROVIDER_NOT_CONFIGURED"))?;
            Ok::<_, AppError>((id, secret))
        })
        .await
        .map_err(|_| failure())??;
        let token = self.authenticate(&id, &secret).await?;
        let value = token.value.clone();
        *cached = Some(token);
        Ok((id, value))
    }
    async fn authenticate(&self, id: &str, secret: &str) -> Result<Token> {
        let bytes = self
            .body(
                self.client
                    .post("https://id.twitch.tv/oauth2/token")
                    .form(&[
                        ("client_id", id),
                        ("client_secret", secret),
                        ("grant_type", "client_credentials"),
                    ]),
                "game",
                1024 * 1024,
            )
            .await?;
        let v: Value = serde_json::from_slice(&bytes).map_err(|_| failure())?;
        let token = v["access_token"].as_str().ok_or_else(failure)?.to_owned();
        let expires = v["expires_in"].as_u64().unwrap_or(3600).min(86400);
        Ok(Token {
            value: token,
            client_id: id.into(),
            expires: Instant::now() + Duration::from_secs(expires),
        })
    }
    async fn igdb(&self, query: String, ttl: u64) -> Result<(Value, bool)> {
        let hash = Sha256::digest(format!("igdb:{query}").as_bytes())
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect::<String>();
        let file = self.cache.join(format!("{hash}.json"));
        if let Some(bytes) = cache_read(&file, ttl, 5 * 1024 * 1024) {
            if let Ok(value) = serde_json::from_slice(&bytes) {
                return Ok((value, true));
            }
        }

        for attempt in 0..2 {
            let (id, token) = self.token().await?;
            match self
                .body(
                    self.client
                        .post("https://api.igdb.com/v4/games")
                        .header("Client-ID", id)
                        .bearer_auth(token)
                        .body(query.clone()),
                    "game",
                    5 * 1024 * 1024,
                )
                .await
            {
                Ok(bytes) => {
                    let value = serde_json::from_slice(&bytes).map_err(|_| failure())?;
                    std::fs::create_dir_all(&self.cache)?;
                    self.trim_cache(bytes.len())?;
                    std::fs::write(&file, bytes)?;
                    return Ok((value, false));
                }
                Err(e) if e.code == "PROVIDER_UNAUTHORIZED" && attempt == 0 => {
                    *self.token.lock().await = None;
                }
                Err(e) => return Err(e),
            }
        }
        Err(failure())
    }
    pub async fn search(&self, r: Search, contact: Option<&str>) -> Result<CatalogPage> {
        if r.query.trim().chars().count() < 2
            || r.query.chars().count() > 300
            || r.page == 0
            || r.page > 100
        {
            return Err(AppError::new("VALIDATION"));
        }
        let (value, cached) = match r.kind.as_str() {
            "book" => {
                let mut u = url::Url::parse("https://openlibrary.org/search.json").unwrap();
                u.query_pairs_mut()
                    .append_pair("q", &r.query)
                    .append_pair("limit", "20")
                    .append_pair("page", &r.page.to_string())
                    .append_pair("lang", "tr")
                    .append_pair(
                        "fields",
                        "key,title,author_name,first_publish_year,cover_i,edition_key",
                    );
                self.json(u, "book", contact, 86400).await?
            }
            "album" => {
                let mut u = url::Url::parse("https://musicbrainz.org/ws/2/release-group/").unwrap();
                u.query_pairs_mut()
                    .append_pair(
                        "query",
                        &format!(
                            "releasegroup:\"{}\" AND primarytype:album",
                            escape_query(&r.query)
                        ),
                    )
                    .append_pair("fmt", "json")
                    .append_pair("limit", "20")
                    .append_pair("offset", &((r.page - 1) * 20).to_string());
                self.json(u, "album", contact, 86400).await?
            }
            "game" => {
                let escaped = r
                    .query
                    .replace('\\', "\\\\")
                    .replace('"', "\\\"")
                    .replace(['\n', '\r'], " ");
                self.igdb(format!("search \"{escaped}\"; fields id,name,first_release_date,cover.image_id,platforms.name,involved_companies.company.name,involved_companies.developer,url; limit 20; offset {};",(r.page-1)*20),86400).await?
            }
            _ => return Err(AppError::new("VALIDATION")),
        };
        let items = map_search(&r.kind, &value)?;
        let has_more = items.len() == 20;
        let mut candidates = self.candidates.lock().await;
        for c in &items {
            candidates.insert(c.key.clone(), c.clone());
        }
        if candidates.len() > 2000 {
            candidates.clear();
            for c in &items {
                candidates.insert(c.key.clone(), c.clone());
            }
        }
        Ok(CatalogPage {
            items,
            page: r.page,
            has_more,
            from_cache: cached,
        })
    }
    pub async fn editions(&self, key: CandidateKey, contact: Option<&str>) -> Result<Vec<Edition>> {
        validate_key(&key)?;
        let (u, kind) = if key.provider == "open_library" {
            (
                format!(
                    "https://openlibrary.org/works/{}/editions.json?limit=20",
                    key.external_id
                ),
                "book",
            )
        } else if key.provider == "musicbrainz" {
            (
                format!(
                    "https://musicbrainz.org/ws/2/release?release-group={}&fmt=json&limit=20",
                    key.external_id
                ),
                "album",
            )
        } else {
            return Ok(vec![]);
        };
        let (v, _) = self
            .json(
                url::Url::parse(&u).map_err(|_| failure())?,
                kind,
                contact,
                604800,
            )
            .await?;
        let rows = if kind == "book" {
            &v["entries"]
        } else {
            &v["releases"]
        };
        Ok(rows
            .as_array()
            .map(|a| {
                a.iter()
                    .map(|x| Edition {
                        id: if kind == "book" {
                            string(x, "key").trim_start_matches("/books/").into()
                        } else {
                            string(x, "id")
                        },
                        title: string(x, "title"),
                        description: if kind == "book" {
                            string(x, "publish_date")
                        } else {
                            format!("{} · {}", string(x, "date"), string(x, "country"))
                        },
                    })
                    .collect()
            })
            .unwrap_or_default())
    }
    pub async fn prepare(
        &self,
        r: Prepare,
        s: Arc<crate::storage::Storage>,
        contact: Option<&str>,
    ) -> Result<Prepared> {
        self.prepare_with_ttl(r, s, contact, 604800).await
    }
    pub async fn prepare_with_ttl(
        &self,
        r: Prepare,
        s: Arc<crate::storage::Storage>,
        contact: Option<&str>,
        ttl: u64,
    ) -> Result<Prepared> {
        validate_key(&r.candidate_key)?;
        let key = &r.candidate_key;
        let candidate = self.candidates.lock().await.get(key).cloned();
        let mut item = ItemDraft {
            id: uuid::Uuid::new_v4().to_string(),
            title: candidate
                .as_ref()
                .map(|c| c.title.clone())
                .unwrap_or_default(),
            creator: candidate
                .as_ref()
                .map(|c| c.creator.clone())
                .unwrap_or_default(),
            release_year: candidate.as_ref().and_then(|c| c.year),
            cover_asset_id: None,
            rating: None,
            favorite: false,
            notes: String::new(),
            tag_names: vec![],
            started_on: None,
            completed_on: None,
            provenance: None,
            content: ItemContent::Book {
                status: Status::Planned,
                details: BookDetails {
                    isbn: None,
                    publisher: String::new(),
                    language: String::new(),
                    total_pages: None,
                    current_page: 0,
                },
            },
        };
        let mut tracks = vec![];
        let mut cover = None;
        let mut entity = key.entity_type.clone();
        let mut external_id = key.external_id.clone();
        let (kind, source) = match key.provider.as_str() {
            "open_library" => {
                let edition = r.edition_id.as_ref().or(if key.entity_type == "edition" {
                    Some(&key.external_id)
                } else {
                    None
                });
                if let Some(id) = edition {
                    if !ol_id(id, 'M') {
                        return Err(AppError::new("VALIDATION"));
                    }
                    entity = "edition".into();
                    external_id = id.clone();
                }
                let u = if entity == "edition" {
                    format!("https://openlibrary.org/books/{external_id}.json")
                } else {
                    format!("https://openlibrary.org/works/{external_id}.json")
                };
                let (v, _) = self
                    .json(url::Url::parse(&u).unwrap(), "book", contact, ttl)
                    .await?;
                item.title = string(&v, "title");
                if let Some(authors) = v["authors"].as_array() {
                    let mut names = Vec::new();
                    for author in authors.iter().take(8) {
                        let key = author["key"]
                            .as_str()
                            .or(author["author"]["key"].as_str())
                            .unwrap_or("");
                        if let Some(id) = key.strip_prefix("/authors/").filter(|id| ol_id(id, 'A'))
                        {
                            let u = url::Url::parse(&format!(
                                "https://openlibrary.org/authors/{id}.json"
                            ))
                            .map_err(|_| failure())?;
                            let (a, _) = self.json(u, "book", contact, ttl).await?;
                            let name = string(&a, "name");
                            if !name.is_empty() {
                                names.push(name);
                            }
                        }
                    }
                    if !names.is_empty() {
                        item.creator = names.join(", ");
                    }
                }
                if let Some(id) = v["covers"]
                    .as_array()
                    .and_then(|a| a.iter().find_map(|x| x.as_i64().filter(|n| *n > 0)))
                {
                    cover = Some(format!(
                        "https://covers.openlibrary.org/b/id/{id}-L.jpg?default=false"
                    ));
                }
                if entity == "edition" {
                    item.release_year =
                        publication_year(&string(&v, "publish_date")).or(item.release_year);
                    item.content = ItemContent::Book {
                        status: Status::Planned,
                        details: BookDetails {
                            isbn: v["isbn_13"][0]
                                .as_str()
                                .or(v["isbn_10"][0].as_str())
                                .map(str::to_owned),
                            publisher: v["publishers"][0].as_str().unwrap_or("").into(),
                            language: v["languages"][0]["key"]
                                .as_str()
                                .unwrap_or("")
                                .trim_start_matches("/languages/")
                                .into(),
                            total_pages: v["number_of_pages"]
                                .as_i64()
                                .and_then(|n| i32::try_from(n).ok())
                                .filter(|n| *n > 0),
                            current_page: 0,
                        },
                    };
                }
                ("book", u.trim_end_matches(".json").to_owned())
            }
            "musicbrainz" => {
                let u = format!(
                    "https://musicbrainz.org/ws/2/release-group/{}?inc=artist-credits&fmt=json",
                    key.external_id
                );
                let (v, _) = self
                    .json(url::Url::parse(&u).unwrap(), "album", contact, ttl)
                    .await?;
                item.title = string(&v, "title");
                item.creator = credit(&v["artist-credit"]);
                item.release_year = year(&string(&v, "first-release-date"));
                item.content = ItemContent::Album {
                    status: AlbumStatus::Planned,
                    details: AlbumDetails {
                        first_listened_on: None,
                        spotify_url: None,
                        apple_music_url: None,
                    },
                };
                if let Some(id) = &r.edition_id {
                    valid_id(id)?;
                    let u=format!("https://musicbrainz.org/ws/2/release/{id}?inc=recordings+artist-credits&fmt=json");
                    let (release, _) = self
                        .json(url::Url::parse(&u).unwrap(), "album", contact, ttl)
                        .await?;
                    if let Some(media) = release["media"].as_array() {
                        for (disc, m) in media.iter().enumerate() {
                            if let Some(ts) = m["tracks"].as_array() {
                                for (ti, t) in ts.iter().enumerate() {
                                    tracks.push(crate::domain::activity::Track {
                                        id: uuid::Uuid::new_v4().to_string(),
                                        disc_number: (disc + 1) as i32,
                                        track_number: (ti + 1) as i32,
                                        title: t["title"]
                                            .as_str()
                                            .or(t["recording"]["title"].as_str())
                                            .unwrap_or("")
                                            .into(),
                                        duration_seconds: t["length"]
                                            .as_i64()
                                            .filter(|n| *n > 0)
                                            .and_then(|n| i32::try_from((n / 1000).max(1)).ok()),
                                        favorite: false,
                                        notes: String::new(),
                                    });
                                }
                            }
                        }
                    }
                    cover = Some(format!(
                        "https://coverartarchive.org/release/{id}/front-500"
                    ));
                } else {
                    cover = Some(format!(
                        "https://coverartarchive.org/release-group/{}/front-500",
                        key.external_id
                    ));
                }
                (
                    "album",
                    format!("https://musicbrainz.org/release-group/{}", key.external_id),
                )
            }
            "igdb" => {
                let v=self.igdb(format!("where id = {}; fields id,name,first_release_date,cover.image_id,platforms.name,involved_companies.company.name,involved_companies.developer,url;",key.external_id),ttl).await?.0;
                let v = &v[0];
                item.title = string(v, "name");
                item.creator = v["involved_companies"]
                    .as_array()
                    .map(|a| {
                        a.iter()
                            .filter(|x| x["developer"] == true)
                            .map(|x| string(&x["company"], "name"))
                            .collect::<Vec<_>>()
                            .join(", ")
                    })
                    .unwrap_or_default();
                item.release_year = v["first_release_date"]
                    .as_i64()
                    .and_then(|n| chrono::DateTime::from_timestamp(n, 0))
                    .map(|d| {
                        use chrono::Datelike;
                        d.year()
                    });
                let image_id = string(&v["cover"], "image_id");
                if !image_id.is_empty()
                    && image_id
                        .chars()
                        .all(|c| c.is_ascii_alphanumeric() || c == '_')
                {
                    cover = Some(format!(
                        "https://images.igdb.com/igdb/image/upload/t_cover_big/{image_id}.jpg"
                    ));
                }
                item.content = ItemContent::Game {
                    status: Status::Planned,
                    details: GameDetails {
                        platform: v["platforms"]
                            .as_array()
                            .map(|a| {
                                a.iter()
                                    .map(|x| string(x, "name"))
                                    .collect::<Vec<_>>()
                                    .join(", ")
                            })
                            .unwrap_or_default(),
                        developer: item.creator.clone(),
                        progress_percent: None,
                        return_note: String::new(),
                    },
                };
                ("game", string(v, "url"))
            }
            _ => return Err(AppError::new("VALIDATION")),
        };
        item.provenance = Some(Provenance {
            provider: key.provider.clone(),
            entity_type: entity,
            external_id,
            edition_id: r.edition_id,
            source_url: source,
            fetched_at: now(),
            attribution: key.provider.clone(),
        });
        let mut warnings = vec![];
        if let Some(u) = cover {
            let u = url::Url::parse(&u).map_err(|_| failure())?;
            let missing = self.cache.join(format!(
                "{}.missing",
                Sha256::digest(u.as_str().as_bytes())
                    .iter()
                    .map(|b| format!("{b:02x}"))
                    .collect::<String>()
            ));
            let negative = std::fs::metadata(&missing)
                .ok()
                .and_then(|m| m.modified().ok())
                .and_then(|t| t.elapsed().ok())
                .is_some_and(|d| d.as_secs() < 3600);
            if negative {
                warnings.push("COVER_UNAVAILABLE".into());
            }
            if allowed(&u) && !negative {
                match self.cover_bytes(u, kind, contact, ttl).await {
                    Ok(bytes) => {
                        let storage = s.clone();
                        let imported = tauri::async_runtime::spawn_blocking(move || {
                            let staging = storage.generation.join("cover-download");
                            std::fs::create_dir_all(&staging)?;
                            let path = staging.join(uuid::Uuid::new_v4().to_string());
                            std::fs::write(&path, bytes)?;
                            let result = crate::storage::assets::import(&storage, &path, "cover");
                            let _ = std::fs::remove_file(path);
                            result
                        })
                        .await
                        .map_err(|_| failure())?;
                        match imported {
                            Ok(a) => item.cover_asset_id = Some(a.id),
                            Err(_) => {
                                warnings.push("COVER_UNAVAILABLE".into());
                            }
                        }
                    }
                    Err(e) => {
                        if e.code == "PROVIDER_NOT_FOUND" {
                            let _ = std::fs::create_dir_all(&self.cache);
                            let _ = std::fs::write(&missing, []);
                        }
                        warnings.push("COVER_UNAVAILABLE".into());
                    }
                }
            }
        }
        item.validate()?;
        let provenance = item.provenance.clone().ok_or_else(failure)?;
        let existing = tauri::async_runtime::spawn_blocking(move || {
            let connection = s.connection.lock().map_err(|_| failure())?;
            crate::repositories::library::catalog_existing(
                &connection,
                &provenance.provider,
                &provenance.entity_type,
                &provenance.external_id,
            )
        })
        .await
        .map_err(|_| failure())??;
        Ok(Prepared {
            existing_item_id: existing.as_ref().map(|(id, _)| id.clone()),
            existing_state: existing
                .map(|(_, trashed)| if trashed { "trashed" } else { "active" }.into()),
            item,
            tracks,
            cover_lease_id: None,
            warnings,
        })
    }
    pub async fn disconnect(&self) -> Result<()> {
        let mut token = self.token.lock().await;
        *token = None;
        tauri::async_runtime::spawn_blocking(|| {
            for name in ["igdb-client-id", "igdb-client-secret"] {
                let e = keyring::Entry::new(&credential_service(), name).map_err(|_| failure())?;
                match e.delete_credential() {
                    Ok(()) | Err(keyring::Error::NoEntry) => (),
                    Err(_) => return Err(failure()),
                }
            }
            Ok(())
        })
        .await
        .map_err(|_| failure())?
    }
    pub async fn configure(&self, r: Credentials) -> Result<ProviderState> {
        if r.client_id.trim().is_empty()
            || r.client_secret.is_empty()
            || r.client_id.len() > 300
            || r.client_secret.len() > 1000
        {
            return Err(AppError::new("VALIDATION"));
        }
        let mut cached = self.token.lock().await;
        let verified = self
            .authenticate(r.client_id.trim(), &r.client_secret)
            .await?;
        tauri::async_runtime::spawn_blocking(move || {
            let id = keyring::Entry::new(&credential_service(), "igdb-client-id")
                .map_err(|_| failure())?;
            let secret = keyring::Entry::new(&credential_service(), "igdb-client-secret")
                .map_err(|_| failure())?;
            let previous = match secret.get_password() {
                Ok(v) => Some(v),
                Err(keyring::Error::NoEntry) => None,
                Err(_) => return Err(failure()),
            };
            secret
                .set_password(&r.client_secret)
                .map_err(|_| failure())?;
            if id.set_password(r.client_id.trim()).is_err() {
                match previous {
                    Some(value) => secret.set_password(&value),
                    None => secret.delete_credential(),
                }
                .map_err(|_| failure())?;
                return Err(failure());
            }
            Ok::<_, AppError>(())
        })
        .await
        .map_err(|_| failure())??;
        *cached = Some(verified);
        Ok(ProviderState {
            provider: "igdb".into(),
            state: "ready".into(),
            message: None,
        })
    }
    pub fn status() -> ProviderState {
        let ready = keyring::Entry::new(&credential_service(), "igdb-client-id")
            .ok()
            .and_then(|e| e.get_password().ok())
            .is_some()
            && keyring::Entry::new(&credential_service(), "igdb-client-secret")
                .ok()
                .and_then(|e| e.get_password().ok())
                .is_some();
        ProviderState {
            provider: "igdb".into(),
            state: if ready { "ready" } else { "not_configured" }.into(),
            message: None,
        }
    }
}
pub fn validate_key(k: &CandidateKey) -> Result<()> {
    let valid = match (k.provider.as_str(), k.entity_type.as_str()) {
        ("open_library", "work") => ol_id(&k.external_id, 'W'),
        ("open_library", "edition") => ol_id(&k.external_id, 'M'),
        ("musicbrainz", "release_group") => uuid::Uuid::parse_str(&k.external_id).is_ok(),
        ("igdb", "game") => k.external_id.parse::<u64>().is_ok_and(|n| n > 0),
        _ => false,
    };
    if valid {
        Ok(())
    } else {
        Err(AppError::new("VALIDATION"))
    }
}
pub fn map_search(kind: &str, v: &Value) -> Result<Vec<Candidate>> {
    let rows = match kind {
        "book" => &v["docs"],
        "album" => &v["release-groups"],
        "game" => v,
        _ => return Err(AppError::new("VALIDATION")),
    };
    let Some(rows) = rows.as_array() else {
        return Err(failure());
    };
    let mut out = vec![];
    for v in rows.iter().take(20) {
        let (key, title, creator, year, page, cover) = match kind {
            "book" => {
                let id = string(v, "key").trim_start_matches("/works/").to_owned();
                (
                    CandidateKey {
                        provider: "open_library".into(),
                        entity_type: "work".into(),
                        external_id: id.clone(),
                    },
                    string(v, "title"),
                    v["author_name"]
                        .as_array()
                        .map(|a| {
                            a.iter()
                                .filter_map(|s| s.as_str())
                                .collect::<Vec<_>>()
                                .join(", ")
                        })
                        .unwrap_or_default(),
                    v["first_publish_year"]
                        .as_i64()
                        .and_then(|n| i32::try_from(n).ok()),
                    format!("https://openlibrary.org/works/{id}"),
                    v["cover_i"].as_i64().is_some(),
                )
            }
            "album" => {
                let id = string(v, "id");
                (
                    CandidateKey {
                        provider: "musicbrainz".into(),
                        entity_type: "release_group".into(),
                        external_id: id.clone(),
                    },
                    string(v, "title"),
                    credit(&v["artist-credit"]),
                    year(&string(v, "first-release-date")),
                    format!("https://musicbrainz.org/release-group/{id}"),
                    true,
                )
            }
            _ => {
                let id = v["id"].as_u64().unwrap_or(0).to_string();
                (
                    CandidateKey {
                        provider: "igdb".into(),
                        entity_type: "game".into(),
                        external_id: id,
                    },
                    string(v, "name"),
                    String::new(),
                    None,
                    string(v, "url"),
                    v["cover"]["image_id"].is_string(),
                )
            }
        };
        if validate_key(&key).is_ok() && !title.trim().is_empty() {
            out.push(Candidate {
                key,
                title,
                creator,
                year,
                provider_page_url: page,
                cover_available: cover,
                existing_item_id: None,
                existing_state: None,
            })
        }
    }
    Ok(out)
}
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    #[test]
    fn disk_cache_evicts_least_recently_used_before_incoming_bytes() {
        let root = tempfile::tempdir().unwrap();
        let catalog = Catalog::new(root.path().into()).unwrap();
        for (name, age) in [("old", 100), ("new", 1)] {
            let file = std::fs::File::create(root.path().join(name)).unwrap();
            file.set_len(60 * 1024 * 1024).unwrap();
            file.set_times(
                std::fs::FileTimes::new()
                    .set_accessed(std::time::SystemTime::now() - Duration::from_secs(age)),
            )
            .unwrap();
        }
        catalog.trim_cache(20 * 1024 * 1024).unwrap();
        assert!(!root.path().join("old").exists());
        assert!(root.path().join("new").exists());
    }
    #[test]
    fn retry_after_cooldown_is_shared_by_following_requests() {
        let root = tempfile::tempdir().unwrap();
        let catalog = Catalog::new(root.path().into()).unwrap();
        tauri::async_runtime::block_on(async {
            catalog
                .backoff("album", 0, Some(Duration::from_secs(60)))
                .await
                .unwrap();
            let error = catalog.throttle("album").await.unwrap_err();
            assert_eq!(error.code, "RATE_LIMITED");
            assert!(error.retry_after_seconds.unwrap() >= 60);
            assert!(catalog.throttle("book").await.is_ok());
        });
    }
    #[test]
    fn cache_expiry_corruption_limits_and_publication_dates() {
        let root = tempfile::tempdir().unwrap();
        let file = root.path().join("cache");
        std::fs::write(&file, b"{}").unwrap();
        assert!(cache_read(&file, 60, 2).is_some());
        assert!(cache_read(&file, 0, 2).is_none());
        assert!(cache_read(&file, 60, 1).is_none());
        assert_eq!(publication_year("October 1, 2020"), Some(2020));
        assert!(map_search("book", &json!({"docs":null})).is_err());
        assert!(
            map_search("book", &json!({"docs":[{"key":"invalid","title":"bad"}]}))
                .unwrap()
                .is_empty()
        );
    }
    #[test]
    fn isolated_native_keychain_roundtrip() {
        let service = format!("app.hush.desktop.test.{}", uuid::Uuid::new_v4());
        let entry = keyring::Entry::new(&service, "synthetic-credential").unwrap();
        let value = uuid::Uuid::new_v4().to_string();
        entry.set_password(&value).unwrap();
        let read = entry.get_password();
        let deleted = entry.delete_credential();
        assert!(read.is_ok_and(|v| v == value));
        assert!(deleted.is_ok());
        assert!(matches!(entry.get_password(), Err(keyring::Error::NoEntry)));
    }
    #[test]
    fn retry_after_dates_and_seconds() {
        assert_eq!(retry_after("120"), Some(Duration::from_secs(120)));
        assert_eq!(
            retry_after("Wed, 21 Oct 2015 07:28:00 GMT"),
            Some(Duration::ZERO)
        );
        assert!(retry_after("invalid").is_none());
    }
    #[test]
    fn http_status_retry_and_body_limits() {
        use std::io::{Read, Write};
        fn run(statuses: Vec<u16>, max: usize) -> (Result<Vec<u8>>, usize) {
            let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
            let addr = listener.local_addr().unwrap();
            listener.set_nonblocking(true).unwrap();
            let count = Arc::new(std::sync::atomic::AtomicUsize::new(0));
            let c = count.clone();
            let done = Arc::new(std::sync::atomic::AtomicBool::new(false));
            let d = done.clone();
            let thread = std::thread::spawn(move || {
                let mut statuses = statuses.into_iter();
                while !d.load(std::sync::atomic::Ordering::SeqCst) {
                    if let Ok((mut socket, _)) = listener.accept() {
                        socket
                            .set_read_timeout(Some(Duration::from_secs(2)))
                            .unwrap();
                        let mut buf = [0; 4096];
                        let _ = socket.read(&mut buf);
                        c.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                        let status = statuses.next().unwrap_or(200);
                        if status == 0 {
                            std::thread::sleep(Duration::from_millis(100));
                            continue;
                        }
                        if status == 1 {
                            let _ =
                                socket.write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 2\r\n\r\n");
                            std::thread::sleep(Duration::from_millis(100));
                            continue;
                        }
                        let response=format!("HTTP/1.1 {status} Fixture\r\nContent-Length: 2\r\nRetry-After: 0\r\nConnection: close\r\n\r\n{{}}");
                        let _ = socket.write_all(response.as_bytes());
                    } else {
                        std::thread::sleep(Duration::from_millis(5));
                    }
                }
            });
            let root = tempfile::tempdir().unwrap();
            let catalog = Catalog::new(root.path().into()).unwrap();
            let result = tauri::async_runtime::block_on(
                catalog.body(
                    catalog
                        .client
                        .get(format!("http://{addr}"))
                        .timeout(Duration::from_millis(50)),
                    "book",
                    max,
                ),
            );
            done.store(true, std::sync::atomic::Ordering::SeqCst);
            thread.join().unwrap();
            (result, count.load(std::sync::atomic::Ordering::SeqCst))
        }
        for status in [0, 1] {
            let (result, count) = run(vec![status, status, status], 100);
            assert_eq!(result.unwrap_err().code, "TIMEOUT");
            assert_eq!(count, 3);
        }
        for status in [400, 401, 403, 404, 500] {
            let (result, count) = run(vec![status], 100);
            assert!(result.is_err());
            assert_eq!(count, 1);
        }
        for status in [429, 503] {
            let (result, count) = run(vec![status, status, 200], 100);
            assert!(result.is_ok());
            assert_eq!(count, 3);
        }
        let (result, count) = run(vec![429, 429, 429], 100);
        assert_eq!(result.unwrap_err().code, "RATE_LIMITED");
        assert_eq!(count, 3);
        let (result, count) = run(vec![200], 1);
        assert!(result.is_err());
        assert_eq!(count, 1);
    }
    #[test]
    fn rejects_unsafe_hosts_and_ids() {
        for u in [
            "http://openlibrary.org/a",
            "https://localhost/a",
            "https://archive.org.evil.test/a",
            "https://user@openlibrary.org/a",
            "https://openlibrary.org:444/a",
        ] {
            assert!(!allowed(&url::Url::parse(u).unwrap()));
        }
        assert!(allowed(
            &url::Url::parse("https://ia800.us.archive.org/x").unwrap()
        ));
        assert!(validate_key(&CandidateKey {
            provider: "open_library".into(),
            entity_type: "work".into(),
            external_id: "../../bad".into()
        })
        .is_err());
    }
    #[test]
    fn maps_missing_optional_metadata() {
        let items = map_search(
            "book",
            &json!({"docs":[{"key":"/works/OL1W","title":"İstanbul"}]}),
        )
        .unwrap();
        assert_eq!(items[0].creator, "");
        assert!(!items[0].cover_available);
        assert_eq!(escape_query("a\"b*"), "a\\\"b\\*");
    }
}
