use bson::DateTime as BsonDateTime;
use serde::{Deserialize, Serialize, Serializer}; // Added Serializer import

pub fn serialize_optional_datetime_as_timestamp<S>(
    datetime: &Option<BsonDateTime>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match datetime {
        Some(dt) => serializer.serialize_str(&dt.timestamp_millis().to_string()),
        None => serializer.serialize_none(),
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Earnings {
    #[serde(rename = "avgNodeCount")]
    pub avg_node_count: f64, // double

    #[serde(rename = "blockRewards")]
    pub block_rewards: i64, // long

    #[serde(rename = "bondingEarnings")]
    pub bonding_earnings: i64, // long

    pub earnings: i64,

    #[serde(rename = "endTime")]
    #[serde(serialize_with = "serialize_optional_datetime_as_timestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<BsonDateTime>, // date

    #[serde(rename = "liquidityEarnings")]
    pub liquidity_earnings: i64, // long

    #[serde(rename = "liquidityFees")]
    pub liquidity_fees: i64, // long

    #[serde(rename = "runePriceUSD")]
    pub rune_price_usd: String, // string

    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_optional_datetime_as_timestamp")]
    pub start_time: Option<BsonDateTime>, // date

    pub pools: Vec<LiquidityPool>, // array
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LiquidityPool {
    #[serde(rename = "assetLiquidityFees")]
    pub asset_liquidity_fees: i64, // long

    pub earnings: i64, // long

    pub pool: String, // string

    pub rewards: i64, // long

    #[serde(rename = "runeLiquidityFees")]
    pub rune_liquidity_fees: i64, // long

    #[serde(rename = "runePriceUSD")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rune_price_usd: Option<String>, // null (as an Option<String>)

    #[serde(rename = "saverEarning")]
    pub saver_earning: i64, // long

    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_optional_datetime_as_timestamp")]
    pub start_time: Option<BsonDateTime>, // null (as an Option<BsonDateTime>)

    #[serde(rename = "totalLiquidityFeesRune")]
    pub total_liquidity_fees_rune: i64, // long
}
