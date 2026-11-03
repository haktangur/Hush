use crate::domain::*;
use rusqlite::{params, Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LibraryFilter {
    pub kind: Option<String>,
    pub query: Option<String>,
    pub status: Option<String>,
    pub favorite: Option<bool>,
    pub sort: String,
    pub offset: i64,
    pub limit: i64,
    pub trashed: Option<bool>,
    pub collection_id: Option<String>,
    pub tags: Option<Vec<String>>,
}
#[derive(Serialize)]
pub struct Page<T> {
    pub items: Vec<T>,
    pub total: i64,
    pub offset: i64,
    pub limit: i64,
}
pub fn get(c: &Connection, id: &str, include_trash: bool) -> Result<LibraryItem> {
    let (mut item,revision,created_at,updated_at,deleted_at)=c.query_row("SELECT id,kind,title,creator,release_year,cover_asset_id,status,rating,favorite,notes,started_on,completed_on,revision,created_at,updated_at,deleted_at FROM library_items WHERE id=?1 AND (?2 OR deleted_at IS NULL)",params![id,include_trash],|r|{
 let kind:String=r.get(1)?;let status:String=r.get(6)?;
 let status:Status=serde_json::from_value(serde_json::Value::String(status)).map_err(|_|rusqlite::Error::InvalidQuery)?;
 let content=match kind.as_str(){"game"=>ItemContent::Game{status,details:GameDetails{platform:String::new(),developer:String::new(),progress_percent:None,return_note:String::new()}},"book"=>ItemContent::Book{status,details:BookDetails{isbn:None,publisher:String::new(),language:String::new(),total_pages:None,current_page:0}},"album"=>ItemContent::Album{status:match status{Status::Planned=>AlbumStatus::Planned,Status::Active=>AlbumStatus::Active,Status::Completed=>AlbumStatus::Completed,_=>return Err(rusqlite::Error::InvalidQuery)},details:AlbumDetails{first_listened_on:None,spotify_url:None,apple_music_url:None}},_=>return Err(rusqlite::Error::InvalidQuery)};
 Ok((ItemDraft{id:r.get(0)?,title:r.get(2)?,creator:r.get(3)?,release_year:r.get(4)?,cover_asset_id:r.get(5)?,rating:r.get(7)?,favorite:r.get(8)?,notes:r.get(9)?,started_on:r.get(10)?,completed_on:r.get(11)?,content,tag_names:vec![],provenance:None},r.get(12)?,r.get(13)?,r.get(14)?,r.get(15)?))})?;
    match &mut item.content {
 ItemContent::Book{details,..}=>*details=c.query_row("SELECT isbn,publisher,language,total_pages,current_page FROM book_details WHERE item_id=?",[id],|r|Ok(BookDetails{isbn:r.get(0)?,publisher:r.get(1)?,language:r.get(2)?,total_pages:r.get(3)?,current_page:r.get(4)?}))?,
 ItemContent::Game{details,..}=>*details=c.query_row("SELECT platform,developer,progress_percent,return_note FROM game_details WHERE item_id=?",[id],|r|Ok(GameDetails{platform:r.get(0)?,developer:r.get(1)?,progress_percent:r.get(2)?,return_note:r.get(3)?}))?,
 ItemContent::Album{details,..}=>*details=c.query_row("SELECT first_listened_on,spotify_url,apple_music_url FROM album_details WHERE item_id=?",[id],|r|Ok(AlbumDetails{first_listened_on:r.get(0)?,spotify_url:r.get(1)?,apple_music_url:r.get(2)?}))?
 }
    item.tag_names=c.prepare("SELECT t.name FROM tags t JOIN item_tags it ON it.tag_id=t.id WHERE it.item_id=? ORDER BY t.normalized_name")?.query_map([id],|r|r.get(0))?.collect::<std::result::Result<_,_>>()?;
    item.provenance=c.query_row("SELECT provider,entity_type,external_id,edition_id,source_url,fetched_at,attribution FROM catalog_refs WHERE item_id=?",[id],|r|Ok(Provenance{provider:r.get(0)?,entity_type:r.get(1)?,external_id:r.get(2)?,edition_id:r.get(3)?,source_url:r.get(4)?,fetched_at:r.get(5)?,attribution:r.get(6)?})).optional()?;
    Ok(LibraryItem {
        item,
        revision,
        created_at,
        updated_at,
        deleted_at,
    })
}
pub fn save(
    c: &mut Connection,
    item: &ItemDraft,
    revision: Option<i64>,
    tracks: Option<&[crate::domain::activity::Track]>,
) -> Result<LibraryItem> {
    if tracks.is_some() && (revision.is_some() || item.kind() != "album") {
        return Err(AppError::new("VALIDATION"));
    }
    if let Some(tracks) = tracks {
        if tracks.len() > 1000 {
            return Err(AppError::new("VALIDATION"));
        }
        let mut ids = std::collections::HashSet::new();
        let mut positions = std::collections::HashSet::new();
        for t in tracks {
            valid_id(&t.id)?;
            if !ids.insert(&t.id)
                || !positions.insert((t.disc_number, t.track_number))
                || t.disc_number < 1
                || t.track_number < 1
                || t.title.trim().is_empty()
                || t.title.chars().count() > 300
                || t.notes.chars().count() > 2000
                || t.duration_seconds.is_some_and(|n| n < 1)
            {
                return Err(AppError::new("VALIDATION"));
            }
        }
    }
    let tx = c.transaction()?;
    let previous = match get(&tx, &item.id, true) {
        Ok(v) => Some(v),
        Err(e) if e.code == "NOT_FOUND" => None,
        Err(e) => return Err(e),
    };
    if let Some(p) = &previous {
        if revision.is_none() {
            if p.item == *item && p.deleted_at.is_none() {
                if let Some(tracks) = tracks {
                    if super::activity::extras(&tx, &item.id)?.tracks != tracks {
                        return Err(AppError::new("CONFLICT"));
                    }
                }
                return Ok(p.clone());
            }
            return Err(AppError::new("CONFLICT"));
        }
        if Some(p.revision) != revision || p.item.kind() != item.kind() || p.deleted_at.is_some() {
            return Err(AppError::new("CONFLICT"));
        }
    } else if revision.is_some() {
        return Err(AppError::new("NOT_FOUND"));
    }
    let timestamp = now();
    let search = fold(&format!(
        "{} {} {} {}",
        item.title,
        item.creator,
        item.notes,
        item.tag_names.join(" ")
    ));
    if previous.is_some() {
        let affected=tx.execute("UPDATE library_items SET title=?1,creator=?2,release_year=?3,cover_asset_id=?4,status=?5,rating=?6,favorite=?7,notes=?8,search_text=?9,started_on=?10,completed_on=?11,revision=revision+1,updated_at=?12 WHERE id=?13 AND revision=?14",params![item.title,item.creator,item.release_year,item.cover_asset_id,item.status(),item.rating,item.favorite,item.notes,search,item.started_on,item.completed_on,timestamp,item.id,revision])?;
        if affected != 1 {
            return Err(AppError::new("CONFLICT"));
        }
    } else {
        tx.execute("INSERT INTO library_items(id,kind,title,creator,release_year,cover_asset_id,status,rating,favorite,notes,search_text,started_on,completed_on,created_at,updated_at) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?14)",params![item.id,item.kind(),item.title,item.creator,item.release_year,item.cover_asset_id,item.status(),item.rating,item.favorite,item.notes,search,item.started_on,item.completed_on,timestamp])?;
    }
    match &item.content {
        ItemContent::Book { details: d, .. } => {
            tx.execute("INSERT INTO book_details(item_id,isbn,publisher,language,total_pages,current_page) VALUES (?1,?2,?3,?4,?5,?6) ON CONFLICT(item_id) DO UPDATE SET isbn=excluded.isbn,publisher=excluded.publisher,language=excluded.language,total_pages=excluded.total_pages,current_page=excluded.current_page",params![item.id,d.isbn,d.publisher,d.language,d.total_pages,d.current_page])?;
        }
        ItemContent::Game { details: d, .. } => {
            tx.execute("INSERT INTO game_details(item_id,platform,developer,progress_percent,return_note) VALUES (?1,?2,?3,?4,?5) ON CONFLICT(item_id) DO UPDATE SET platform=excluded.platform,developer=excluded.developer,progress_percent=excluded.progress_percent,return_note=excluded.return_note",params![item.id,d.platform,d.developer,d.progress_percent,d.return_note])?;
        }
        ItemContent::Album { details: d, .. } => {
            tx.execute("INSERT INTO album_details(item_id,first_listened_on,spotify_url,apple_music_url) VALUES (?1,?2,?3,?4) ON CONFLICT(item_id) DO UPDATE SET first_listened_on=excluded.first_listened_on,spotify_url=excluded.spotify_url,apple_music_url=excluded.apple_music_url",params![item.id,d.first_listened_on,d.spotify_url,d.apple_music_url])?;
        }
    }
    tx.execute("DELETE FROM item_tags WHERE item_id=?", [&item.id])?;
    for name in &item.tag_names {
        let normalized = fold(name);
        tx.execute("INSERT INTO tags(id,name,normalized_name) VALUES (?,?,?) ON CONFLICT(normalized_name) DO NOTHING",params![uuid::Uuid::new_v4().to_string(),name,normalized])?;
        tx.execute(
            "INSERT INTO item_tags(item_id,tag_id) SELECT ?,id FROM tags WHERE normalized_name=?",
            params![item.id, normalized],
        )?;
    }
    tx.execute("DELETE FROM catalog_refs WHERE item_id=?", [&item.id])?;
    if let Some(p) = &item.provenance {
        tx.execute("INSERT INTO catalog_refs(item_id,provider,entity_type,external_id,edition_id,source_url,fetched_at,attribution) VALUES (?,?,?,?,?,?,?,?)",params![item.id,p.provider,p.entity_type,p.external_id,p.edition_id,p.source_url,p.fetched_at,p.attribution])?;
    }
    if let Some(tracks) = tracks {
        for t in tracks {
            tx.execute("INSERT INTO album_tracks(id,album_id,disc_number,track_number,title,duration_seconds,favorite,notes) VALUES (?,?,?,?,?,?,?,?)",params![t.id,item.id,t.disc_number,t.track_number,t.title,t.duration_seconds,t.favorite,t.notes])?;
        }
    }
    let result = get(&tx, &item.id, false)?;
    tx.commit()?;
    Ok(result)
}
pub fn list(c: &Connection, f: &LibraryFilter) -> Result<Page<LibraryItem>> {
    if !(1..=100).contains(&f.limit)
        || f.offset < 0
        || f.kind
            .as_ref()
            .is_some_and(|k| !matches!(k.as_str(), "game" | "book" | "album"))
    {
        return Err(AppError::new("VALIDATION"));
    }
    let order = match f.sort.as_str() {
        "title_asc" => "title COLLATE NOCASE ASC",
        "created_desc" => "created_at DESC",
        "rating_desc" => "rating DESC",
        "updated_desc" => "updated_at DESC",
        _ => return Err(AppError::new("VALIDATION")),
    };
    let query = format!(
        "%{}%",
        fold(f.query.as_deref().unwrap_or(""))
            .replace('\\', "\\\\")
            .replace('%', "\\%")
            .replace('_', "\\_")
    );
    let tags = serde_json::to_string(
        &f.tags
            .clone()
            .unwrap_or_default()
            .iter()
            .map(|v| fold(v))
            .collect::<Vec<_>>(),
    )
    .map_err(|_| AppError::new("INTERNAL"))?;
    let clause = r"WHERE (deleted_at IS NOT NULL)=?1 AND (?2 IS NULL OR kind=?2) AND search_text LIKE ?3 ESCAPE '\' AND (?4 IS NULL OR status=?4) AND (?5 IS NULL OR favorite=?5) AND (?6 IS NULL OR id IN (SELECT item_id FROM collection_items WHERE collection_id=?6)) AND NOT EXISTS (SELECT 1 FROM json_each(?7) wanted WHERE NOT EXISTS (SELECT 1 FROM item_tags it JOIN tags t ON t.id=it.tag_id WHERE it.item_id=library_items.id AND t.normalized_name=wanted.value))";
    let args = params![
        f.trashed.unwrap_or(false),
        f.kind,
        query,
        f.status,
        f.favorite,
        f.collection_id,
        tags
    ];
    let total = c.query_row(
        &format!("SELECT COUNT(*) FROM library_items {clause}"),
        args,
        |r| r.get(0),
    )?;
    let ids: Vec<String> = c
        .prepare(&format!(
            "SELECT id FROM library_items {clause} ORDER BY {order},id LIMIT ?8 OFFSET ?9"
        ))?
        .query_map(
            params![
                f.trashed.unwrap_or(false),
                f.kind,
                query,
                f.status,
                f.favorite,
                f.collection_id,
                tags,
                f.limit,
                f.offset
            ],
            |r| r.get(0),
        )?
        .collect::<std::result::Result<_, _>>()?;
    let items = ids
        .iter()
        .map(|id| get(c, id, f.trashed.unwrap_or(false)))
        .collect::<Result<_>>()?;
    Ok(Page {
        items,
        total,
        offset: f.offset,
        limit: f.limit,
    })
}
pub fn trash(c: &Connection, id: &str, revision: i64, restore: bool) -> Result<()> {
    valid_id(id)?;
    let affected=c.execute("UPDATE library_items SET deleted_at=?1,updated_at=?2,revision=revision+1 WHERE id=?3 AND revision=?4 AND (deleted_at IS NOT NULL)=?5",params![if restore{None}else{Some(now())},now(),id,revision,restore])?;
    if affected != 1 {
        return Err(AppError::new("CONFLICT"));
    }
    Ok(())
}

pub fn catalog_existing(
    c: &Connection,
    provider: &str,
    entity: &str,
    external_id: &str,
) -> Result<Option<(String, bool)>> {
    Ok(c.query_row("SELECT i.id,i.deleted_at IS NOT NULL FROM library_items i JOIN catalog_refs r ON r.item_id=i.id WHERE r.provider=? AND r.entity_type=? AND r.external_id=?",params![provider,entity,external_id],|r|Ok((r.get(0)?,r.get(1)?))).optional()?)
}
