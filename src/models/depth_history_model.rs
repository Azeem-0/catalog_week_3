use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DepthHistory {
    pub start_time: String,
    pub end_time: String,
    pub asset_depth: String,
    pub rune_depth: String,
    pub asset_price: String,
    #[serde(rename = "assetPriceUSD")]
    pub asset_price_usd: String,
    pub liquidity_units: String,
    pub members_count: String,
    pub synth_units: String,
    pub synth_supply: String,
    pub units: String,
    pub luvi: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DepthHistoryMeta {
    pub start_time: String,
    pub end_time: String,
    pub price_shift_loss: String,
    pub luvi_increase: String,
    pub start_asset_depth: String,
    pub start_rune_depth: String,
    #[serde(rename = "startLPUnits")]
    pub start_lp_units: String,
    pub start_member_count: String,
    pub start_synth_units: String,
    pub end_asset_depth: String,
    pub end_rune_depth: String,
    #[serde(rename = "endLPUnits")]
    pub end_lp_units: String,
    pub end_member_count: String,
    pub end_synth_units: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DepthHistoryResponse {
    pub meta: DepthHistoryMeta,
    pub intervals: Vec<DepthHistory>,
}
