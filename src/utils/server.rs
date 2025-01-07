use crate::{routes, state::AppState, utils::config::AppConfig};
use actix_web::{App, HttpServer};

pub async fn setup_app(
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
