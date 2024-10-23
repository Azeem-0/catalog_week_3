use serde::Deserialize;

#[derive(Deserialize)]
pub struct QueryParameters {
    pub from: Option<i64>,
    pub count: Option<i32>,
    pub interval: Option<String>,
    pub to: Option<i64>,
    pub pool: Option<String>,
}
