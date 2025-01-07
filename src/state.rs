use crate::utils::config::AppError;
use crate::{db, services};
use actix_web::web;

pub struct AppState {
    pub data_service: web::Data<services::data_service::DataService>,
}

impl AppState {
    pub async fn new(config: &crate::utils::config::AppConfig) -> Result<Self, AppError> {
        let db = db::init_db(&config.mongodb_uri, &config.database_name)
            .await
            .map_err(|e| AppError(format!("Database initialization failed: {}", e)))?;

        log::info!("Connected to MongoDB at {}", config.mongodb_uri);

        // Create DataService with database reference
        let data_service = web::Data::new(services::data_service::DataService::new(&db));

        Ok(Self { data_service })
    }
}
