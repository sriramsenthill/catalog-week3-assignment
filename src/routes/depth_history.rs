// routes/depth_history.rs
use crate::db::Database;
use crate::schema::{DepthHistory, HistoryParams};
use axum::{extract::Extension, extract::Query, response::Json};
use std::sync::Arc;

pub async fn get_depth_history(
    Query(params): Query<HistoryParams>,
    Extension(db): Extension<Arc<Database>>,
) -> Json<Vec<DepthHistory>> {
    let data = db.get_depth_history(&params).await.unwrap();
    Json(data)
}
