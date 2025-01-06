use crate::models::depth_model::{Depth, QueryParams};
use bson::{doc, from_document, Document};
use chrono::{DateTime, Utc};
use futures::stream::TryStreamExt;
use mongodb::{Collection, Database};

pub struct DepthDB {
    collection: Collection<Document>,
}

impl DepthDB {
    pub fn new(db: &Database) -> Self {
        let collection = db.collection("depths");
        Self { collection }
    }

    pub async fn find_depths(
        &self,
        params: &QueryParams,
    ) -> Result<Vec<Depth>, mongodb::error::Error> {
        let mut pipeline = Vec::new();

        // Check if there are no query parameters
        if params.date_range.is_none() && params.interval.is_none() && params.sort_by.is_none() {
            // If no query params, return the first 400 records
            log::debug!("No query parameters provided; limiting to first 400 records.");
            pipeline.push(doc! { "$limit": 400 });
        } else {
            // Build the match stage based on date range
            if let Some(date_range) = &params.date_range {
                let dates: Vec<&str> = date_range.split(',').collect();
                log::debug!("Received date range: {:?}", dates);
                if dates.len() == 2 {
                    let mut match_doc = doc! {};

                    if !dates[0].is_empty() {
                        match_doc.insert(
                            "start_time",
                            doc! {
                                "$gte": DateTime::parse_from_str(
                                    &format!("{} 00:00:00 +0000", dates[0]),
                                    "%Y-%m-%d %H:%M:%S %z"
                                )
                                .unwrap()
                                .with_timezone(&Utc)
                            },
                        );
                    }

                    if !dates[1].is_empty() {
                        match_doc.insert(
                            "end_time",
                            doc! {
                                "$lt": DateTime::parse_from_str(
                                    &format!("{} 00:00:00 +0000", dates[1]),
                                    "%Y-%m-%d %H:%M:%S %z"
                                )
                                .unwrap()
                                .with_timezone(&Utc)
                            },
                        );
                    }

                    pipeline.push(doc! { "$match": match_doc });
                }
            }

            // Build the group stage based on interval
            if let Some(interval) = &params.interval {
                let group_id = match interval.as_str() {
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
                };

                pipeline.push(doc! {
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
                });
            }

            // Build the sort stage
            if let Some(sort_by) = &params.sort_by {
                let order = if params.order.as_deref() == Some("desc") {
                    -1
                } else {
                    1
                };
                pipeline.push(doc! {
                    "$sort": {
                        sort_by: order
                    }
                });
            }

            // Add pagination
            let skip = (params.page.unwrap_or(1) - 1) * params.limit.unwrap_or(10);
            pipeline.push(doc! {
                "$skip": skip
            });

            pipeline.push(doc! {
                "$limit": params.limit.unwrap_or(10)
            });
        }

        log::debug!("Final pipeline: {:?}", pipeline);

        let mut depths = Vec::new();
        let mut cursor = self.collection.aggregate(pipeline, None).await?;

        // Iterate through the cursor and convert documents to Depth structs
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
