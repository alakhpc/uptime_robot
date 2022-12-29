use serde::{de, Deserialize};
use serde_with::{serde_as, DurationSeconds};
use std::time::Duration;

use crate::enums::{MonitorType, Stat, Status};

fn empty_string_as_none<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
where
    T: Deserialize<'de>,
    D: de::Deserializer<'de>,
{
    de::Deserialize::deserialize(deserializer)
        .map(Some)
        .unwrap_or_default()
        .map(Ok)
        .transpose()
}

#[derive(Deserialize, Debug, Clone)]
pub struct GetMonitorsResponse {
    pub stat: Stat,
    pub pagination: Pagination,
    pub monitors: Vec<Monitor>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Pagination {
    pub offset: u64,
    pub limit: u64,
    pub total: u64,
}

#[serde_as]
#[derive(Deserialize, Debug, Clone)]
pub struct Monitor {
    pub id: u64,
    pub friendly_name: String,
    pub url: String,
    #[serde(rename = "type")]
    pub type_: MonitorType,
    #[serde(deserialize_with = "empty_string_as_none")]
    pub sub_type: Option<u8>,
    pub keyword_type: Option<u8>,
    pub keyword_case_type: Option<u8>,
    #[serde(deserialize_with = "empty_string_as_none")]
    pub keyword_value: Option<String>,
    #[serde(deserialize_with = "empty_string_as_none")]
    pub http_username: Option<String>,
    #[serde(deserialize_with = "empty_string_as_none")]
    pub http_password: Option<String>,
    #[serde(deserialize_with = "empty_string_as_none")]
    pub port: Option<u16>,
    #[serde_as(as = "DurationSeconds")]
    pub interval: Duration,
    pub status: Status,
    pub create_datetime: u64,
}
