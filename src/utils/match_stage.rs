// utils/match_stage.rs
use crate::utils::date_utils::parse_date_range;
use bson::{doc, Bson, DateTime as BsonDateTime, Document};

pub fn build_match_stage(date_range: &Option<String>) -> Option<Document> {
    date_range.as_ref().and_then(|date_range| {
        let parsed_dates = parse_date_range(date_range)?;
        let mut match_doc = doc! {};

        if let Some(start) = parsed_dates.start {
            let start_bson = BsonDateTime::from_chrono(start);
            match_doc.insert("startTime", doc! { "$gte": start_bson });
        }

        if let Some(end) = parsed_dates.end {
            let end_bson = BsonDateTime::from_chrono(end);
            // Fixed the document access pattern
            if let Some(existing_start) = match_doc.get("startTime").and_then(|d| d.as_document()) {
                match_doc.insert(
                    "startTime",
                    doc! {
                        "$gte": existing_start.get("$gte").unwrap_or(&Bson::Null),
                        "$lt": end_bson
                    },
                );
            } else {
                match_doc.insert("startTime", doc! { "$lt": end_bson });
            }
        }

        Some(doc! { "$match": match_doc })
    })
}
