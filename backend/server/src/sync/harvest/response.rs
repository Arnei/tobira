use chrono::{DateTime, Utc};
use serde::Deserialize;


/// What the harvesting API returns.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct HarvestResponse {
    #[serde(with = "chrono::serde::ts_milliseconds")]
    pub(super) includes_items_until: DateTime<Utc>,
    pub(super) has_more: bool,
    pub(super) items: Vec<HarvestItem>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "kind")]
#[serde(rename_all = "kebab-case")]
pub(super) enum HarvestItem {
    #[serde(rename_all = "camelCase")]
    Event {
        id: String,
        title: String,
        description: Option<String>,
        part_of: Option<String>,
        #[serde(with = "chrono::serde::ts_milliseconds")]
        updated: DateTime<Utc>,
    },

    #[serde(rename_all = "camelCase")]
    EventDeleted {
        id: String,
        #[serde(with = "chrono::serde::ts_milliseconds")]
        updated: DateTime<Utc>,
    },
}

impl HarvestItem {
    pub(super) fn updated(&self) -> DateTime<Utc> {
        match *self {
            Self::Event { updated, .. } => updated,
            Self::EventDeleted { updated, .. } =>  updated,
        }
    }
}
