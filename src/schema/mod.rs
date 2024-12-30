use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize}; // Correct import for ObjectId

#[derive(Serialize, Deserialize)]
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
