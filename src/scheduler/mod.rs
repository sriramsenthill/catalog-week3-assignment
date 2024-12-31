// scheduler/mod.rs
use crate::db::Database;
use chrono::Utc;
use std::sync::Arc;
use std::time::Duration;
use tokio::time;

const INITIAL_TIMESTAMP: i64 = 1647910800; // Your specified start time

pub async fn start_scheduler(db: Arc<Database>) {
    // First, do an initial fetch of historical data if needed
    if let Err(e) = db.fetch_and_store_data(INITIAL_TIMESTAMP).await {
        eprintln!("Error in initial historical data fetch: {}", e);
    }

    let mut interval = time::interval(Duration::from_secs(3600)); // 1 hour
    let mut retry_delay = Duration::from_secs(60);

    loop {
        interval.tick().await;

        // For subsequent updates, start from the last hour
        match db.fetch_and_store_data(Utc::now().timestamp() - 3600).await {
            Ok(_) => {
                println!("Scheduler: Successfully fetched and stored data");
                retry_delay = Duration::from_secs(60);
            }
            Err(e) => {
                eprintln!("Scheduler: Error in scheduled data fetch: {}", e);
                eprintln!("Scheduler: Will retry in {} seconds", retry_delay.as_secs());
                time::sleep(retry_delay).await;
                retry_delay =
                    std::cmp::min(retry_delay.saturating_mul(2), Duration::from_secs(900));
            }
        }
    }
}
