use std::collections::HashSet;

use crate::utils::deserialize_util::deserialize_string_to_number;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RunePoolHistory {
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub start_time: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub end_time: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub count: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub units: f64,
}

impl RunePoolHistory {
    pub fn has_field(field: &str) -> bool {
        let camel_to_snake_fields: HashSet<&str> = vec!["startTime", "endTime", "count", "units"]
            .into_iter()
            .collect();

        camel_to_snake_fields.contains(field)
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RunePoolHistoryMeta {
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub start_time: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub end_time: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub start_units: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub start_count: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub end_units: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub end_count: f64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RunePoolHistoryResponse {
    pub meta: RunePoolHistoryMeta,
    pub intervals: Vec<RunePoolHistory>,
}
