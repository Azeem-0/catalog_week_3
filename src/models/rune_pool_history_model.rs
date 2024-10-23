use crate::utils::deserialize_util::deserialize_string_to_number;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RunePoolHistory {
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub start_time: i64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub end_time: i64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub depth: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub count: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub units: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RunePoolHistoryMeta {
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub start_time: i64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub end_time: i64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub start_units: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub start_count: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub end_units: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub end_count: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RunePoolHistoryResponse {
    pub meta: RunePoolHistoryMeta,
    pub intervals: Vec<RunePoolHistory>,
}
