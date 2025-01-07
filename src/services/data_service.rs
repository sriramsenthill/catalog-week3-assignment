use crate::db::base_db::BaseDB;
use crate::models::{collection_type::CollectionType, query_params::QueryParams};
use mongodb::Database;
use serde::de::DeserializeOwned;

pub struct DataService {
    depths_db: BaseDB,
    swaps_db: BaseDB,
    runepools_db: BaseDB,
    earnings_db: BaseDB,
}

impl DataService {
    pub fn new(db: &Database) -> Self {
        Self {
            depths_db: BaseDB::new(db, CollectionType::Depths.as_str()),
            swaps_db: BaseDB::new(db, CollectionType::Swaps.as_str()),
            runepools_db: BaseDB::new(db, CollectionType::Runepools.as_str()),
            earnings_db: BaseDB::new(db, CollectionType::Earnings.as_str()),
        }
    }

    pub async fn get_data<T>(
        &self,
        collection_type: CollectionType,
        params: &QueryParams,
    ) -> Result<Vec<T>, mongodb::error::Error>
    where
        T: DeserializeOwned,
    {
        match collection_type {
            CollectionType::Depths => self.depths_db.find_documents(params),
            CollectionType::Swaps => self.swaps_db.find_documents(params),
            CollectionType::Runepools => self.runepools_db.find_documents(params),
            CollectionType::Earnings => self.earnings_db.find_documents(params),
        }
        .await
    }
}
