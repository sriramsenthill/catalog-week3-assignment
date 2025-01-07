use crate::models::{collection_type::CollectionType, query_params::QueryParams};
use crate::services::data_service::DataService;
use actix_web::{web, HttpResponse};
use serde::de::DeserializeOwned;

pub async fn get_data<T>(
    collection_type: CollectionType,
    query: web::Query<QueryParams>,
    service: web::Data<DataService>,
) -> HttpResponse
where
    T: serde::Serialize + DeserializeOwned,
{
    match service.get_data::<T>(collection_type, &query).await {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(e) => {
            log::error!(
                "Failed to fetch data from {}: {}",
                collection_type.as_str(),
                e
            );
            HttpResponse::InternalServerError().finish()
        }
    }
}
