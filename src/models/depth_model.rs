use bson::{DateTime as BsonDateTime, Decimal128};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Depth {
    #[serde(rename = "assetDepth")]
    pub asset_depth: Decimal128,
    #[serde(rename = "assetPrice")]
    pub asset_price: f64,
    #[serde(rename = "assetPriceUSD")]
    pub asset_price_usd: f64,
    #[serde(rename = "endTime")]
    pub end_time: BsonDateTime,
    #[serde(rename = "liquidityUnits")]
    pub liquidity_units: i64,
    pub luvi: f64,
    #[serde(rename = "membersCount")]
    pub members_count: i32,
    #[serde(rename = "runeDepth")]
    pub rune_depth: i64,
    #[serde(rename = "startTime")]
    pub start_time: BsonDateTime,
    #[serde(rename = "synthSupply")]
    pub synth_supply: i64,
    #[serde(rename = "synthUnits")]
    pub synth_units: i64,
    pub units: i64,
}
#[derive(Debug, Deserialize)]
pub struct QueryParams {
    pub interval: Option<String>,
    pub date_range: Option<String>,
    pub sort_by: Option<String>,
    pub order: Option<String>,
    pub limit: Option<i64>,
    pub page: Option<i64>,
}
