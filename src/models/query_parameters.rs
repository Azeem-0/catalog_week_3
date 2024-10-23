use serde::Deserialize;

#[derive(Deserialize)]
pub struct QueryParameters {
    pub from: Option<f64>,
    pub count: Option<f64>,
    pub interval: Option<String>,
    pub to: Option<f64>,
    pub pool: Option<String>,
}
