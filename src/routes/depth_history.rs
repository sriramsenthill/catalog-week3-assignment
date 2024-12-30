use crate::db::get_depth_history_from_db;
use crate::schema::DepthHistory;
use axum::{extract::Query, Json};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct DepthHistoryQueryParams {
    page: Option<u32>,
    limit: Option<u32>,
}

pub async fn get_depth_history(
    Query(params): Query<DepthHistoryQueryParams>,
) -> Json<Vec<DepthHistory>> {
    let data = get_depth_history_from_db().await.unwrap();
    Json(data)
}
