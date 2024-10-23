use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EarningsHistoryPool {
    pub pools: String,
    pub asset_liquidity_fees: String,
    pub rune_liquidity_fees: String,
    pub total_liquidity_fees_rune: String,
    pub saver_earning: String,
    pub rewards: String,
    pub earnings_pool: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EarningsHistory {
    pub start_time: String,
    pub end_time: String,
    pub liquidity_fees: String,
    pub block_rewards: String,
    pub earnings: String,
    pub bonding_earnings: String,
    pub liquidity_earnings: String,
    pub avg_node_count: String,
    #[serde(rename = "runePriceUSD")]
    pub rune_price_usd: String,
    pub pools: Vec<EarningsHistoryPool>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EarningsHistoryMeta {
    pub start_time: String,
    pub end_time: String,
    pub liquidity_fees: String,
    pub block_rewards: String,
    pub earnings: String,
    pub bonding_earnings: String,
    pub liquidity_earnings: String,
    pub avg_node_count: String,
    #[serde(rename = "runePriceUSD")]
    pub rune_price_usd: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EarningsHistoryResponse {
    pub meta: EarningsHistoryMeta,
    pub intervals: Vec<EarningsHistory>,
}
