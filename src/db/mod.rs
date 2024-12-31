// db/mod.rs
use crate::error::AppError;
use crate::schema::{DepthHistory, HistoryParams, MetaData};
use chrono::Utc;
use futures_util::stream::TryStreamExt;
use mongodb::{
    bson::{doc, oid::ObjectId},
    options::{FindOptions, IndexOptions},
    Client, Collection, IndexModel,
};
use reqwest;
use serde_json::Value;

pub struct Database {
    client: Client,
    db_name: String,
}

impl Database {
    pub async fn new(uri: &str, db_name: &str) -> Result<Self, AppError> {
        let client = Client::with_uri_str(uri).await?;
        let db = Self {
            client,
            db_name: db_name.to_string(),
        };
        db.ensure_indexes().await?;
        Ok(db)
    }

    async fn ensure_indexes(&self) -> Result<(), AppError> {
        let collection = self.get_collection();

        let options = IndexOptions::builder().unique(true).build();
        let index = IndexModel::builder()
            .keys(doc! {
                "start_time": 1,
                "end_time": 1
            })
            .options(options)
            .build();

        collection.create_index(index, None).await?;
        Ok(())
    }

    fn get_collection(&self) -> Collection<DepthHistory> {
        self.client
            .database(&self.db_name)
            .collection("depth_history")
    }

    // db/mod.rs (relevant section)
    pub async fn fetch_and_store_data(&self, mut from_timestamp: i64) -> Result<(), AppError> {
        let client = reqwest::Client::new();
        let now = Utc::now().timestamp();

        while from_timestamp < now {
            let from_str = from_timestamp.to_string();
            let query_params = vec![("interval", "hour"), ("count", "400"), ("from", &from_str)];

            let url = "https://midgard.ninerealms.com/v2/history/depths/BTC.BTC";
            println!(
                "Fetching data from: {} with params: {:?}",
                url, query_params
            );

            let response = client.get(url).query(&query_params).send().await?;

            if !response.status().is_success() {
                let status = response.status();
                let text = response.text().await?;
                return Err(AppError::Other(format!(
                    "API request failed with status {}: {}",
                    status, text
                )));
            }

            let response_text = response.text().await?;
            let json: Value = serde_json::from_str(&response_text)?;

            let intervals = json["intervals"].as_array().ok_or_else(|| {
                AppError::Parse("No 'intervals' array found in response".to_string())
            })?;

            let meta: MetaData = serde_json::from_value(json["meta"].clone())
                .map_err(|e| AppError::Parse(format!("Failed to parse meta data: {}", e)))?;

            from_timestamp = meta
                .end_time
                .parse::<i64>()
                .map_err(|e| AppError::Parse(format!("Failed to parse end_time: {}", e)))?;

            println!("Processing {} intervals", intervals.len());
            if intervals.is_empty() {
                break;
            }

            let collection = self.get_collection();

            for interval in intervals {
                let depth_data = DepthHistory {
                    id: ObjectId::new(),
                    asset_depth: interval["assetDepth"]
                        .as_str()
                        .unwrap_or_default()
                        .to_string(),
                    asset_price: interval["assetPrice"]
                        .as_str()
                        .unwrap_or_default()
                        .to_string(),
                    asset_price_usd: interval["assetPriceUSD"]
                        .as_str()
                        .unwrap_or_default()
                        .to_string(),
                    end_time: interval["endTime"].as_str().unwrap_or_default().to_string(),
                    liquidity_units: interval["liquidityUnits"]
                        .as_str()
                        .unwrap_or_default()
                        .to_string(),
                    luvi: interval["luvi"].as_str().unwrap_or_default().to_string(),
                    members_count: interval["membersCount"]
                        .as_str()
                        .unwrap_or_default()
                        .to_string(),
                    rune_depth: interval["runeDepth"]
                        .as_str()
                        .unwrap_or_default()
                        .to_string(),
                    start_time: interval["startTime"]
                        .as_str()
                        .unwrap_or_default()
                        .to_string(),
                    synth_supply: interval["synthSupply"]
                        .as_str()
                        .unwrap_or_default()
                        .to_string(),
                    synth_units: interval["synthUnits"]
                        .as_str()
                        .unwrap_or_default()
                        .to_string(),
                    units: interval["units"].as_str().unwrap_or_default().to_string(),
                };

                collection
                    .update_one(
                        doc! {
                            "start_time": &depth_data.start_time,
                            "end_time": &depth_data.end_time
                        },
                        doc! {
                            "$set": {
                                "asset_depth": &depth_data.asset_depth,
                                "asset_price": &depth_data.asset_price,
                                "asset_price_usd": &depth_data.asset_price_usd,
                                "liquidity_units": &depth_data.liquidity_units,
                                "luvi": &depth_data.luvi,
                                "members_count": &depth_data.members_count,
                                "rune_depth": &depth_data.rune_depth,
                                "synth_supply": &depth_data.synth_supply,
                                "synth_units": &depth_data.synth_units,
                                "units": &depth_data.units
                            }
                        },
                        Some(
                            mongodb::options::UpdateOptions::builder()
                                .upsert(true)
                                .build(),
                        ),
                    )
                    .await?;
            }
        }

        Ok(())
    }

    pub async fn get_depth_history(
        &self,
        params: &HistoryParams,
    ) -> Result<Vec<DepthHistory>, AppError> {
        let collection = self.get_collection();
        let mut filter = doc! {};

        if let Some(from) = params.from {
            filter.insert("start_time", doc! { "$gte": from.to_string() });
        }

        let count = std::cmp::min(params.count.unwrap_or(400), 400) as i64;

        let options = FindOptions::builder()
            .limit(count)
            .sort(doc! { "start_time": 1 })
            .build();

        let cursor = collection.find(filter, options).await?;
        let results = cursor.try_collect().await?;

        Ok(results)
    }
}
