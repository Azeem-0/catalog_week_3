use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RunePool {
    pub name: String, // name of the pool
    pub start_time: String,
    pub end_time: String,
    pub depth: String,
    pub count: String,
    pub units: String,
}
