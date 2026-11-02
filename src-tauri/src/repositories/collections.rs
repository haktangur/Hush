use crate::domain::{collection::*, *};
use rusqlite::{params, Connection, OptionalExtension};
pub fn get(c: &Connection, id: &str) -> Result<Collection> {
    let mut result = c.query_row(
        "SELECT id,name,description,revision,created_at,updated_at FROM collections WHERE id=?",
        [id],
        |r| {
            Ok(Collection {
                draft: CollectionDraft {
                    id: r.get(0)?,
                    name: r.get(1)?,
                    description: r.get(2)?,
                },
                revision: r.get(3)?,
                created_at: r.get(4)?,
                updated_at: r.get(5)?,
                item_ids: vec![],
            })
        },
    )?;
    result.item_ids = c
        .prepare("SELECT item_id FROM collection_items WHERE collection_id=? ORDER BY sort_order")?
        .query_map([id], |r| r.get(0))?
        .collect::<std::result::Result<_, _>>()?;
    Ok(result)
}
pub fn list(c: &Connection) -> Result<Vec<Collection>> {
    let ids = c
        .prepare("SELECT id FROM collections ORDER BY name")?
        .query_map([], |r| r.get::<_, String>(0))?
        .collect::<std::result::Result<Vec<_>, _>>()?;
    ids.iter().map(|id| get(c, id)).collect()
}
pub fn save(c: &mut Connection, r: SaveCollection) -> Result<Collection> {
    valid_id(&r.draft.id)?;
    if r.draft.name.trim().is_empty()
        || r.draft.name.chars().count() > 100
        || r.draft.description.chars().count() > 2000
        || r.item_ids.len() > 5000
    {
        return Err(AppError::new("VALIDATION"));
    }
    let tx = c.transaction()?;
    let current: Option<i64> = tx
        .query_row(
            "SELECT revision FROM collections WHERE id=?",
            [&r.draft.id],
            |r| r.get(0),
        )
        .optional()?;
    if current != r.expected_revision {
        return Err(AppError::new("CONFLICT"));
    }
    if let Some(revision) = current {
        tx.execute("UPDATE collections SET name=?,description=?,revision=revision+1,updated_at=? WHERE id=? AND revision=?",params![r.draft.name.trim(),r.draft.description,now(),r.draft.id,revision])?;
    } else {
        tx.execute(
            "INSERT INTO collections(id,name,description,created_at,updated_at) VALUES (?,?,?,?,?)",
            params![
                r.draft.id,
                r.draft.name.trim(),
                r.draft.description,
                now(),
                now()
            ],
        )?;
    }
    tx.execute(
        "DELETE FROM collection_items WHERE collection_id=?",
        [&r.draft.id],
    )?;
    for (index, id) in r.item_ids.iter().enumerate() {
        valid_id(id)?;
        super::library::get(&tx, id, false)?;
        tx.execute(
            "INSERT INTO collection_items(collection_id,item_id,sort_order) VALUES (?,?,?)",
            params![r.draft.id, id, index as i64],
        )?;
    }
    let result = get(&tx, &r.draft.id)?;
    tx.commit()?;
    Ok(result)
}
pub fn delete(c: &Connection, id: &str, revision: i64) -> Result<()> {
    valid_id(id)?;
    if c.execute(
        "DELETE FROM collections WHERE id=? AND revision=?",
        params![id, revision],
    )? != 1
    {
        return Err(AppError::new("CONFLICT"));
    }
    Ok(())
}
pub fn purge(c: &Connection, id: &str) -> Result<()> {
    valid_id(id)?;
    if c.execute(
        "DELETE FROM library_items WHERE id=? AND deleted_at IS NOT NULL",
        [id],
    )? != 1
    {
        return Err(AppError::new("NOT_FOUND"));
    }
    Ok(())
}
