// db/depth_db.rs
use crate::models::{depth_model::Depth, query_params::QueryParams};
use crate::utils::{add_pagination_stages, build_group_stage, build_match_stage, build_sort_stage};
use bson::{doc, from_document, Document};
use futures::stream::TryStreamExt;
use mongodb::{error::Error as MongoError, Collection, Database};

const DEFAULT_LIMIT: i64 = 10;
const MAX_RECORDS_NO_FILTER: i64 = 400;

pub struct DepthDB {
    collection: Collection<Document>,
}

impl DepthDB {
    pub fn new(db: &Database) -> Self {
        let collection = db.collection("depths");
        Self { collection }
    }

    pub async fn find_depths(&self, params: &QueryParams) -> Result<Vec<Depth>, MongoError> {
        let pipeline = self.build_pipeline(params);
        log::debug!("Final pipeline: {:?}", pipeline);
        self.execute_pipeline(pipeline).await
    }

    fn build_pipeline(&self, params: &QueryParams) -> Vec<Document> {
        if self.should_use_simple_query(params) {
            return vec![doc! { "$limit": MAX_RECORDS_NO_FILTER }];
        }

        let mut pipeline = Vec::new();

        if let Some(match_stage) = build_match_stage(&params.date_range) {
            pipeline.push(match_stage);
        }

        if let Some(group_stage) = build_group_stage(&params.interval) {
            pipeline.push(group_stage);
        }

        if let Some(sort_stage) = build_sort_stage(&params.sort_by, &params.order) {
            pipeline.push(sort_stage);
        }

        add_pagination_stages(&mut pipeline, params);
        pipeline
    }

    fn should_use_simple_query(&self, params: &QueryParams) -> bool {
        params.date_range.is_none() && params.interval.is_none() && params.sort_by.is_none()
    }

    async fn execute_pipeline(&self, pipeline: Vec<Document>) -> Result<Vec<Depth>, MongoError> {
        let mut cursor = self.collection.aggregate(pipeline, None).await?;
        let mut depths = Vec::new();

        while let Some(doc_result) = cursor.try_next().await? {
            log::debug!("Fetched document: {:?}", doc_result);
            if let Ok(depth) = from_document(doc_result) {
                depths.push(depth);
            }
        }

        log::debug!("Depths found: {:?}", depths);
        Ok(depths)
    }
}
