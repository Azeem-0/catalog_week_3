use crate::utils::deserialize_util::deserialize_string_to_number;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SwapsHistory {
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub start_time: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub end_time: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub to_asset_count: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub to_rune_count: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub to_trade_count: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub from_trade_count: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub synth_mint_count: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub synth_redeem_count: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub total_count: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub to_asset_volume: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub to_rune_volume: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub to_trade_volume: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub from_trade_volume: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub synth_mint_volume: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub synth_redeem_volume: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub total_volume: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    #[serde(rename = "toAssetVolumeUSD")]
    pub to_asset_volume_usd: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    #[serde(rename = "toRuneVolumeUSD")]
    pub to_rune_volume_usd: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    #[serde(rename = "toTradeVolumeUSD")]
    pub to_trade_volume_usd: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    #[serde(rename = "fromTradeVolumeUSD")]
    pub from_trade_volume_usd: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    #[serde(rename = "synthMintVolumeUSD")]
    pub synth_mint_volume_usd: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    #[serde(rename = "synthRedeemVolumeUSD")]
    pub synth_redeem_volume_usd: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    #[serde(rename = "totalVolumeUSD")]
    pub total_volume_usd: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub to_asset_fees: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub to_rune_fees: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub to_trade_fees: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub from_trade_fees: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub synth_mint_fees: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub synth_redeem_fees: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub total_fees: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub to_asset_average_slip: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub to_rune_average_slip: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub to_trade_average_slip: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub from_trade_average_slip: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub synth_mint_average_slip: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub synth_redeem_average_slip: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub average_slip: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    #[serde(rename = "runePriceUSD")]
    pub rune_price_usd: f64,
}

impl SwapsHistory {
    pub fn has_field(field: &str) -> bool {
        let camel_to_snake_fields: HashSet<&str> = vec![
            "startTime",
            "endTime",
            "toAssetCount",
            "toRuneCount",
            "toTradeCount",
            "fromTradeCount",
            "synthMintCount",
            "synthRedeemCount",
            "totalCount",
            "toAssetVolume",
            "toRuneVolume",
            "toTradeVolume",
            "fromTradeVolume",
            "synthMintVolume",
            "synthRedeemVolume",
            "totalVolume",
            "toAssetVolumeUSD",
            "toRuneVolumeUSD",
            "toTradeVolumeUSD",
            "fromTradeVolumeUSD",
            "synthMintVolumeUSD",
            "synthRedeemVolumeUSD",
            "totalVolumeUSD",
            "toAssetFees",
            "toRuneFees",
            "toTradeFees",
            "fromTradeFees",
            "synthMintFees",
            "synthRedeemFees",
            "totalFees",
            "toAssetAverageSlip",
            "toRuneAverageSlip",
            "toTradeAverageSlip",
            "fromTradeAverageSlip",
            "synthMintAverageSlip",
            "synthRedeemAverageSlip",
            "averageSlip",
            "runePriceUSD",
        ]
        .into_iter()
        .collect();

        camel_to_snake_fields.contains(field)
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SwapsHistoryMeta {
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub start_time: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub end_time: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub to_asset_count: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub to_rune_count: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub to_trade_count: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub from_trade_count: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub synth_mint_count: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub synth_redeem_count: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub total_count: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub to_asset_volume: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub to_rune_volume: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub to_trade_volume: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub from_trade_volume: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub synth_mint_volume: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub synth_redeem_volume: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub total_volume: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    #[serde(rename = "toAssetVolumeUSD")]
    pub to_asset_volume_usd: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    #[serde(rename = "toRuneVolumeUSD")]
    pub to_rune_volume_usd: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    #[serde(rename = "toTradeVolumeUSD")]
    pub to_trade_volume_usd: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    #[serde(rename = "fromTradeVolumeUSD")]
    pub from_trade_volume_usd: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    #[serde(rename = "synthMintVolumeUSD")]
    pub synth_mint_volume_usd: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    #[serde(rename = "synthRedeemVolumeUSD")]
    pub synth_redeem_volume_usd: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    #[serde(rename = "totalVolumeUSD")]
    pub total_volume_usd: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub to_asset_fees: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub to_rune_fees: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub to_trade_fees: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub from_trade_fees: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub synth_mint_fees: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub synth_redeem_fees: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub total_fees: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub to_asset_average_slip: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub to_rune_average_slip: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub to_trade_average_slip: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub from_trade_average_slip: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub synth_mint_average_slip: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub synth_redeem_average_slip: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    pub average_slip: f64,
    #[serde(deserialize_with = "deserialize_string_to_number")]
    #[serde(rename = "runePriceUSD")]
    pub rune_price_usd: f64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SwapsHistoryResponse {
    pub meta: SwapsHistoryMeta,
    pub intervals: Vec<SwapsHistory>,
}
