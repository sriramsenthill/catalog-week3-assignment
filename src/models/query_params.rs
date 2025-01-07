use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct QueryParams {
    pub date_range: Option<String>,
    pub sort_by: Option<String>,
    pub order: Option<String>,
    pub limit: Option<i64>,
}
