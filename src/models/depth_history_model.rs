use serde::{Deserialize, Deserializer, Serialize};

use crate::utils::deserialize_util::deserialize_string_to_number;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DepthHistory {
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub start_time: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub end_time: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub asset_depth: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub rune_depth: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub asset_price: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    #[serde(rename = "assetPriceUSD")]
    pub asset_price_usd: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub liquidity_units: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub members_count: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub synth_units: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub synth_supply: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub units: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub luvi: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DepthHistoryMeta {
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub start_time: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub end_time: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub price_shift_loss: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub luvi_increase: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub start_asset_depth: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub start_rune_depth: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    #[serde(rename = "startLPUnits")]
    pub start_lp_units: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub start_member_count: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub start_synth_units: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub end_asset_depth: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub end_rune_depth: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    #[serde(rename = "endLPUnits")]
    pub end_lp_units: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub end_member_count: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub end_synth_units: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DepthHistoryResponse {
    pub meta: DepthHistoryMeta,
    pub intervals: Vec<DepthHistory>,
}
