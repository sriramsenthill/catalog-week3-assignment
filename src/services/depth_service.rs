use crate::db::depth_db::DepthDB;
use crate::models::{depth_model::Depth, query_params::QueryParams};

pub struct DepthService {
    db: DepthDB,
}

impl DepthService {
    pub fn new(db: DepthDB) -> Self {
        Self { db }
    }

    pub async fn get_depths(
        &self,
        params: &QueryParams,
    ) -> Result<Vec<Depth>, mongodb::error::Error> {
        self.db.find_depths(params).await
    }
}
