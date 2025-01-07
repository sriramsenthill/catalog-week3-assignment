// utils/pagination.rs
use crate::models::query_params::QueryParams;
use bson::doc;

pub fn add_pagination_stages(pipeline: &mut Vec<bson::Document>, params: &QueryParams) {
    let page = params.page.unwrap_or(1).max(1);
    let limit = params.limit.unwrap_or(10).max(1);
    let skip = (page - 1) * limit;
    pipeline.push(doc! { "$skip": skip });
    pipeline.push(doc! { "$limit": limit });
}
