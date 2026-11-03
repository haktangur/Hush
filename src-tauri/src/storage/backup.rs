use super::Storage;
use crate::domain::{now, valid_id, AppError, Result};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeSet,
    fs::{self, File},
    io::{Read, Write},
    path::Path,
    time::{Duration, Instant},
};
use zip::{write::SimpleFileOptions, ZipArchive, ZipWriter};
const TOTAL_LIMIT: u64 = 2 * 1024 * 1024 * 1024;
const DB_LIMIT: u64 = 256 * 1024 * 1024;
fn zip_error(error: zip::result::ZipError) -> AppError {
    match error {
        zip::result::ZipError::Io(error) => error.into(),
        _ => invalid(),
    }
}
// Persist directory entries as well as file contents before committing the pointer.
fn sync_tree(path: &Path) -> Result<()> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            sync_tree(&entry.path())?;
        } else {
            File::open(entry.path())?.sync_all()?;
        }
    }
    File::open(path)?.sync_all()?;
    Ok(())
}
fn invalid() -> AppError {
    AppError::new("INVALID_BACKUP")
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Manifest {
    format_version: u32,
    schema_version: u32,
    app_version: String,
    created_at: String,
    files: Vec<Entry>,
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Entry {
    path: String,
    sha256: String,
    bytes: u64,
}
#[derive(Serialize)]
pub struct Counts {
    items: i64,
    journals: i64,
    assets: i64,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Preview {
    pub inspection_token: String,
    created_at: String,
    app_version: String,
    counts: Counts,
    replaces_current_data: bool,
}
pub struct Inspection {
    pub token: String,
    pub directory: tempfile::TempDir,
    pub created: Instant,
}
fn digest(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}
fn safe_path(p: &str) -> bool {
    if p == "library.sqlite3" {
        return true;
    }
    let parts: Vec<_> = p.split('/').collect();
    parts.len() == 3
        && parts[0] == "assets"
        && valid_id(parts[1]).is_ok()
        && matches!(parts[2], "image.webp" | "thumb.webp")
}
fn read_bounded(path: &Path, limit: u64) -> Result<Vec<u8>> {
    let f = File::open(path)?;
    if f.metadata()?.len() > limit {
        return Err(invalid());
    }
    let mut b = Vec::new();
    f.take(limit + 1).read_to_end(&mut b)?;
    if b.len() as u64 > limit {
        return Err(invalid());
    }
    Ok(b)
}
fn asset_paths(c: &Connection) -> Result<Vec<String>> {
    let mut q = c.prepare("SELECT id,relative_path,thumbnail_path FROM assets ORDER BY id")?;
    let rows = q
        .query_map([], |r| {
            Ok((
                r.get::<_, String>(0)?,
                r.get::<_, String>(1)?,
                r.get::<_, String>(2)?,
            ))
        })?
        .collect::<std::result::Result<Vec<_>, _>>()?;
    let mut out = Vec::new();
    for (id, p, t) in rows {
        valid_id(&id)?;
        if p != format!("assets/{id}/image.webp") || t != format!("assets/{id}/thumb.webp") {
            return Err(invalid());
        }
        out.extend([p, t])
    }
    Ok(out)
}
pub fn export(s: &Storage, target: &Path) -> Result<()> {
    let parent = target.parent().ok_or_else(invalid)?;
    let estimate = {
        let c = s.connection.lock().map_err(|_| AppError::new("INTERNAL"))?;
        let pages: i64 = c.query_row("PRAGMA page_count", [], |r| r.get(0))?;
        let page_size: i64 = c.query_row("PRAGMA page_size", [], |r| r.get(0))?;
        let assets: i64 =
            c.query_row("SELECT coalesce(sum(byte_size),0) FROM assets", [], |r| {
                r.get(0)
            })?;
        pages
            .saturating_mul(page_size)
            .saturating_mul(3)
            .saturating_add(assets.saturating_mul(2))
    };
    check_space(parent, estimate.max(0) as u64)?;
    let stage = tempfile::tempdir_in(parent)?;
    let db = stage.path().join("library.sqlite3");
    {
        let c = s.connection.lock().map_err(|_| AppError::new("INTERNAL"))?;
        crate::services::settings::get(&c)?;
        c.backup(rusqlite::MAIN_DB, &db, rusqlite::backup::NO_PROGRESS)?;
    }
    let c = Connection::open(&db)?;
    // Orphan imports have no durable owner and do not belong in an exported snapshot.
    c.execute("DELETE FROM assets WHERE id NOT IN (SELECT cover_asset_id FROM library_items WHERE cover_asset_id IS NOT NULL UNION SELECT asset_id FROM game_screenshots UNION SELECT asset_id FROM room_slots WHERE asset_id IS NOT NULL)",[])?;
    let paths = asset_paths(&c)?;
    drop(c);
    let mut manifest = Manifest {
        format_version: 1,
        schema_version: 1,
        app_version: env!("CARGO_PKG_VERSION").into(),
        created_at: now(),
        files: Vec::new(),
    };
    let partial = tempfile::NamedTempFile::new_in(parent)?;
    let mut zip = ZipWriter::new(partial.reopen()?);
    let options = SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o600);
    let mut total = 0;
    for p in std::iter::once("library.sqlite3".to_string()).chain(paths) {
        let bytes = read_bounded(
            &if p == "library.sqlite3" {
                db.clone()
            } else {
                s.generation.join(&p)
            },
            if p == "library.sqlite3" {
                DB_LIMIT
            } else {
                20 * 1024 * 1024
            },
        )?;
        total += bytes.len() as u64;
        if total > TOTAL_LIMIT {
            return Err(invalid());
        }
        zip.start_file(&p, options).map_err(|_| invalid())?;
        zip.write_all(&bytes)?;
        manifest.files.push(Entry {
            path: p,
            sha256: digest(&bytes),
            bytes: bytes.len() as u64,
        });
    }
    zip.start_file("manifest.json", options)
        .map_err(|_| invalid())?;
    zip.write_all(&serde_json::to_vec(&manifest).map_err(|_| invalid())?)?;
    zip.finish().map_err(zip_error)?.sync_all()?;
    #[cfg(feature = "e2e")]
    injected_io("export_write")?;
    partial
        .persist(target)
        .map_err(|e| AppError::from(e.error))?;
    File::open(parent)?.sync_all()?;
    Ok(())
}
pub fn inspect(root: &Path, source: &Path) -> Result<(Inspection, Preview)> {
    let stage_root = root.join("restore-staging");
    fs::create_dir_all(&stage_root)?;
    let directory = tempfile::tempdir_in(&stage_root)?;
    let mut zip = ZipArchive::new(File::open(source)?).map_err(|_| invalid())?;
    if zip.len() > 20_000 || zip.has_duplicate_names() {
        return Err(invalid());
    }
    zip.validate_header_consistency().map_err(|_| invalid())?;
    let mut names = BTreeSet::new();
    let mut total = 0u64;
    // Validate every central-directory entry before writing any content.
    for i in 0..zip.len() {
        let e = zip.by_index(i).map_err(|_| invalid())?;
        let name = e.name().map_err(|_| invalid())?;
        if !(name == "manifest.json" || safe_path(&name))
            || !names.insert(name.to_string())
            || e.is_dir()
            || e.unix_mode()
                .is_some_and(|m| m & 0o170000 != 0 && m & 0o170000 != 0o100000)
        {
            return Err(invalid());
        }
        let limit = if name == "library.sqlite3" {
            DB_LIMIT
        } else if name == "manifest.json" {
            8 * 1024 * 1024
        } else {
            20 * 1024 * 1024
        };
        if e.size() > limit {
            return Err(invalid());
        }
        total = total.checked_add(e.size()).ok_or_else(invalid)?;
        if total > TOTAL_LIMIT {
            return Err(invalid());
        }
    }
    #[cfg(feature = "e2e")]
    injected_io("inspect_space")?;
    check_space(directory.path(), total)?;
    if !names.contains("manifest.json") || !names.contains("library.sqlite3") {
        return Err(invalid());
    }
    for i in 0..zip.len() {
        let mut e = zip.by_index(i).map_err(|_| invalid())?;
        let path = directory
            .path()
            .join(e.name().map_err(|_| invalid())?.as_ref());
        fs::create_dir_all(path.parent().ok_or_else(invalid)?)?;
        let mut f = File::create(path)?;
        let expected = e.size();
        let copied = std::io::copy(&mut (&mut e).take(expected + 1), &mut f)?;
        if copied != expected {
            return Err(invalid());
        }
        f.sync_all()?;
    }
    let manifest: Manifest = serde_json::from_slice(&read_bounded(
        &directory.path().join("manifest.json"),
        8 * 1024 * 1024,
    )?)
    .map_err(|_| invalid())?;
    if manifest.format_version != 1 || manifest.schema_version != 1 {
        return Err(AppError::new("UNSUPPORTED_SCHEMA"));
    }
    chrono::DateTime::parse_from_rfc3339(&manifest.created_at).map_err(|_| invalid())?;
    let mut declared = BTreeSet::new();
    declared.insert("manifest.json".into());
    for e in &manifest.files {
        if !safe_path(&e.path) || !names.contains(&e.path) || !declared.insert(e.path.clone()) {
            return Err(invalid());
        }
        let bytes = read_bounded(
            &directory.path().join(&e.path),
            if e.path == "library.sqlite3" {
                DB_LIMIT
            } else {
                20 * 1024 * 1024
            },
        )?;
        if bytes.len() as u64 != e.bytes || digest(&bytes) != e.sha256 {
            return Err(invalid());
        }
    }
    if names != declared {
        return Err(invalid());
    }
    let counts = validate(directory.path()).map_err(|e| {
        if e.code == "UNSUPPORTED_SCHEMA" {
            e
        } else {
            invalid()
        }
    })?;
    fs::remove_file(directory.path().join("manifest.json"))?;
    let token = uuid::Uuid::new_v4().to_string();
    Ok((
        Inspection {
            token: token.clone(),
            directory,
            created: Instant::now(),
        },
        Preview {
            inspection_token: token,
            created_at: manifest.created_at,
            app_version: manifest.app_version,
            counts,
            replaces_current_data: true,
        },
    ))
}
fn check_space(path: &Path, bytes: u64) -> Result<()> {
    let info = rustix::fs::statvfs(path).map_err(|_| AppError::new("IO"))?;
    if info.f_bavail.saturating_mul(info.f_frsize) < bytes.saturating_add(16 * 1024 * 1024) {
        return Err(AppError::new("DISK_FULL"));
    }
    Ok(())
}
fn schema(c: &Connection) -> Result<Vec<(String, String)>> {
    Ok(c.prepare("SELECT name,sql FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%' ORDER BY name")?.query_map([],|r|Ok((r.get(0)?,r.get(1)?)))?.collect::<std::result::Result<Vec<_>,_>>()?)
}
fn validate(path: &Path) -> Result<Counts> {
    let c = Connection::open_with_flags(
        path.join("library.sqlite3"),
        rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY,
    )?;
    c.execute_batch("PRAGMA trusted_schema=OFF; PRAGMA query_only=ON;")?;
    if c.query_row("PRAGMA user_version", [], |r| r.get::<_, u32>(0))? != 1 {
        return Err(AppError::new("UNSUPPORTED_SCHEMA"));
    }
    if c.query_row("PRAGMA integrity_check", [], |r| r.get::<_, String>(0))? != "ok"
        || c.prepare("PRAGMA foreign_key_check")?.exists([])?
    {
        return Err(invalid());
    }
    let mut reference = Connection::open_in_memory()?;
    super::migrate(&mut reference)?;
    if schema(&c)? != schema(&reference)? {
        return Err(invalid());
    }
    // No executable schema objects are part of Hush's v1 database.
    if c.query_row(
        "SELECT count(*) FROM sqlite_master WHERE type IN ('trigger','view')",
        [],
        |r| r.get::<_, i64>(0),
    )? != 0
    {
        return Err(invalid());
    }
    let ids = c
        .prepare("SELECT id FROM library_items")?
        .query_map([], |r| r.get::<_, String>(0))?
        .collect::<std::result::Result<Vec<_>, _>>()?;
    for id in &ids {
        let mut item = crate::repositories::library::get(&c, id, true)?;
        item.item.validate()?;
    }
    let journals = c
        .prepare("SELECT id,deleted_at IS NOT NULL FROM journal_entries")?
        .query_map([], |r| Ok((r.get::<_, String>(0)?, r.get::<_, bool>(1)?)))?
        .collect::<std::result::Result<Vec<_>, _>>()?;
    for (id, trash) in &journals {
        crate::repositories::journal::validate(
            &crate::repositories::journal::get(&c, id, *trash)?.draft,
            false,
        )?;
    }
    if let Some(draft) = crate::repositories::journal::draft_get(&c)? {
        crate::repositories::journal::validate(&draft.entry, true)?;
    }
    let settings = crate::services::settings::get(&c)?;
    crate::services::settings::validate(&settings.value)?;
    let slots = crate::services::settings::slots(&c)?;
    if c.query_row(
        "SELECT count(*) FROM room_slots WHERE slot_id='record_stand' AND asset_id IS NOT NULL",
        [],
        |r| r.get::<_, i64>(0),
    )? != 0
    {
        return Err(invalid());
    }
    if slots.len() != 3 {
        return Err(invalid());
    }
    if c.query_row("SELECT count(*) FROM room_slots s JOIN library_items i ON i.id=s.item_id WHERE s.slot_id='record_stand' AND i.kind!='album'",[],|r|r.get::<_,i64>(0))?!=0{return Err(invalid())}
    let paths = asset_paths(&c)?;
    let mut actual = BTreeSet::new();
    let assets_dir = path.join("assets");
    if assets_dir.exists() {
        for dir in fs::read_dir(&assets_dir)? {
            let dir = dir?;
            if !dir.file_type()?.is_dir() {
                return Err(invalid());
            }
            for file in fs::read_dir(dir.path())? {
                let file = file?;
                if !file.file_type()?.is_file() {
                    return Err(invalid());
                }
                actual.insert(
                    file.path()
                        .strip_prefix(path)
                        .map_err(|_| invalid())?
                        .to_string_lossy()
                        .into_owned(),
                );
            }
        }
    }
    if actual != paths.iter().cloned().collect() {
        return Err(invalid());
    }
    for p in &paths {
        let bytes = read_bounded(&path.join(p), 20 * 1024 * 1024)?;
        let reader = image::ImageReader::new(std::io::Cursor::new(&bytes))
            .with_guessed_format()
            .map_err(|_| invalid())?;
        if reader.format() != Some(image::ImageFormat::WebP) {
            return Err(invalid());
        }
        let decoder = reader.into_decoder().map_err(|_| invalid())?;
        use image::ImageDecoder;
        let (w, h) = decoder.dimensions();
        if w == 0 || h == 0 || u64::from(w) * u64::from(h) > 25_000_000 {
            return Err(invalid());
        }
        if p.ends_with("/image.webp") {
            let (hash, size, width, height): (String, i64, u32, u32) = c.query_row(
                "SELECT sha256,byte_size,width,height FROM assets WHERE relative_path=?",
                [p],
                |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?)),
            )?;
            if digest(&bytes) != hash || bytes.len() as i64 != size || w != width || h != height {
                return Err(invalid());
            }
        }
        image::DynamicImage::from_decoder(decoder).map_err(|_| invalid())?;
    }
    Ok(Counts {
        items: ids.len() as i64,
        journals: journals.len() as i64,
        assets: paths.len() as i64 / 2,
    })
}
pub fn restore(root: &Path, old: &Storage, inspection: Inspection) -> Result<Storage> {
    if inspection.created.elapsed() > Duration::from_secs(15 * 60) {
        return Err(AppError::new("EXPIRED_BACKUP"));
    }
    validate(inspection.directory.path())?;
    {
        let c = old
            .connection
            .lock()
            .map_err(|_| AppError::new("INTERNAL"))?;
        c.execute_batch("PRAGMA wal_checkpoint(TRUNCATE)")?;
    }
    #[cfg(feature = "e2e")]
    injected_io("restore_write")?;
    sync_tree(inspection.directory.path())?;
    let epoch = uuid::Uuid::new_v4().to_string();
    let destination = root.join("storage").join(&epoch);
    #[cfg(feature = "e2e")]
    fault(root, "before_generation");
    fs::rename(inspection.directory.path(), &destination)?;
    File::open(root.join("storage"))?.sync_all()?;
    #[cfg(feature = "e2e")]
    fault(root, "before_pointer");
    if let Err(error) = super::write_pointer(root, &epoch) {
        super::write_pointer(root, &old.epoch)?;
        return Err(error);
    }
    #[cfg(feature = "e2e")]
    fault(root, "after_pointer");
    match Storage::open(root) {
        Ok(s) => Ok(s),
        Err(e) => {
            super::write_pointer(root, &old.epoch)?;
            Err(e)
        }
    }
}
#[cfg(feature = "e2e")]
fn injected_io(point: &str) -> Result<()> {
    if let Some(root) = std::env::var_os("HUSH_TEST_DATA_DIR") {
        if std::fs::read_to_string(Path::new(&root).join("backup-io-fault"))
            .ok()
            .as_deref()
            == Some(point)
        {
            return Err(std::io::Error::from(std::io::ErrorKind::StorageFull).into());
        }
    }
    Ok(())
}

#[cfg(feature = "e2e")]
fn fault(root: &Path, point: &str) {
    if std::fs::read_to_string(root.join("backup-fault"))
        .ok()
        .as_deref()
        == Some(point)
    {
        // Runner waits for this durable marker, then SIGKILLs this exact process.
        let mut marker = File::create(root.join("backup-fault-reached")).unwrap();
        write!(marker, "{}:{point}", std::process::id()).unwrap();
        marker.sync_all().unwrap();
        loop {
            std::thread::park();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn archive_paths_are_strict() {
        for s in [
            "../library.sqlite3",
            "/library.sqlite3",
            "assets/../image.webp",
            "assets\\a\\image.webp",
            "assets/a/image.webp",
            "manifest.json",
        ] {
            assert!(!safe_path(s))
        }
        assert!(safe_path("library.sqlite3"));
    }
    #[test]
    fn snapshot_restore_preserves_draft_and_old_generation() {
        let root = tempfile::tempdir().unwrap();
        let s = Storage::open(root.path()).unwrap();
        {
            let c = s.connection.lock().unwrap();
            crate::services::settings::get(&c).unwrap();
            let draft = crate::domain::journal::SaveJournal {
                entry: crate::domain::journal::JournalDraft {
                    id: uuid::Uuid::new_v4().to_string(),
                    title: "Taslak".into(),
                    entry_date: "2026-11-02".into(),
                    mood: None,
                    body: "Korunmalı".into(),
                    item_ids: vec![],
                },
                expected_revision: None,
            };
            crate::repositories::journal::draft_put(&c, draft).unwrap();
        }
        let archive = root.path().join("test.hushbackup");
        export(&s, &archive).unwrap();
        let (inspection, preview) = inspect(root.path(), &archive).unwrap();
        assert_eq!(preview.counts.items, 0);
        {
            let c = s.connection.lock().unwrap();
            crate::repositories::journal::draft_delete(&c).unwrap();
        }
        let new = restore(root.path(), &s, inspection).unwrap();
        assert_ne!(new.epoch, s.epoch);
        assert!(s.generation.join("library.sqlite3").exists());
        assert_eq!(
            crate::repositories::journal::draft_get(&new.connection.lock().unwrap())
                .unwrap()
                .unwrap()
                .entry
                .body,
            "Korunmalı"
        );
        assert_eq!(Storage::open(root.path()).unwrap().epoch, new.epoch);
    }
    #[test]
    fn expired_or_changed_stage_never_commits() {
        let root = tempfile::tempdir().unwrap();
        let old = Storage::open(root.path()).unwrap();
        let archive = root.path().join("test.hushbackup");
        export(&old, &archive).unwrap();
        let (mut expired, _) = inspect(root.path(), &archive).unwrap();
        expired.created = Instant::now() - Duration::from_secs(901);
        assert!(
            matches!(restore(root.path(), &old, expired), Err(e) if e.code == "EXPIRED_BACKUP")
        );
        let (changed, _) = inspect(root.path(), &archive).unwrap();
        fs::write(changed.directory.path().join("library.sqlite3"), b"broken").unwrap();
        assert!(restore(root.path(), &old, changed).is_err());
        assert_eq!(Storage::open(root.path()).unwrap().epoch, old.epoch);
    }
    #[test]
    fn missing_owned_asset_preserves_existing_export_and_orphan_is_excluded() {
        let root = tempfile::tempdir().unwrap();
        let old = Storage::open(root.path()).unwrap();
        let id = uuid::Uuid::new_v4().to_string();
        let archive = root.path().join("test.hushbackup");
        {
            let c = old.connection.lock().unwrap();
            c.execute(
                "INSERT INTO assets VALUES (?1,?2,?3,'hash','image/webp',1,1,1,?4)",
                rusqlite::params![
                    id,
                    format!("assets/{id}/image.webp"),
                    format!("assets/{id}/thumb.webp"),
                    now()
                ],
            )
            .unwrap();
        }
        export(&old, &archive).unwrap(); // Unowned missing import is deliberately omitted.
        let (stage, preview) = inspect(root.path(), &archive).unwrap();
        assert_eq!(preview.counts.assets, 0);
        drop(stage);
        let original = fs::read(&archive).unwrap();
        old.connection
            .lock()
            .unwrap()
            .execute(
                "UPDATE room_slots SET asset_id=? WHERE slot_id='poster_left'",
                [&id],
            )
            .unwrap();
        assert!(export(&old, &archive).is_err());
        assert_eq!(fs::read(&archive).unwrap(), original);
    }
    #[test]
    fn native_keychain_is_excluded_and_unchanged_by_restore() {
        let service = format!("app.hush.desktop.test.{}", uuid::Uuid::new_v4());
        let credential = keyring::Entry::new(&service, "backup-synthetic").unwrap();
        let secret = uuid::Uuid::new_v4().to_string();
        credential.set_password(&secret).unwrap();
        // Always delete the synthetic credential before assertions, including error paths.
        let result = (|| -> Result<bool> {
            let root = tempfile::tempdir()?;
            let old = Storage::open(root.path())?;
            let archive = root.path().join("test.hushbackup");
            export(&old, &archive)?;
            let mut zip = ZipArchive::new(File::open(&archive)?).map_err(|_| invalid())?;
            for i in 0..zip.len() {
                let mut bytes = Vec::new();
                zip.by_index(i)
                    .map_err(|_| invalid())?
                    .read_to_end(&mut bytes)?;
                if bytes.windows(secret.len()).any(|w| w == secret.as_bytes()) {
                    return Ok(false);
                }
            }
            let (stage, _) = inspect(root.path(), &archive)?;
            let _new = restore(root.path(), &old, stage)?;
            Ok(credential.get_password().is_ok_and(|v| v == secret))
        })();
        let deleted = credential.delete_credential();
        assert!(deleted.is_ok());
        assert!(matches!(result, Ok(true)));
    }
    #[test]
    fn malformed_archive_leaves_active_data_untouched() {
        let root = tempfile::tempdir().unwrap();
        let s = Storage::open(root.path()).unwrap();
        let p = root.path().join("bad.hushbackup");
        fs::write(&p, b"not a zip").unwrap();
        assert!(inspect(root.path(), &p).is_err());
        assert_eq!(Storage::open(root.path()).unwrap().epoch, s.epoch);
    }
}
