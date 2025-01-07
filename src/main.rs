use env_logger;
use log::info;

mod db;
mod error;
mod handlers;
mod models;
mod routes;
mod services;
mod state;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logger
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    // Load configuration
    let config = utils::config::AppConfig::from_env()
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.0))?;

    info!("Starting server with configuration: {:?}", config);

    // Initialize application state
    let state = state::AppState::new(&config)
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.0))?;

    // Start server
    let server = utils::server::setup_app(config, state).await?;
    info!("Server started successfully");

    server.await
}
