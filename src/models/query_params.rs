use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct QueryParams {
    pub interval: Option<String>,
    pub date_range: Option<String>,
    pub sort_by: Option<String>,
    pub order: Option<String>,
    pub limit: Option<i64>,
    pub page: Option<i64>,
}
