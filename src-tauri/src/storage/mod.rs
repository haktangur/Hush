pub mod assets;
use crate::domain::{AppError, Result};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::{
    path::{Path, PathBuf},
    sync::Mutex,
};
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Pointer {
    format_version: u32,
    generation_id: String,
}
pub struct Storage {
    pub connection: Mutex<Connection>,
    pub generation: PathBuf,
    pub epoch: String,
}
impl Storage {
    pub fn open(root: &Path) -> Result<Self> {
        std::fs::create_dir_all(root)?;
        let pointer_path = root.join("active.json");
        let pointer = if pointer_path.exists() {
            let p: Pointer = serde_json::from_slice(&std::fs::read(&pointer_path)?)
                .map_err(|_| AppError::new("IO"))?;
            crate::domain::valid_id(&p.generation_id)?;
            if p.format_version != 1 {
                return Err(AppError::new("UNSUPPORTED_SCHEMA"));
            }
            if !root
                .join("storage")
                .join(&p.generation_id)
                .join("library.sqlite3")
                .is_file()
            {
                return Err(AppError::new("IO"));
            }
            p
        } else {
            let dir = root.join("storage");
            if dir.exists() && std::fs::read_dir(&dir)?.next().is_some() {
                return Err(AppError::new("IO"));
            }
            let p = Pointer {
                format_version: 1,
                generation_id: uuid::Uuid::new_v4().to_string(),
            };
            let generation = dir.join(&p.generation_id);
            std::fs::create_dir_all(generation.join("assets"))?;
            let mut c = connect(&generation.join("library.sqlite3"))?;
            migrate(&mut c)?;
            drop(c);
            let temp = root.join("active.json.tmp");
            let bytes = serde_json::to_vec(&p).map_err(|_| AppError::new("INTERNAL"))?;
            use std::io::Write;
            let mut f = std::fs::File::create(&temp)?;
            f.write_all(&bytes)?;
            f.sync_all()?;
            std::fs::rename(temp, &pointer_path)?;
            std::fs::File::open(root)?.sync_all()?;
            p
        };
        let generation = root.join("storage").join(&pointer.generation_id);
        let mut connection = connect(&generation.join("library.sqlite3"))?;
        migrate(&mut connection)?;
        Ok(Self {
            connection: Mutex::new(connection),
            generation,
            epoch: pointer.generation_id,
        })
    }
}
pub fn connect(path: &Path) -> Result<Connection> {
    let c = Connection::open(path)?;
    c.execute_batch("PRAGMA foreign_keys=ON; PRAGMA journal_mode=WAL; PRAGMA busy_timeout=5000; PRAGMA synchronous=FULL;")?;
    Ok(c)
}
pub fn migrate(c: &mut Connection) -> Result<()> {
    let version: i32 = c.query_row("PRAGMA user_version", [], |r| r.get(0))?;
    if version > 1 {
        return Err(AppError::new("UNSUPPORTED_SCHEMA"));
    }
    if version == 0 {
        let tx = c.transaction()?;
        tx.execute_batch(include_str!("../../migrations/0001_initial.sql"))?;
        tx.commit()?;
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn persistence_and_missing_pointer_do_not_reset() {
        let d = tempfile::tempdir().unwrap();
        let s = Storage::open(d.path()).unwrap();
        let epoch = s.epoch.clone();
        drop(s);
        assert_eq!(Storage::open(d.path()).unwrap().epoch, epoch);
        std::fs::remove_file(d.path().join("active.json")).unwrap();
        assert!(Storage::open(d.path()).is_err());
        assert!(d
            .path()
            .join("storage")
            .join(epoch)
            .join("library.sqlite3")
            .exists());
    }
    #[test]
    fn future_schema_is_rejected() {
        let mut c = Connection::open_in_memory().unwrap();
        c.pragma_update(None, "user_version", 99).unwrap();
        assert_eq!(migrate(&mut c).unwrap_err().code, "UNSUPPORTED_SCHEMA");
    }
    #[test]
    fn migration_matches_contract() {
        assert_eq!(
            include_str!("../../migrations/0001_initial.sql"),
            include_str!("../../../contracts/0001_initial.sql")
        );
    }
}
