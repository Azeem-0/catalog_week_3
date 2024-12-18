use std::collections::HashSet;

use crate::utils::deserialize_util::deserialize_string_to_number;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct EarningsHistoryPool {
    pub pool: String,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub asset_liquidity_fees: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub rune_liquidity_fees: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub total_liquidity_fees_rune: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub saver_earning: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub rewards: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub earnings: f64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct EarningsHistory {
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub start_time: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub end_time: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub liquidity_fees: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub block_rewards: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub earnings: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub bonding_earnings: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub liquidity_earnings: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub avg_node_count: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    #[serde(rename = "runePriceUSD")]
    pub rune_price_usd: f64,
    pub pools: Vec<EarningsHistoryPool>,
}

impl EarningsHistory {
    pub fn has_field(field: &str) -> bool {
        let camel_to_snake_fields: HashSet<&str> = vec![
            "startTime",
            "endTime",
            "liquidityFees",
            "blockRewards",
            "earnings",
            "bondingEarnings",
            "liquidityEarnings",
            "avgNodeCount",
            "runePriceUSD",
        ]
        .into_iter()
        .collect();

        camel_to_snake_fields.contains(field)
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct EarningsHistoryMeta {
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub start_time: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub end_time: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub liquidity_fees: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub block_rewards: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub earnings: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub bonding_earnings: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub liquidity_earnings: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub avg_node_count: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    #[serde(rename = "runePriceUSD")]
    pub rune_price_usd: f64,
}
#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct EarningsHistoryResponse {
    #[schema(inline)]
    pub meta: EarningsHistoryMeta,
    pub intervals: Vec<EarningsHistory>,
}

#[derive(Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct EarningsHistoryFlattenResponse {
    pub pool: String,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub start_time: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub end_time: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub liquidity_fees: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub block_rewards: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub earnings: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub bonding_earnings: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub liquidity_earnings: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub avg_node_count: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    #[serde(rename = "runePriceUSD")]
    pub rune_price_usd: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub asset_liquidity_fees: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub rune_liquidity_fees: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub total_liquidity_fees_rune: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub saver_earning: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub rewards: f64,
}
