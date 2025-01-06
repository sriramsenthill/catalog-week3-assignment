use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use log::info;
use std::env;

mod db;
mod error;
mod handlers;
mod models;
mod routes;
mod services;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let mongodb_uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set");
    let database_name = env::var("DATABASE_NAME").expect("DATABASE_NAME must be set");

    let db = db::init_db(&mongodb_uri, &database_name)
        .await
        .expect("Failed to connect to MongoDB");

    info!("Connected to MongoDB");

    // Create an instance of DepthDB
    let depth_db = db::depth_db::DepthDB::new(&db); // Ensure this matches your DepthDB constructor

    // Create an instance of DepthService
    let depth_service = web::Data::new(services::depth_service::DepthService::new(depth_db));

    HttpServer::new(move || {
        App::new()
            .app_data(depth_service.clone()) // Ensure DepthService is added here
            .configure(routes::config)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
