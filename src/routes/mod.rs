use crate::handlers::data_handler::get_data;
use crate::models::{
    collection_type::CollectionType, depth_model::Depth, earnings_model::Earnings,
    runepools_model::RunePool, swaps_model::Swaps,
};
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route(
                "/depths",
                web::get().to(|q, s| get_data::<Depth>(CollectionType::Depths, q, s)),
            )
            .route(
                "/swaps",
                web::get().to(|q, s| get_data::<Swaps>(CollectionType::Swaps, q, s)),
            )
            .route(
                "/runepools",
                web::get().to(|q, s| get_data::<RunePool>(CollectionType::Runepools, q, s)),
            )
            .route(
                "/earnings",
                web::get().to(|q, s| get_data::<Earnings>(CollectionType::Earnings, q, s)),
            ),
    );
}
