use crate::models::query_params::QueryParams;
use crate::services::depth_service::DepthService;
use actix_web::{web, HttpResponse};

pub async fn get_depths(
    query: web::Query<QueryParams>,
    service: web::Data<DepthService>,
) -> HttpResponse {
    match service.get_depths(&query).await {
        Ok(depths) => HttpResponse::Ok().json(depths),
        Err(e) => {
            log::error!("Failed to fetch depths: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
