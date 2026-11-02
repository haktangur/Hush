use crate::domain::{activity::*, *};
use rusqlite::{params, Connection, OptionalExtension};
pub fn extras(c: &Connection, id: &str) -> Result<Extras> {
    super::library::get(c, id, false)?;
    let sessions=c.prepare("SELECT id,item_id,occurred_at,duration_minutes,notes,revision,created_at,updated_at FROM sessions WHERE item_id=? ORDER BY occurred_at DESC")?.query_map([id],|r|Ok(Session{draft:SessionDraft{id:r.get(0)?,item_id:r.get(1)?,occurred_at:r.get(2)?,duration_minutes:r.get(3)?,notes:r.get(4)?},revision:r.get(5)?,created_at:r.get(6)?,updated_at:r.get(7)?}))?.collect::<std::result::Result<Vec<_>,_>>()?;
    let total_minutes = sessions
        .iter()
        .map(|s| i64::from(s.draft.duration_minutes))
        .sum();
    let tracks=c.prepare("SELECT id,disc_number,track_number,title,duration_seconds,favorite,notes FROM album_tracks WHERE album_id=? ORDER BY disc_number,track_number")?.query_map([id],|r|Ok(Track{id:r.get(0)?,disc_number:r.get(1)?,track_number:r.get(2)?,title:r.get(3)?,duration_seconds:r.get(4)?,favorite:r.get(5)?,notes:r.get(6)?}))?.collect::<std::result::Result<_,_>>()?;
    let quotes=c.prepare("SELECT id,body,page_number,comment,created_at,updated_at FROM book_quotes WHERE book_id=? ORDER BY created_at")?.query_map([id],|r|Ok(Quote{id:r.get(0)?,body:r.get(1)?,page_number:r.get(2)?,comment:r.get(3)?,created_at:r.get(4)?,updated_at:r.get(5)?}))?.collect::<std::result::Result<_,_>>()?;
    let screenshots=c.prepare("SELECT id,asset_id,caption,sort_order,created_at FROM game_screenshots WHERE game_id=? ORDER BY sort_order")?.query_map([id],|r|Ok(Screenshot{id:r.get(0)?,asset_id:r.get(1)?,caption:r.get(2)?,sort_order:r.get(3)?,created_at:r.get(4)?}))?.collect::<std::result::Result<_,_>>()?;
    Ok(Extras {
        sessions,
        total_minutes,
        tracks,
        quotes,
        screenshots,
    })
}
pub fn save_session(c: &mut Connection, r: SaveSession) -> Result<Session> {
    let mut d = r.draft;
    valid_id(&d.id)?;
    valid_id(&d.item_id)?;
    chrono::DateTime::parse_from_rfc3339(&d.occurred_at)
        .map_err(|_| AppError::new("VALIDATION"))?;
    if !(1..=1440).contains(&d.duration_minutes) || d.notes.chars().count() > 2000 {
        return Err(AppError::new("VALIDATION"));
    }
    d.occurred_at = chrono::DateTime::parse_from_rfc3339(&d.occurred_at)
        .map_err(|_| AppError::new("VALIDATION"))?
        .with_timezone(&chrono::Utc)
        .to_rfc3339_opts(chrono::SecondsFormat::Millis, true);
    let tx = c.transaction()?;
    super::library::get(&tx, &d.item_id, false)?;
    let previous: Option<(String, i64)> = tx
        .query_row(
            "SELECT item_id,revision FROM sessions WHERE id=?",
            [&d.id],
            |r| Ok((r.get(0)?, r.get(1)?)),
        )
        .optional()?;
    let timestamp = now();
    match previous {
        Some((parent, revision)) => {
            if parent != d.item_id || r.expected_revision != Some(revision) {
                return Err(AppError::new("CONFLICT"));
            }
            tx.execute("UPDATE sessions SET occurred_at=?,duration_minutes=?,notes=?,revision=revision+1,updated_at=? WHERE id=? AND revision=?",params![d.occurred_at,d.duration_minutes,d.notes,timestamp,d.id,revision])?;
        }
        None => {
            if r.expected_revision.is_some() {
                return Err(AppError::new("NOT_FOUND"));
            }
            tx.execute("INSERT INTO sessions(id,item_id,occurred_at,duration_minutes,notes,created_at,updated_at) VALUES (?,?,?,?,?,?,?)",params![d.id,d.item_id,d.occurred_at,d.duration_minutes,d.notes,timestamp,timestamp])?;
        }
    }
    let result = tx.query_row(
        "SELECT revision,created_at,updated_at FROM sessions WHERE id=?",
        [&d.id],
        |r| {
            Ok(Session {
                draft: d.clone(),
                revision: r.get(0)?,
                created_at: r.get(1)?,
                updated_at: r.get(2)?,
            })
        },
    )?;
    tx.commit()?;
    Ok(result)
}
pub fn delete_session(c: &Connection, id: &str, revision: i64) -> Result<()> {
    valid_id(id)?;
    if c.execute("DELETE FROM sessions WHERE id=? AND revision=? AND item_id IN (SELECT id FROM library_items WHERE deleted_at IS NULL)",params![id,revision])?!=1{return Err(AppError::new("CONFLICT"));}
    Ok(())
}
pub fn replace(c: &mut Connection, r: ReplaceChildren) -> Result<LibraryItem> {
    valid_id(&r.item_id)?;
    let tx = c.transaction()?;
    let item = super::library::get(&tx, &r.item_id, false)?;
    if item.revision != r.expected_revision {
        return Err(AppError::new("CONFLICT"));
    }
    let invalid = || AppError::new("VALIDATION");
    let mut ids = std::collections::HashSet::new();
    match r.children {
        Children::Album { tracks } => {
            if item.item.kind() != "album" || tracks.len() > 1000 {
                return Err(invalid());
            }
            let mut positions = std::collections::HashSet::new();
            for t in &tracks {
                valid_id(&t.id)?;
                if !ids.insert(&t.id)
                    || !positions.insert((t.disc_number, t.track_number))
                    || t.disc_number < 1
                    || t.track_number < 1
                    || t.title.trim().is_empty()
                    || t.title.chars().count() > 300
                    || t.notes.chars().count() > 2000
                    || t.duration_seconds.is_some_and(|n| n <= 0)
                {
                    return Err(invalid());
                }
            }
            tx.execute("DELETE FROM album_tracks WHERE album_id=?", [&r.item_id])?;
            for t in tracks {
                tx.execute("INSERT INTO album_tracks(id,album_id,disc_number,track_number,title,duration_seconds,favorite,notes) VALUES (?,?,?,?,?,?,?,?)",params![t.id,r.item_id,t.disc_number,t.track_number,t.title,t.duration_seconds,t.favorite,t.notes])?;
            }
        }
        Children::Book { quotes } => {
            if item.item.kind() != "book" || quotes.len() > 1000 {
                return Err(invalid());
            }
            for q in &quotes {
                valid_id(&q.id)?;
                if !ids.insert(&q.id)
                    || q.body.trim().is_empty()
                    || q.body.chars().count() > 10000
                    || q.comment.chars().count() > 2000
                    || q.page_number.is_some_and(|n| n < 1)
                {
                    return Err(invalid());
                }
                chrono::DateTime::parse_from_rfc3339(&q.created_at).map_err(|_| invalid())?;
                chrono::DateTime::parse_from_rfc3339(&q.updated_at).map_err(|_| invalid())?;
            }
            tx.execute("DELETE FROM book_quotes WHERE book_id=?", [&r.item_id])?;
            for q in quotes {
                tx.execute("INSERT INTO book_quotes(id,book_id,body,page_number,comment,created_at,updated_at) VALUES (?,?,?,?,?,?,?)",params![q.id,r.item_id,q.body,q.page_number,q.comment,q.created_at,q.updated_at])?;
            }
        }
        Children::Game { screenshots } => {
            if item.item.kind() != "game" || screenshots.len() > 1000 {
                return Err(invalid());
            }
            for s in &screenshots {
                valid_id(&s.id)?;
                valid_id(&s.asset_id)?;
                if !ids.insert(&s.id) || s.caption.chars().count() > 2000 || s.sort_order < 0 {
                    return Err(invalid());
                }
                chrono::DateTime::parse_from_rfc3339(&s.created_at).map_err(|_| invalid())?;
            }
            tx.execute("DELETE FROM game_screenshots WHERE game_id=?", [&r.item_id])?;
            for s in screenshots {
                tx.execute("INSERT INTO game_screenshots(id,game_id,asset_id,caption,sort_order,created_at) VALUES (?,?,?,?,?,?)",params![s.id,r.item_id,s.asset_id,s.caption,s.sort_order,s.created_at])?;
            }
        }
    }
    tx.execute(
        "UPDATE library_items SET revision=revision+1,updated_at=? WHERE id=? AND revision=?",
        params![now(), r.item_id, r.expected_revision],
    )?;
    let result = super::library::get(&tx, &r.item_id, false)?;
    tx.commit()?;
    Ok(result)
}
