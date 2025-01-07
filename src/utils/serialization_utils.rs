use bson::{DateTime as BsonDateTime, Decimal128};
use serde::Serializer;

pub fn serialize_datetime_as_timestamp<S>(
    datetime: &BsonDateTime,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&datetime.timestamp_millis().to_string())
}

pub fn serialize_decimal_as_string<S>(
    decimal: &Decimal128,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&decimal.to_string())
}
