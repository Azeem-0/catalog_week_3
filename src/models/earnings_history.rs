use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EarningsHistory {
    pub name: String, // name of the pool
    pub start_time: String,
    pub end_time: String,
    pub liquidity_fees: String,
    pub block_rewards: String,
    pub earnings: String,
    pub bonding_earnings: String,
    pub liquidity_earnings: String,
    pub avg_node_count: String,
    pub rune_price_usd: String,
}
