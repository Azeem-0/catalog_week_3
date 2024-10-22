use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SwapsHistory {
    pub start_time: String,
    pub end_time: String,
    pub to_asset_count: String,
    pub to_rune_count: String,
    pub to_trade_count: String,
    pub from_trade_count: String,
    pub synth_mint_count: String,
    pub synth_redeem_count: String,
    pub total_count: String,
    pub to_asset_volume: String,
    pub to_rune_volume: String,
    pub to_trade_volume: String,
    pub from_trade_volume: String,
    pub synth_mint_volume: String,
    pub synth_redeem_volume: String,
    pub total_volume: String,
    #[serde(rename = "toAssetVolumeUSD")]
    pub to_asset_volume_usd: String,
    #[serde(rename = "toRuneVolumeUSD")]
    pub to_rune_volume_usd: String,
    #[serde(rename = "toTradeVolumeUSD")]
    pub to_trade_volume_usd: String,
    #[serde(rename = "fromTradeVolumeUSD")]
    pub from_trade_volume_usd: String,
    #[serde(rename = "synthMintVolumeUSD")]
    pub synth_mint_volume_usd: String,
    #[serde(rename = "synthRedeemVolumeUSD")]
    pub synth_redeem_volume_usd: String,
    #[serde(rename = "totalVolumeUSD")]
    pub total_volume_usd: String,
    pub to_asset_fees: String,
    pub to_rune_fees: String,
    pub to_trade_fees: String,
    pub from_trade_fees: String,
    pub synth_mint_fees: String,
    pub synth_redeem_fees: String,
    pub total_fees: String,
    pub to_asset_average_slip: String,
    pub to_rune_average_slip: String,
    pub to_trade_average_slip: String,
    pub from_trade_average_slip: String,
    pub synth_mint_average_slip: String,
    pub synth_redeem_average_slip: String,
    pub average_slip: String,
    #[serde(rename = "runePriceUSD")]
    pub rune_price_usd: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SwapsHistoryMeta {
    pub start_time: String,
    pub end_time: String,
    pub to_asset_count: String,
    pub to_rune_count: String,
    pub to_trade_count: String,
    pub from_trade_count: String,
    pub synth_mint_count: String,
    pub synth_redeem_count: String,
    pub total_count: String,
    pub to_asset_volume: String,
    pub to_rune_volume: String,
    pub to_trade_volume: String,
    pub from_trade_volume: String,
    pub synth_mint_volume: String,
    pub synth_redeem_volume: String,
    pub total_volume: String,
    #[serde(rename = "toAssetVolumeUSD")]
    pub to_asset_volume_usd: String,
    #[serde(rename = "toRuneVolumeUSD")]
    pub to_rune_volume_usd: String,
    #[serde(rename = "toTradeVolumeUSD")]
    pub to_trade_volume_usd: String,
    #[serde(rename = "fromTradeVolumeUSD")]
    pub from_trade_volume_usd: String,
    #[serde(rename = "synthMintVolumeUSD")]
    pub synth_mint_volume_usd: String,
    #[serde(rename = "synthRedeemVolumeUSD")]
    pub synth_redeem_volume_usd: String,
    #[serde(rename = "totalVolumeUSD")]
    pub total_volume_usd: String,
    pub to_asset_fees: String,
    pub to_rune_fees: String,
    pub to_trade_fees: String,
    pub from_trade_fees: String,
    pub synth_mint_fees: String,
    pub synth_redeem_fees: String,
    pub total_fees: String,
    pub to_asset_average_slip: String,
    pub to_rune_average_slip: String,
    pub to_trade_average_slip: String,
    pub from_trade_average_slip: String,
    pub synth_mint_average_slip: String,
    pub synth_redeem_average_slip: String,
    pub average_slip: String,
    #[serde(rename = "runePriceUSD")]
    pub rune_price_usd: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SwapsHistoryResponse {
    pub meta: SwapsHistoryMeta,
    pub intervals: Vec<SwapsHistory>,
}
