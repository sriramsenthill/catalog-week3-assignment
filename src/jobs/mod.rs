use crate::db::insert_depth_history;
use reqwest;

pub async fn fetch_and_store_data() {
    let client = reqwest::Client::new();
    let response = client
        .get("https://midgard.ninerealms.com/v2/history/depths/BTC.BTC")
        .header("x-client-id", "YOUR_CLIENT_ID")
        .send()
        .await
        .unwrap();

    let depth_data: DepthHistoryResponse = response.json().await.unwrap();

    insert_depth_history(depth_data).await.unwrap();
}
