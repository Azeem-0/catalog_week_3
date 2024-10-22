use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DepthHistory {
    pub name: String, // name of the pool
    pub start_time: String,
    pub end_time: String,
    pub asset_depth: String,
    pub rune_depth: String,
    pub asset_price: String,
    pub asset_price_usd: String,
    pub liquidity_units: String,
    pub members_count: String,
    pub synth_units: String,
    pub synth_supply: String,
    pub units: String,
    pub luvi: String,
}
