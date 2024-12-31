// main.rs
mod db;
mod error;
mod routes;
mod scheduler;
mod schema;

use axum::{extract::Extension, routing::get, Router};
use dotenv::dotenv;
use std::{env, sync::Arc};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let mongo_uri = env::var("MONGO_URI").expect("MONGO_URI must be set");
    let db_name = env::var("DB_NAME").expect("DB_NAME must be set");

    let db = Arc::new(
        db::Database::new(&mongo_uri, &db_name)
            .await
            .expect("Failed to connect to database"),
    );

    // Start the scheduler in a separate task
    let scheduler_db = Arc::clone(&db);
    tokio::spawn(async move {
        scheduler::start_scheduler(scheduler_db).await;
    });

    // Set up Axum router with database connection
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route(
            "/api/depth-history",
            get(routes::depth_history::get_depth_history),
        )
        .layer(Extension(db));

    println!("Server starting on 0.0.0.0:3000");
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
