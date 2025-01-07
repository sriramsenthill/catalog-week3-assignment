use dotenv::dotenv;
use std::env;
use std::net::SocketAddr;

#[derive(Debug)]
pub struct AppConfig {
    pub mongodb_uri: String,
    pub database_name: String,
    pub server_addr: SocketAddr,
}

#[derive(Debug)]
pub struct AppError(pub String);

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for AppError {}

impl AppConfig {
    pub fn from_env() -> Result<Self, AppError> {
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
