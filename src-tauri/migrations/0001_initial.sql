-- Hush v1 reference migration. Run transactionally by the migration runner.
-- Connection PRAGMAs (foreign_keys/WAL/busy_timeout/synchronous) are set outside migrations.
CREATE TABLE assets (
  id TEXT PRIMARY KEY,
  relative_path TEXT NOT NULL UNIQUE,
  thumbnail_path TEXT NOT NULL UNIQUE,
  sha256 TEXT NOT NULL,
  mime_type TEXT NOT NULL CHECK(mime_type = 'image/webp'),
  byte_size INTEGER NOT NULL CHECK(byte_size > 0),
  width INTEGER NOT NULL CHECK(width > 0),
  height INTEGER NOT NULL CHECK(height > 0),
  created_at TEXT NOT NULL
);

CREATE TABLE library_items (
  id TEXT PRIMARY KEY,
  kind TEXT NOT NULL CHECK(kind IN ('game','album','book')),
  title TEXT NOT NULL CHECK(length(trim(title)) BETWEEN 1 AND 300),
  creator TEXT NOT NULL DEFAULT '',
  release_year INTEGER CHECK(release_year BETWEEN 1 AND 9999),
  cover_asset_id TEXT REFERENCES assets(id) ON DELETE SET NULL,
  status TEXT NOT NULL,
  rating INTEGER CHECK(rating BETWEEN 0 AND 10),
  favorite INTEGER NOT NULL DEFAULT 0 CHECK(favorite IN (0,1)),
  notes TEXT NOT NULL DEFAULT '',
  search_text TEXT NOT NULL DEFAULT '',
  started_on TEXT,
  completed_on TEXT,
  revision INTEGER NOT NULL DEFAULT 1 CHECK(revision > 0),
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  deleted_at TEXT,
  UNIQUE(id, kind),
  CHECK(
    (kind IN ('game','book') AND status IN ('planned','active','completed','paused','dropped'))
    OR (kind = 'album' AND status IN ('planned','active','completed'))
  )
);

CREATE TABLE game_details (
  item_id TEXT PRIMARY KEY,
  kind TEXT NOT NULL DEFAULT 'game' CHECK(kind = 'game'),
  platform TEXT NOT NULL DEFAULT '',
  developer TEXT NOT NULL DEFAULT '',
  progress_percent INTEGER CHECK(progress_percent BETWEEN 0 AND 100),
  return_note TEXT NOT NULL DEFAULT '',
  FOREIGN KEY(item_id, kind) REFERENCES library_items(id, kind) ON DELETE CASCADE
);

CREATE TABLE album_details (
  item_id TEXT PRIMARY KEY,
  kind TEXT NOT NULL DEFAULT 'album' CHECK(kind = 'album'),
  first_listened_on TEXT,
  spotify_url TEXT,
  apple_music_url TEXT,
  FOREIGN KEY(item_id, kind) REFERENCES library_items(id, kind) ON DELETE CASCADE
);

CREATE TABLE book_details (
  item_id TEXT PRIMARY KEY,
  kind TEXT NOT NULL DEFAULT 'book' CHECK(kind = 'book'),
  isbn TEXT,
  publisher TEXT NOT NULL DEFAULT '',
  language TEXT NOT NULL DEFAULT '',
  total_pages INTEGER CHECK(total_pages > 0),
  current_page INTEGER NOT NULL DEFAULT 0 CHECK(current_page >= 0),
  CHECK(total_pages IS NULL OR current_page <= total_pages),
  FOREIGN KEY(item_id, kind) REFERENCES library_items(id, kind) ON DELETE CASCADE
);

CREATE TABLE catalog_refs (
  item_id TEXT PRIMARY KEY REFERENCES library_items(id) ON DELETE CASCADE,
  provider TEXT NOT NULL CHECK(provider IN ('open_library','musicbrainz','igdb')),
  entity_type TEXT NOT NULL,
  external_id TEXT NOT NULL,
  edition_id TEXT,
  source_url TEXT NOT NULL,
  fetched_at TEXT NOT NULL,
  attribution TEXT NOT NULL,
  UNIQUE(provider, entity_type, external_id)
);

CREATE TABLE tags (
  id TEXT PRIMARY KEY,
  name TEXT NOT NULL CHECK(length(trim(name)) BETWEEN 1 AND 60),
  normalized_name TEXT NOT NULL UNIQUE
);
CREATE TABLE item_tags (
  item_id TEXT NOT NULL REFERENCES library_items(id) ON DELETE CASCADE,
  tag_id TEXT NOT NULL REFERENCES tags(id) ON DELETE CASCADE,
  PRIMARY KEY(item_id, tag_id)
);

CREATE TABLE collections (
  id TEXT PRIMARY KEY,
  name TEXT NOT NULL CHECK(length(trim(name)) BETWEEN 1 AND 100),
  description TEXT NOT NULL DEFAULT '',
  revision INTEGER NOT NULL DEFAULT 1,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);
CREATE TABLE collection_items (
  collection_id TEXT NOT NULL REFERENCES collections(id) ON DELETE CASCADE,
  item_id TEXT NOT NULL REFERENCES library_items(id) ON DELETE CASCADE,
  sort_order INTEGER NOT NULL DEFAULT 0,
  PRIMARY KEY(collection_id, item_id)
);

CREATE TABLE sessions (
  id TEXT PRIMARY KEY,
  item_id TEXT NOT NULL REFERENCES library_items(id) ON DELETE CASCADE,
  occurred_at TEXT NOT NULL,
  duration_minutes INTEGER NOT NULL CHECK(duration_minutes BETWEEN 1 AND 1440),
  notes TEXT NOT NULL DEFAULT '',
  revision INTEGER NOT NULL DEFAULT 1,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);

CREATE TABLE album_tracks (
  id TEXT PRIMARY KEY,
  album_id TEXT NOT NULL REFERENCES album_details(item_id) ON DELETE CASCADE,
  disc_number INTEGER NOT NULL DEFAULT 1 CHECK(disc_number > 0),
  track_number INTEGER NOT NULL CHECK(track_number > 0),
  title TEXT NOT NULL CHECK(length(trim(title)) BETWEEN 1 AND 300),
  duration_seconds INTEGER CHECK(duration_seconds > 0),
  favorite INTEGER NOT NULL DEFAULT 0 CHECK(favorite IN (0,1)),
  notes TEXT NOT NULL DEFAULT '',
  UNIQUE(album_id, disc_number, track_number)
);

CREATE TABLE book_quotes (
  id TEXT PRIMARY KEY,
  book_id TEXT NOT NULL REFERENCES book_details(item_id) ON DELETE CASCADE,
  body TEXT NOT NULL CHECK(length(trim(body)) BETWEEN 1 AND 10000),
  page_number INTEGER CHECK(page_number > 0),
  comment TEXT NOT NULL DEFAULT '',
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);

CREATE TABLE game_screenshots (
  id TEXT PRIMARY KEY,
  game_id TEXT NOT NULL REFERENCES game_details(item_id) ON DELETE CASCADE,
  asset_id TEXT NOT NULL REFERENCES assets(id) ON DELETE RESTRICT,
  caption TEXT NOT NULL DEFAULT '',
  sort_order INTEGER NOT NULL DEFAULT 0,
  created_at TEXT NOT NULL
);

CREATE TABLE journal_entries (
  id TEXT PRIMARY KEY,
  title TEXT NOT NULL CHECK(length(trim(title)) BETWEEN 1 AND 200),
  entry_date TEXT NOT NULL,
  mood TEXT CHECK(mood IN ('calm','happy','focused','tired','melancholic','excited')),
  body TEXT NOT NULL CHECK(length(body) <= 50000),
  search_text TEXT NOT NULL DEFAULT '',
  revision INTEGER NOT NULL DEFAULT 1,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  deleted_at TEXT
);
CREATE TABLE journal_items (
  journal_id TEXT NOT NULL REFERENCES journal_entries(id) ON DELETE CASCADE,
  item_id TEXT NOT NULL REFERENCES library_items(id) ON DELETE CASCADE,
  PRIMARY KEY(journal_id, item_id)
);

CREATE TABLE drafts (
  draft_key TEXT PRIMARY KEY,
  payload_json TEXT NOT NULL,
  updated_at TEXT NOT NULL
);

CREATE TABLE room_slots (
  slot_id TEXT PRIMARY KEY CHECK(slot_id IN ('poster_left','poster_right','record_stand')),
  item_id TEXT REFERENCES library_items(id) ON DELETE SET NULL,
  asset_id TEXT REFERENCES assets(id) ON DELETE SET NULL,
  CHECK(item_id IS NULL OR asset_id IS NULL)
);
INSERT INTO room_slots(slot_id) VALUES ('poster_left'),('poster_right'),('record_stand');

CREATE TABLE app_settings (
  id INTEGER PRIMARY KEY CHECK(id = 1),
  schema_version INTEGER NOT NULL DEFAULT 1,
  payload_json TEXT NOT NULL,
  revision INTEGER NOT NULL DEFAULT 1,
  updated_at TEXT NOT NULL
);

CREATE INDEX idx_items_list ON library_items(kind, deleted_at, updated_at DESC, id);
CREATE INDEX idx_items_status ON library_items(kind, status, deleted_at);
CREATE INDEX idx_sessions_date ON sessions(occurred_at);
CREATE INDEX idx_sessions_item ON sessions(item_id, occurred_at DESC);
CREATE INDEX idx_journal_date ON journal_entries(deleted_at, entry_date DESC, id);
CREATE INDEX idx_screenshots_game ON game_screenshots(game_id, sort_order);
CREATE INDEX idx_quotes_book ON book_quotes(book_id, created_at);
CREATE INDEX idx_collection_order ON collection_items(collection_id, sort_order);
CREATE INDEX idx_item_tags_tag ON item_tags(tag_id, item_id);
PRAGMA user_version = 1;
