use serde::{Deserialize, Serialize};
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CollectionDraft {
    pub id: String,
    pub name: String,
    pub description: String,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Collection {
    #[serde(flatten)]
    pub draft: CollectionDraft,
    pub revision: i64,
    pub created_at: String,
    pub updated_at: String,
    pub item_ids: Vec<String>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SaveCollection {
    pub draft: CollectionDraft,
    pub expected_revision: Option<i64>,
    pub item_ids: Vec<String>,
}
