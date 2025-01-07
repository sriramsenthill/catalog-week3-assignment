use crate::utils::serialization_utils::serialize_datetime_as_timestamp;
use bson::DateTime as BsonDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Swaps {
    #[serde(rename = "averageSlip")]
    pub average_slip: f64,

    #[serde(rename = "endTime")]
    #[serde(serialize_with = "serialize_datetime_as_timestamp")]
    pub end_time: BsonDateTime,

    #[serde(rename = "fromTradeAverageSlip")]
    pub from_trade_average_slip: f64,

    #[serde(rename = "fromTradeCount")]
    pub from_trade_count: i32,

    #[serde(rename = "fromTradeFees")]
    pub from_trade_fees: i64,

    #[serde(rename = "fromTradeVolume")]
    pub from_trade_volume: i64,

    #[serde(rename = "fromTradeVolumeUSD")]
    pub from_trade_volume_usd: f64,

    #[serde(rename = "runePriceUSD")]
    pub rune_price_usd: f64,

    #[serde(rename = "startTime")]
    #[serde(serialize_with = "serialize_datetime_as_timestamp")]
    pub start_time: BsonDateTime,

    #[serde(rename = "synthMintAverageSlip")]
    pub synth_mint_average_slip: f64,

    #[serde(rename = "synthMintCount")]
    pub synth_mint_count: i32,

    #[serde(rename = "synthMintFees")]
    pub synth_mint_fees: i64,

    #[serde(rename = "synthMintVolume")]
    pub synth_mint_volume: i64,

    #[serde(rename = "synthMintVolumeUSD")]
    pub synth_mint_volume_usd: f64,

    #[serde(rename = "synthRedeemAverageSlip")]
    pub synth_redeem_average_slip: f64,

    #[serde(rename = "synthRedeemCount")]
    pub synth_redeem_count: i32,

    #[serde(rename = "synthRedeemFees")]
    pub synth_redeem_fees: i64,

    #[serde(rename = "synthRedeemVolume")]
    pub synth_redeem_volume: i64,

    #[serde(rename = "synthRedeemVolumeUSD")]
    pub synth_redeem_volume_usd: f64,

    #[serde(rename = "toAssetAverageSlip")]
    pub to_asset_average_slip: f64,

    #[serde(rename = "toAssetCount")]
    pub to_asset_count: i32,

    #[serde(rename = "toAssetFees")]
    pub to_asset_fees: i64,

    #[serde(rename = "toAssetVolume")]
    pub to_asset_volume: i64,

    #[serde(rename = "toAssetVolumeUSD")]
    pub to_asset_volume_usd: f64,

    #[serde(rename = "toRuneAverageSlip")]
    pub to_rune_average_slip: f64,

    #[serde(rename = "toRuneCount")]
    pub to_rune_count: i32,

    #[serde(rename = "toRuneFees")]
    pub to_rune_fees: i64,

    #[serde(rename = "toRuneVolume")]
    pub to_rune_volume: i64,

    #[serde(rename = "toRuneVolumeUSD")]
    pub to_rune_volume_usd: f64,

    #[serde(rename = "toTradeAverageSlip")]
    pub to_trade_average_slip: f64,

    #[serde(rename = "toTradeCount")]
    pub to_trade_count: i32,

    #[serde(rename = "toTradeFees")]
    pub to_trade_fees: i64,

    #[serde(rename = "toTradeVolume")]
    pub to_trade_volume: i64,

    #[serde(rename = "toTradeVolumeUSD")]
    pub to_trade_volume_usd: f64,

    #[serde(rename = "totalCount")]
    pub total_count: i32,

    #[serde(rename = "totalFees")]
    pub total_fees: i64,

    #[serde(rename = "totalVolume")]
    pub total_volume: i64,

    #[serde(rename = "totalVolumeUSD")]
    pub total_volume_usd: f64,
}
