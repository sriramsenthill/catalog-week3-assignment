use crate::utils::serialization_utils::serialize_datetime_as_timestamp;
use bson::DateTime as BsonDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RunePool {
    #[serde(rename = "count")]
    pub count: i32,

    #[serde(rename = "endTime")]
    #[serde(serialize_with = "serialize_datetime_as_timestamp")]
    pub end_time: BsonDateTime,

    #[serde(rename = "startTime")]
    #[serde(serialize_with = "serialize_datetime_as_timestamp")]
    pub start_time: BsonDateTime,

    pub units: i64,
}
