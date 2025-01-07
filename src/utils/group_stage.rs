// utils/group_stage.rs
use crate::utils::validate_interval;
use bson::{doc, Document};

pub fn build_group_stage(interval: &Option<String>) -> Option<Document> {
    interval.as_ref().and_then(|interval| {
        if !validate_interval(interval) {
            log::warn!("Invalid interval value: {}", interval);
            return None;
        }
        let group_id = get_group_id_for_interval(interval);
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

fn get_group_id_for_interval(interval: &str) -> Document {
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
