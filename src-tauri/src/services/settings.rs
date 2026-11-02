use crate::domain::*;
use rusqlite::{params, Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Preset {
    CozyNight,
    VioletRain,
    QuietDawn,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Motion {
    System,
    Reduced,
    Full,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RoomSettings {
    pub preset: Preset,
    pub neon_intensity: i32,
    pub lamp_on: bool,
    pub rain_on: bool,
    pub motion: Motion,
    pub labels_visible: bool,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CatalogSettings {
    pub enabled: bool,
    pub contact: Option<String>,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Settings {
    pub schema_version: u32,
    pub locale: String,
    pub room: RoomSettings,
    pub catalog: CatalogSettings,
    pub onboarding_completed: bool,
}
impl Default for Settings {
    fn default() -> Self {
        Self {
            schema_version: 1,
            locale: "tr".into(),
            room: RoomSettings {
                preset: Preset::CozyNight,
                neon_intensity: 60,
                lamp_on: true,
                rain_on: true,
                motion: Motion::System,
                labels_visible: true,
            },
            catalog: CatalogSettings {
                enabled: true,
                contact: None,
            },
            onboarding_completed: false,
        }
    }
}
#[derive(Serialize, Deserialize)]
pub struct Snapshot {
    pub value: Settings,
    pub revision: i64,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SaveSettings {
    pub settings: Settings,
    pub expected_revision: i64,
}
pub fn get(c: &Connection) -> Result<Snapshot> {
    let row: Option<(String, i64)> = c
        .query_row(
            "SELECT payload_json,revision FROM app_settings WHERE id=1",
            [],
            |r| Ok((r.get(0)?, r.get(1)?)),
        )
        .optional()?;
    match row {
        Some((json, revision)) => Ok(Snapshot {
            value: serde_json::from_str(&json).map_err(|_| AppError::new("IO"))?,
            revision,
        }),
        None => {
            let settings = Settings::default();
            let json = serde_json::to_string(&settings).map_err(|_| AppError::new("INTERNAL"))?;
            c.execute(
                "INSERT INTO app_settings(id,payload_json,updated_at) VALUES (1,?,?)",
                params![json, now()],
            )?;
            Ok(Snapshot {
                value: settings,
                revision: 1,
            })
        }
    }
}
pub fn save(c: &Connection, r: SaveSettings) -> Result<Snapshot> {
    if r.settings.schema_version != 1
        || r.settings.locale != "tr"
        || !(0..=100).contains(&r.settings.room.neon_intensity)
        || r.settings
            .catalog
            .contact
            .as_ref()
            .is_some_and(|v| !valid_contact(v))
    {
        return Err(AppError::new("VALIDATION"));
    }
    let json = serde_json::to_string(&r.settings).map_err(|_| AppError::new("INTERNAL"))?;
    if c.execute("UPDATE app_settings SET payload_json=?,revision=revision+1,updated_at=? WHERE id=1 AND revision=?",params![json,now(),r.expected_revision])?!=1{return Err(AppError::new("CONFLICT"));}
    get(c)
}
#[derive(Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum SlotSource {
    Default,
    Item {
        #[serde(rename = "itemId")]
        item_id: String,
    },
    Image {
        #[serde(rename = "assetId")]
        asset_id: String,
    },
}
pub type Slots = std::collections::BTreeMap<String, SlotSource>;
pub fn slots(c: &Connection) -> Result<Slots> {
    let rows = c
        .prepare("SELECT slot_id,item_id,asset_id FROM room_slots")?
        .query_map([], |r| {
            Ok((
                r.get::<_, String>(0)?,
                r.get::<_, Option<String>>(1)?,
                r.get::<_, Option<String>>(2)?,
            ))
        })?
        .collect::<std::result::Result<Vec<_>, _>>()?;
    Ok(rows
        .into_iter()
        .map(|(id, item, asset)| {
            (
                id,
                match (item, asset) {
                    (Some(item_id), _) => SlotSource::Item { item_id },
                    (_, Some(asset_id)) => SlotSource::Image { asset_id },
                    _ => SlotSource::Default,
                },
            )
        })
        .collect())
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SetSlot {
    pub slot_id: String,
    pub item_id: Option<String>,
    pub asset_id: Option<String>,
}
pub fn set_slot(c: &Connection, r: SetSlot) -> Result<Slots> {
    if !matches!(
        r.slot_id.as_str(),
        "poster_left" | "poster_right" | "record_stand"
    ) || r.item_id.is_some() && r.asset_id.is_some()
        || r.slot_id == "record_stand" && r.asset_id.is_some()
    {
        return Err(AppError::new("VALIDATION"));
    }
    if let Some(id) = &r.item_id {
        valid_id(id)?;
        let kind: String = c.query_row(
            "SELECT kind FROM library_items WHERE id=? AND deleted_at IS NULL",
            [id],
            |r| r.get(0),
        )?;
        if r.slot_id == "record_stand" && kind != "album" {
            return Err(AppError::new("VALIDATION"));
        }
    }
    if let Some(id) = &r.asset_id {
        valid_id(id)?;
    }
    c.execute(
        "UPDATE room_slots SET item_id=?,asset_id=? WHERE slot_id=?",
        params![r.item_id, r.asset_id, r.slot_id],
    )?;
    slots(c)
}

fn valid_contact(value: &str) -> bool {
    value.len() <= 300
        && url::Url::parse(value).is_ok_and(|u| {
            u.scheme() == "https"
                && u.host_str().is_some()
                && u.username().is_empty()
                && u.password().is_none()
                && u.port().is_none()
        })
}
#[cfg(test)]
mod contact_tests {
    use super::*;
    #[test]
    fn contact_requires_public_https_address_without_credentials() {
        assert!(valid_contact("https://example.org/contact"));
        for value in [
            "hello",
            "http://example.org",
            "https://secret@example.org",
            "https://example.org:8443",
        ] {
            assert!(!valid_contact(value));
        }
    }
}
