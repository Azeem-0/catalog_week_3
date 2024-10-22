use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RunePoolHistory {
    pub start_time: String,
    pub end_time: String,
    pub depth: String,
    pub count: String,
    pub units: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RunePoolHistoryMeta {
    pub start_time: String,
    pub end_time: String,
    pub start_units: String,
    pub start_count: String,
    pub end_units: String,
    pub end_count: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RunePoolHistoryResponse {
    pub meta: RunePoolHistoryMeta,
    pub intervals: Vec<RunePoolHistory>,
}
