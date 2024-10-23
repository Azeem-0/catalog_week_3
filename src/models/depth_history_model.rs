use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DepthHistory {
    pub start_time: String,
    pub end_time: String,

    pub asset_depth: f64,
    pub rune_depth: f64,
    pub asset_price: f64,
    #[serde(rename = "assetPriceUSD")]
    pub asset_price_usd: f64,
    pub liquidity_units: f64,
    pub members_count: f64,
    pub synth_units: f64,
    pub synth_supply: f64,
    pub units: f64,
    pub luvi: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DepthHistoryMeta {
    pub start_time: String,
    pub end_time: String,
    pub price_shift_loss: f64,
    pub luvi_increase: f64,
    pub start_asset_depth: f64,
    pub start_rune_depth: f64,
    #[serde(rename = "startLPUnits")]
    pub start_lp_units: f64,
    pub start_member_count: f64,
    pub start_synth_units: f64,
    pub end_asset_depth: f64,
    pub end_rune_depth: f64,
    #[serde(rename = "endLPUnits")]
    pub end_lp_units: f64,
    pub end_member_count: f64,
    pub end_synth_units: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DepthHistoryResponse {
    pub meta: DepthHistoryMeta,
    pub intervals: Vec<DepthHistory>,
}
