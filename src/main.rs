use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use log::{error, info};
use std::env;
use std::error::Error as StdError;
use std::net::SocketAddr;

mod db;
mod error;
mod handlers;
mod models;
mod routes;
mod services;
mod utils;

#[derive(Debug)]
struct AppConfig {
    mongodb_uri: String,
    database_name: String,
    server_addr: SocketAddr,
}

// Custom error type that implements necessary traits
#[derive(Debug)]
struct AppError(String);

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl StdError for AppError {}

// Make AppError thread-safe
impl From<AppError> for std::io::Error {
    fn from(error: AppError) -> Self {
        std::io::Error::new(std::io::ErrorKind::Other, error.0)
    }
}

impl AppConfig {
    fn from_env() -> Result<Self, AppError> {
        dotenv().ok();

        Ok(Self {
            mongodb_uri: env::var("MONGODB_URI")
                .map_err(|_| AppError("MONGODB_URI must be set".to_string()))?,
            database_name: env::var("DATABASE_NAME")
                .map_err(|_| AppError("DATABASE_NAME must be set".to_string()))?,
            server_addr: env::var("SERVER_ADDR")
                .unwrap_or_else(|_| "0.0.0.0:3000".to_string())
                .parse()
                .map_err(|_| AppError("Invalid SERVER_ADDR format".to_string()))?,
        })
    }
}

struct AppState {
    data_service: web::Data<services::data_service::DataService>,
}

impl AppState {
    async fn new(config: &AppConfig) -> Result<Self, AppError> {
        let db = db::init_db(&config.mongodb_uri, &config.database_name)
            .await
            .map_err(|e| AppError(format!("Database initialization failed: {}", e)))?;

        info!("Connected to MongoDB at {}", config.mongodb_uri);

        // Create DataService with database reference
        let data_service = web::Data::new(services::data_service::DataService::new(&db));

        Ok(Self { data_service })
    }
}

async fn setup_app(
    config: AppConfig,
    state: AppState,
) -> Result<actix_web::dev::Server, std::io::Error> {
    let server = HttpServer::new(move || {
        App::new()
            .app_data(state.data_service.clone())
            .configure(routes::config)
            .wrap(actix_web::middleware::Logger::default())
            .wrap(actix_web::middleware::Compress::default())
    })
    .bind(config.server_addr)?
    .run();

    Ok(server)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logger
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    // Load configuration
    let config =
        AppConfig::from_env().map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.0))?;

    info!("Starting server with configuration: {:?}", config);

    // Initialize application state
    let state = AppState::new(&config)
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.0))?;

    // Start server
    let server = setup_app(config, state).await?;
    info!("Server started successfully");

    server.await
}
