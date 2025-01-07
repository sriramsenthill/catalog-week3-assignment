use crate::utils::parse_date_range;
use bson::{doc, DateTime as BsonDateTime, Document};
use chrono::{DateTime, Utc};

pub fn build_match_stage(date_range: &Option<String>, interval: &str) -> Option<Document> {
    date_range.as_ref().and_then(|date_range| {
        let parsed_dates = parse_date_range(date_range)?;

        log::debug!(
            "Building match stage with start: {:?}, end: {:?}",
            parsed_dates.start,
            parsed_dates.end
        );

        let mut match_doc = doc! {};

        if let Some(start) = parsed_dates.start {
            // Convert to BSON DateTime
            let start_bson = BsonDateTime::from_chrono(start);
            log::debug!("Adding start time filter: {:?}", start_bson);
            match_doc.insert("startTime", doc! { "$gte": start_bson });
        }

        if let Some(end) = parsed_dates.end {
            // Convert to BSON DateTime
            let end_bson = BsonDateTime::from_chrono(end);
            log::debug!("Adding end time filter: {:?}", end_bson);
            match_doc.insert("startTime", doc! { "$lt": end_bson }); // Changed from endTime to startTime
        }

        let final_doc = doc! { "$match": match_doc };
        log::debug!("Final match stage: {:?}", final_doc);

        Some(final_doc)
    })
}
