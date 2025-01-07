use crate::utils::validate_interval;
use bson::{doc, Document};

pub fn build_group_stage(interval: &Option<String>) -> Option<Document> {
    log::debug!("Received interval: {:?}", interval);

    interval.as_ref().and_then(|interval| {
        if !validate_interval(interval) {
            log::warn!("Invalid interval value: {}", interval);
            return None;
        }

        let group_id = get_group_id_for_interval(interval);
        log::debug!(
            "Generated group ID for interval {}: {:?}",
            interval,
            group_id
        );

        Some(doc! {
            "$group": {
                "_id": group_id,
                "asset_depth": { "$avg": "$assetDepth" },
                "asset_price": { "$avg": "$assetPrice" },
                "asset_price_usd": { "$avg": "$assetPriceUSD" },
                "liquidity_units": { "$avg": "$liquidityUnits" },
                "luvi": { "$avg": "$luvi" },
                "members_count": { "$avg": "$membersCount" },
                "rune_depth": { "$avg": "$runeDepth" },
                "synth_supply": { "$avg": "$synthSupply" },
                "synth_units": { "$avg": "$synthUnits" },
                "units": { "$avg": "$units" }
            }
        })
    })
}

fn get_group_id_for_interval(interval: &str) -> Document {
    match interval {
        "hour" => doc! {
            "year": { "$year": "$startTime" },
            "month": { "$month": "$startTime" },
            "day": { "$dayOfMonth": "$startTime" },
            "hour": { "$hour": "$startTime" }
        },
        "day" => doc! {
            "year": { "$year": "$startTime" },
            "month": { "$month": "$startTime" },
            "day": { "$dayOfMonth": "$startTime" }
        },
        _ => doc! {}, // Fallback for invalid intervals
    }
}
