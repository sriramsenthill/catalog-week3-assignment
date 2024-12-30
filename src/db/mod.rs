use crate::schema::DepthHistory;
use futures_util::stream::TryStreamExt; // Import TryStreamExt for try_collect
use mongodb::bson::oid::ObjectId;
use mongodb::{Client, Collection};
use reqwest;
use serde_json::Value; // For parsing JSON responses // Correct import for ObjectId

pub async fn connect_to_mongo(uri: &str) -> Result<Client, mongodb::error::Error> {
    let client = Client::with_uri_str(uri).await?;
    Ok(client)
}

pub async fn fetch_and_store_data() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let response = client
        .get("https://midgard.ninerealms.com/v2/history/depths/BTC.BTC?interval=hour&count=400")
        .send()
        .await?
        .json::<Value>()
        .await?;

    let intervals = response["intervals"].as_array().unwrap();

    let mongo_client = connect_to_mongo(&std::env::var("MONGO_URI").unwrap()).await?;

    let collection: Collection<DepthHistory> = mongo_client
        .database("your_database_name") // Replace with your actual database name
        .collection("depth_history");

    for interval in intervals {
        let depth_data = DepthHistory {
            id: ObjectId::new(), // Automatically generate a new ObjectId
            asset_depth: interval["assetDepth"].as_str().unwrap().to_string(),
            asset_price: interval["assetPrice"].as_str().unwrap().to_string(),
            asset_price_usd: interval["assetPriceUSD"].as_str().unwrap().to_string(),
            end_time: interval["endTime"].as_str().unwrap().to_string(),
            liquidity_units: interval["liquidityUnits"].as_str().unwrap().to_string(),
            luvi: interval["luvi"].as_str().unwrap().to_string(),
            members_count: interval["membersCount"].as_str().unwrap().to_string(),
            rune_depth: interval["runeDepth"].as_str().unwrap().to_string(),
            start_time: interval["startTime"].as_str().unwrap().to_string(),
            synth_supply: interval["synthSupply"].as_str().unwrap().to_string(),
            synth_units: interval["synthUnits"].as_str().unwrap().to_string(),
            units: interval["units"].as_str().unwrap().to_string(),
        };

        collection.insert_one(depth_data, None).await?;
    }

    Ok(())
}

pub async fn get_depth_history_from_db() -> Result<Vec<DepthHistory>, Box<dyn std::error::Error>> {
    let mongo_client = connect_to_mongo(&std::env::var("MONGO_URI").unwrap()).await?;

    let collection: Collection<DepthHistory> = mongo_client
        .database("your_database_name") // Replace with your actual database name
        .collection("depth_history");

    let cursor = collection.find(None, None).await?;

    let results: Vec<DepthHistory> = cursor.try_collect().await?; // Use try_collect here

    Ok(results)
}
