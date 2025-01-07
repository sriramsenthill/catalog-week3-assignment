use crate::models::query_params::QueryParams;
use bson::{doc, Document};
use futures::stream::TryStreamExt;
use mongodb::{error::Error as MongoError, Collection, Database};
use serde::de::DeserializeOwned;

pub struct BaseDB {
    collection: Collection<Document>,
}

impl BaseDB {
    const DEFAULT_LIMIT: i64 = 24;
    const MAX_RECORDS_NO_FILTER: i64 = 400;

    pub fn new(db: &Database, collection_name: &str) -> Self {
        Self {
            collection: db.collection(collection_name),
        }
    }

    pub async fn find_documents<T>(&self, params: &QueryParams) -> Result<Vec<T>, MongoError>
    where
        T: DeserializeOwned,
    {
        let pipeline = self.build_pipeline(params);
        self.execute_pipeline(pipeline).await
    }

    fn build_pipeline(&self, params: &QueryParams) -> Vec<Document> {
        if params.date_range.is_none() && params.sort_by.is_none() {
            return vec![doc! { "$limit": Self::MAX_RECORDS_NO_FILTER }];
        }

        let mut pipeline = Vec::new();

        if let Some(match_stage) = crate::utils::build_match_stage(&params.date_range) {
            pipeline.push(match_stage);
        }

        if let Some(sort_stage) = crate::utils::build_sort_stage(&params.sort_by, &params.order) {
            pipeline.push(sort_stage);
        }

        let limit = params
            .limit
            .unwrap_or(Self::DEFAULT_LIMIT)
            .min(Self::MAX_RECORDS_NO_FILTER) as i64;
        pipeline.push(doc! { "$limit": limit });

        pipeline
    }

    async fn execute_pipeline<T>(&self, pipeline: Vec<Document>) -> Result<Vec<T>, MongoError>
    where
        T: DeserializeOwned,
    {
        let mut cursor = self.collection.aggregate(pipeline, None).await?;
        let mut results = Vec::new();

        while let Some(doc_result) = cursor.try_next().await? {
            if let Ok(item) = bson::from_document(doc_result) {
                results.push(item);
            }
        }

        Ok(results)
    }
}
