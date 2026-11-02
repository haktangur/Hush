use super::library::Page;
use crate::domain::{journal::*, *};
use rusqlite::{params, Connection, OptionalExtension};
pub fn get(c: &Connection, id: &str, trash: bool) -> Result<JournalEntry> {
    let mut e=c.query_row("SELECT id,title,entry_date,mood,body,revision,created_at,updated_at,deleted_at FROM journal_entries WHERE id=? AND (? OR deleted_at IS NULL)",params![id,trash],|r|{let mood:Option<String>=r.get(3)?;let mood=mood.map(|m|serde_json::from_value(serde_json::Value::String(m))).transpose().map_err(|_|rusqlite::Error::InvalidQuery)?;Ok(JournalEntry{draft:JournalDraft{id:r.get(0)?,title:r.get(1)?,entry_date:r.get(2)?,mood,body:r.get(4)?,item_ids:vec![]},revision:r.get(5)?,created_at:r.get(6)?,updated_at:r.get(7)?,deleted_at:r.get(8)?})})?;
    e.draft.item_ids = c
        .prepare("SELECT item_id FROM journal_items WHERE journal_id=? ORDER BY item_id")?
        .query_map([id], |r| r.get(0))?
        .collect::<std::result::Result<_, _>>()?;
    Ok(e)
}
pub fn validate(e: &JournalDraft, allow_empty: bool) -> Result<()> {
    valid_id(&e.id)?;
    valid_day(&e.entry_date)?;
    if (!allow_empty && e.title.trim().is_empty())
        || e.title.chars().count() > 200
        || e.body.chars().count() > 50000
        || e.item_ids.len() > 100
    {
        return Err(AppError::new("VALIDATION"));
    }
    let mut seen = std::collections::HashSet::new();
    for id in &e.item_ids {
        valid_id(id)?;
        if !seen.insert(id) {
            return Err(AppError::new("VALIDATION"));
        }
    }
    Ok(())
}
pub fn save(c: &mut Connection, mut r: SaveJournal) -> Result<JournalEntry> {
    r.entry.title = r.entry.title.trim().into();
    validate(&r.entry, false)?;
    let e = &r.entry;
    let tx = c.transaction()?;
    let existing = match get(&tx, &e.id, true) {
        Ok(v) => Some(v),
        Err(x) if x.code == "NOT_FOUND" => None,
        Err(x) => return Err(x),
    };
    if let Some(old) = existing {
        if r.expected_revision.is_none() && old.draft == *e && old.deleted_at.is_none() {
            return Ok(old);
        }
        if r.expected_revision != Some(old.revision) || old.deleted_at.is_some() {
            return Err(AppError::new("CONFLICT"));
        }
    } else if r.expected_revision.is_some() {
        return Err(AppError::new("NOT_FOUND"));
    }
    let mood = e.mood.as_ref().map(|m| {
        serde_json::to_value(m)
            .unwrap()
            .as_str()
            .unwrap()
            .to_owned()
    });
    let search = fold(&format!("{} {}", e.title, e.body));
    let stamp = now();
    if let Some(revision) = r.expected_revision {
        tx.execute("UPDATE journal_entries SET title=?,entry_date=?,mood=?,body=?,search_text=?,revision=revision+1,updated_at=? WHERE id=? AND revision=?",params![e.title,e.entry_date,mood,e.body,search,stamp,e.id,revision])?;
    } else {
        tx.execute("INSERT INTO journal_entries(id,title,entry_date,mood,body,search_text,created_at,updated_at) VALUES (?,?,?,?,?,?,?,?)",params![e.id,e.title,e.entry_date,mood,e.body,search,stamp,stamp])?;
    }
    tx.execute("DELETE FROM journal_items WHERE journal_id=?", [&e.id])?;
    for id in &e.item_ids {
        super::library::get(&tx, id, false)?;
        tx.execute(
            "INSERT INTO journal_items(journal_id,item_id) VALUES (?,?)",
            params![e.id, id],
        )?;
    }
    tx.execute("DELETE FROM drafts WHERE draft_key='journal'", [])?;
    let result = get(&tx, &e.id, false)?;
    tx.commit()?;
    Ok(result)
}
pub fn list(c: &Connection, f: JournalFilter) -> Result<Page<JournalEntry>> {
    if !(1..=100).contains(&f.limit) || f.offset < 0 {
        return Err(AppError::new("VALIDATION"));
    }
    for d in [&f.start, &f.end].into_iter().flatten() {
        valid_day(d)?;
    }
    let q = format!(
        "%{}%",
        fold(f.query.as_deref().unwrap_or(""))
            .replace('\\', "\\\\")
            .replace('%', "\\%")
            .replace('_', "\\_")
    );
    let clause = r"WHERE (deleted_at IS NOT NULL)=?1 AND search_text LIKE ?2 ESCAPE '\' AND (?3 IS NULL OR entry_date>=?3) AND (?4 IS NULL OR entry_date<=?4)";
    let total = c.query_row(
        &format!("SELECT count(*) FROM journal_entries {clause}"),
        params![f.trashed.unwrap_or(false), q, f.start, f.end],
        |r| r.get(0),
    )?;
    let ids=c.prepare(&format!("SELECT id FROM journal_entries {clause} ORDER BY entry_date DESC,created_at DESC LIMIT ?5 OFFSET ?6"))?.query_map(params![f.trashed.unwrap_or(false),q,f.start,f.end,f.limit,f.offset],|r|r.get::<_,String>(0))?.collect::<std::result::Result<Vec<_>,_>>()?;
    Ok(Page {
        items: ids
            .iter()
            .map(|id| get(c, id, f.trashed.unwrap_or(false)))
            .collect::<Result<_>>()?,
        total,
        offset: f.offset,
        limit: f.limit,
    })
}
pub fn draft_get(c: &Connection) -> Result<Option<SaveJournal>> {
    let json: Option<String> = c
        .query_row(
            "SELECT payload_json FROM drafts WHERE draft_key='journal'",
            [],
            |r| r.get(0),
        )
        .optional()?;
    json.map(|s| serde_json::from_str(&s).map_err(|_| AppError::new("IO")))
        .transpose()
}
pub fn draft_put(c: &Connection, draft: SaveJournal) -> Result<()> {
    validate(&draft.entry, true)?;
    let json = serde_json::to_string(&draft).map_err(|_| AppError::new("INTERNAL"))?;
    c.execute("INSERT INTO drafts(draft_key,payload_json,updated_at) VALUES ('journal',?,?) ON CONFLICT(draft_key) DO UPDATE SET payload_json=excluded.payload_json,updated_at=excluded.updated_at",params![json,now()])?;
    Ok(())
}
pub fn draft_delete(c: &Connection) -> Result<()> {
    c.execute("DELETE FROM drafts WHERE draft_key='journal'", [])?;
    Ok(())
}
pub fn trash(c: &Connection, id: &str, revision: i64, restore: bool) -> Result<()> {
    valid_id(id)?;
    if c.execute("UPDATE journal_entries SET deleted_at=?,updated_at=?,revision=revision+1 WHERE id=? AND revision=? AND (deleted_at IS NOT NULL)=?",params![if restore{None}else{Some(now())},now(),id,revision,restore])?!=1{return Err(AppError::new("CONFLICT"));}
    Ok(())
}
pub fn calendar(c: &Connection, r: CalendarRequest) -> Result<CalendarMonth> {
    use chrono::{Datelike, TimeZone};
    let tz: r#chrono_tz::Tz = r
        .time_zone
        .parse()
        .map_err(|_| AppError::new("VALIDATION"))?;
    let start =
        chrono::NaiveDate::from_ymd_opt(r.year, r.month, 1).ok_or(AppError::new("VALIDATION"))?;
    let end = if r.month == 12 {
        chrono::NaiveDate::from_ymd_opt(r.year + 1, 1, 1)
    } else {
        chrono::NaiveDate::from_ymd_opt(r.year, r.month + 1, 1)
    }
    .ok_or(AppError::new("VALIDATION"))?;
    let from = tz
        .from_local_datetime(&start.and_hms_opt(0, 0, 0).unwrap())
        .earliest()
        .ok_or(AppError::new("VALIDATION"))?
        .with_timezone(&chrono::Utc)
        .to_rfc3339_opts(chrono::SecondsFormat::Millis, true);
    let until = tz
        .from_local_datetime(&end.and_hms_opt(0, 0, 0).unwrap())
        .earliest()
        .ok_or(AppError::new("VALIDATION"))?
        .with_timezone(&chrono::Utc)
        .to_rfc3339_opts(chrono::SecondsFormat::Millis, true);
    let mut events=c.prepare("SELECT id,title,entry_date FROM journal_entries WHERE deleted_at IS NULL AND entry_date>=? AND entry_date<?")?.query_map(params![start.to_string(),end.to_string()],|row|Ok(CalendarEvent{id:row.get(0)?,title:row.get(1)?,day:row.get(2)?,kind:"journal".into(),duration_minutes:None}))?.collect::<std::result::Result<Vec<_>,_>>()?;
    let journal_days = events
        .iter()
        .map(|e| e.day.clone())
        .collect::<std::collections::HashSet<_>>()
        .len();
    let mut game_minutes = 0;
    let mut book_minutes = 0;
    let mut album_minutes = 0;
    let sessions=c.prepare("SELECT s.id,i.title,s.occurred_at,i.kind,s.duration_minutes FROM sessions s JOIN library_items i ON s.item_id=i.id WHERE i.deleted_at IS NULL AND s.occurred_at>=? AND s.occurred_at<?")?.query_map(params![from,until],|r|Ok((r.get::<_,String>(0)?,r.get::<_,String>(1)?,r.get::<_,String>(2)?,r.get::<_,String>(3)?,r.get::<_,i32>(4)?)))?.collect::<std::result::Result<Vec<_>,_>>()?;
    for (id, title, date, kind, minutes) in sessions {
        let local = chrono::DateTime::parse_from_rfc3339(&date)
            .map_err(|_| AppError::new("IO"))?
            .with_timezone(&tz);
        if local.year() != r.year || local.month() != r.month {
            continue;
        }
        match kind.as_str() {
            "game" => game_minutes += i64::from(minutes),
            "book" => book_minutes += i64::from(minutes),
            "album" => album_minutes += i64::from(minutes),
            _ => (),
        };
        events.push(CalendarEvent {
            id,
            title,
            day: local.date_naive().to_string(),
            kind,
            duration_minutes: Some(minutes),
        })
    }
    let added_items=c.query_row("SELECT count(*) FROM library_items WHERE deleted_at IS NULL AND created_at>=? AND created_at<?",params![from,until],|r|r.get(0))?;
    events.sort_by(|a, b| a.day.cmp(&b.day));
    Ok(CalendarMonth {
        events,
        added_items,
        journal_days,
        game_minutes,
        book_minutes,
        album_minutes,
    })
}
