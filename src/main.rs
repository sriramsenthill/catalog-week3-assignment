mod db;
mod routes;
mod schema;

use axum::{routing::get, Router};
use dotenv::dotenv;
use std::env;
use tokio;

#[tokio::main]
async fn main() {
    dotenv().ok(); // Load environment variables from .env file

    // Connect to MongoDB
    let mongo_uri = env::var("MONGO_URI").expect("MONGO_URI must be set");
    match db::connect_to_mongo(&mongo_uri).await {
        Ok(_) => {
            println!("Successfully connected to MongoDB.");
            // Fetch and store data from Midgard API
            if let Err(e) = db::fetch_and_store_data().await {
                eprintln!("Error fetching and storing data: {}", e);
            }
        }
        Err(e) => {
            eprintln!("Failed to connect to MongoDB: {}", e);
            return;
        }
    }

    // Set up Axum router
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route(
            "/api/depth-history",
            get(routes::depth_history::get_depth_history),
        );

    // Run the server
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
