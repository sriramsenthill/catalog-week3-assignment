// utils/match_stage.rs
use crate::utils::parse_date_range;
use bson::{doc, Document};

pub fn build_match_stage(date_range: &Option<String>) -> Option<Document> {
    date_range.as_ref().and_then(|date_range| {
        let parsed_dates = parse_date_range(date_range)?;
        let mut match_doc = doc! {};

        if let Some(start) = parsed_dates.start {
            match_doc.insert("start_time", doc! { "$gte": start });
        }

        if let Some(end) = parsed_dates.end {
            match_doc.insert("end_time", doc! { "$lt": end });
        }

        Some(doc! { "$match": match_doc })
    })
}
