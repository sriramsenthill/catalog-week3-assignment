use crate::models::depth_model::{Depth, QueryParams};
use bson::{doc, from_document, Document};
use chrono::{DateTime, Utc};
use futures::stream::TryStreamExt;
use mongodb::{error::Error as MongoError, Collection, Database};
use std::collections::HashMap;

const DEFAULT_LIMIT: i64 = 10;
const MAX_RECORDS_NO_FILTER: i64 = 400;

pub struct DepthDB {
    collection: Collection<Document>,
}

#[derive(Debug)]
struct DateRange {
    start: Option<DateTime<Utc>>,
    end: Option<DateTime<Utc>>,
}

impl DepthDB {
    pub fn new(db: &Database) -> Self {
        let collection = db.collection("depths");
        Self { collection }
    }

    pub async fn find_depths(&self, params: &QueryParams) -> Result<Vec<Depth>, MongoError> {
        let pipeline = self.build_pipeline(params);
        log::debug!("Final pipeline: {:?}", pipeline);
        self.execute_pipeline(pipeline).await
    }

    fn build_pipeline(&self, params: &QueryParams) -> Vec<Document> {
        if self.should_use_simple_query(params) {
            return vec![doc! { "$limit": MAX_RECORDS_NO_FILTER }];
        }

        let mut pipeline = Vec::new();

        if let Some(match_stage) = self.build_match_stage(params) {
            pipeline.push(match_stage);
        }

        if let Some(group_stage) = self.build_group_stage(params) {
            pipeline.push(group_stage);
        }

        if let Some(sort_stage) = self.build_sort_stage(params) {
            pipeline.push(sort_stage);
        }

        self.add_pagination_stages(&mut pipeline, params);
        pipeline
    }

    fn should_use_simple_query(&self, params: &QueryParams) -> bool {
        params.date_range.is_none() && params.interval.is_none() && params.sort_by.is_none()
    }

    fn build_match_stage(&self, params: &QueryParams) -> Option<Document> {
        params.date_range.as_ref().and_then(|date_range| {
            let parsed_dates = self.parse_date_range(date_range)?;
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

    fn parse_date_range(&self, date_range: &str) -> Option<DateRange> {
        let dates: Vec<&str> = date_range.split(',').collect();
        if dates.len() != 2 {
            log::warn!("Invalid date range format: {}", date_range);
            return None;
        }

        let parse_date = |date: &str| {
            if date.trim().is_empty() {
                None
            } else {
                DateTime::parse_from_str(
                    &format!("{} 00:00:00 +0000", date),
                    "%Y-%m-%d %H:%M:%S %z",
                )
                .ok()
                .map(|dt| dt.with_timezone(&Utc))
            }
        };

        Some(DateRange {
            start: parse_date(dates[0]),
            end: parse_date(dates[1]),
        })
    }

    fn build_group_stage(&self, params: &QueryParams) -> Option<Document> {
        params.interval.as_ref().and_then(|interval| {
            if !self.validate_interval(interval) {
                log::warn!("Invalid interval value: {}", interval);
                return None;
            }
            let group_id = self.get_group_id_for_interval(interval);
            Some(doc! {
                "$group": {
                    "_id": group_id,
                    "asset_depth": { "$avg": "$asset_depth" },
                    "asset_price": { "$avg": "$asset_price" },
                    "asset_price_usd": { "$avg": "$asset_price_usd" },
                    "liquidity_units": { "$avg": "$liquidity_units" },
                    "luvi": { "$avg": "$luvi" },
                    "members_count": { "$avg": "$members_count" },
                    "rune_depth": { "$avg": "$rune_depth" },
                    "synth_supply": { "$avg": "$synth_supply" },
                    "synth_units": { "$avg": "$synth_units" },
                    "units": { "$avg": "$units" }
                }
            })
        })
    }

    fn validate_interval(&self, interval: &str) -> bool {
        matches!(interval, "hour" | "day" | "week" | "month")
    }

    fn get_group_id_for_interval(&self, interval: &str) -> Document {
        match interval {
            "hour" => doc! {
                "year": { "$year": "$start_time" },
                "month": { "$month": "$start_time" },
                "day": { "$dayOfMonth": "$start_time" },
                "hour": { "$hour": "$start_time" }
            },
            "day" => doc! {
                "year": { "$year": "$start_time" },
                "month": { "$month": "$start_time" },
                "day": { "$dayOfMonth": "$start_time" }
            },
            "week" => doc! {
                "year": { "$year": "$start_time" },
                "week": { "$week": "$start_time" }
            },
            "month" => doc! {
                "year": { "$year": "$start_time" },
                "month": { "$month": "$start_time" }
            },
            _ => doc! {},
        }
    }

    fn build_sort_stage(&self, params: &QueryParams) -> Option<Document> {
        params.sort_by.as_ref().map(|sort_by| {
            let order = if params.order.as_deref() == Some("desc") {
                -1
            } else {
                1
            };
            doc! { "$sort": { sort_by.clone(): order } }
        })
    }

    fn add_pagination_stages(&self, pipeline: &mut Vec<Document>, params: &QueryParams) {
        let page = params.page.unwrap_or(1).max(1);
        let limit = params.limit.unwrap_or(DEFAULT_LIMIT).max(1);
        let skip = (page - 1) * limit;
        pipeline.push(doc! { "$skip": skip });
        pipeline.push(doc! { "$limit": limit });
    }

    async fn execute_pipeline(&self, pipeline: Vec<Document>) -> Result<Vec<Depth>, MongoError> {
        let mut cursor = self.collection.aggregate(pipeline, None).await?;
        let mut depths = Vec::new();

        while let Some(doc_result) = cursor.try_next().await? {
            log::debug!("Fetched document: {:?}", doc_result);
            if let Ok(depth) = from_document(doc_result) {
                depths.push(depth);
            }
        }

        log::debug!("Depths found: {:?}", depths);
        Ok(depths)
    }
}
