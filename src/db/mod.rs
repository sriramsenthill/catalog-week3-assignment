use mongodb::{Client, Database};
pub mod base_db;

pub async fn init_db(uri: &str, db_name: &str) -> Result<Database, mongodb::error::Error> {
    let client = Client::with_uri_str(uri).await?;
    Ok(client.database(db_name))
}
