use crate::utils::{serialize_datetime_as_timestamp, serialize_decimal_as_string};
use bson::{DateTime as BsonDateTime, Decimal128};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Deserialize)]
pub struct QueryParams {
    pub interval: Option<String>,
    pub date_range: Option<String>,
    pub sort_by: Option<String>,
    pub order: Option<String>,
    pub limit: Option<i64>,
    pub page: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Depth {
    #[serde(rename = "assetDepth")]
    #[serde(serialize_with = "serialize_decimal_as_string")]
    pub asset_depth: Decimal128,

    #[serde(rename = "assetPrice")]
    pub asset_price: f64,

    #[serde(rename = "assetPriceUSD")]
    pub asset_price_usd: f64,

    #[serde(rename = "endTime")]
    #[serde(serialize_with = "serialize_datetime_as_timestamp")]
    pub end_time: BsonDateTime,

    #[serde(rename = "liquidityUnits")]
    pub liquidity_units: i64,

    pub luvi: f64,

    #[serde(rename = "membersCount")]
    pub members_count: i32,

    #[serde(rename = "runeDepth")]
    pub rune_depth: i64,

    #[serde(rename = "startTime")]
    #[serde(serialize_with = "serialize_datetime_as_timestamp")]
    pub start_time: BsonDateTime,

    #[serde(rename = "synthSupply")]
    pub synth_supply: i64,

    #[serde(rename = "synthUnits")]
    pub synth_units: i64,

    pub units: i64,
}
