// schema/mod.rs
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DepthHistory {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub asset_depth: String,
    pub asset_price: String,
    pub asset_price_usd: String,
    pub end_time: String,
    pub liquidity_units: String,
    pub luvi: String,
    pub members_count: String,
    pub rune_depth: String,
    pub start_time: String,
    pub synth_supply: String,
    pub synth_units: String,
    pub units: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HistoryParams {
    pub from: Option<i64>,
    pub interval: Option<String>,
    pub count: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MetaData {
    #[serde(rename = "endTime")]
    pub end_time: String,
    #[serde(rename = "startTime")]
    pub start_time: String,
    #[serde(rename = "endAssetDepth")]
    pub end_asset_depth: String,
    #[serde(rename = "endLPUnits")]
    pub end_lp_units: String,
    #[serde(rename = "endMemberCount")]
    pub end_member_count: String,
    #[serde(rename = "endRuneDepth")]
    pub end_rune_depth: String,
    #[serde(rename = "endSynthUnits")]
    pub end_synth_units: String,
    #[serde(rename = "startAssetDepth")]
    pub start_asset_depth: String,
    #[serde(rename = "startLPUnits")]
    pub start_lp_units: String,
    #[serde(rename = "startMemberCount")]
    pub start_member_count: String,
    #[serde(rename = "startRuneDepth")]
    pub start_rune_depth: String,
    #[serde(rename = "startSynthUnits")]
    pub start_synth_units: String,
    #[serde(rename = "luviIncrease")]
    pub luvi_increase: String,
    #[serde(rename = "priceShiftLoss")]
    pub price_shift_loss: String,
}
