use std::{env, time::Duration};

use tokio::task::JoinHandle;

use crate::{
    app_context::AppContext,
    lightning_fetcher::{self, LightningResponse},
};

pub fn ingest_data(context: AppContext, data: Vec<LightningResponse>) {}

/// This function will spawn a new thread that will fetch the data from lightning network
pub fn run_ingestor(context: AppContext) -> JoinHandle<()> {
    let interval_in_secs = env::var("FETCH_INTERNVAL_IN_SECS")
        .expect("msg")
        .parse::<u64>()
        .expect("Please provide a valid i32 in FETCH_INTERVAL_IN_SECS");

    println!("Ingesting data with interval of {} secs", interval_in_secs);
    let interval_in_secs = Duration::from_secs(interval_in_secs);

    tokio::spawn(async move {
        loop {
            let data = lightning_fetcher::fetch_data().await;
            if data.is_ok() {
                ingest_data(context.clone(), data.unwrap());
            } else {
                println!(
                    "An error occurred when fetching data: {}",
                    data.unwrap_err()
                );
            }

            tokio::time::sleep(interval_in_secs).await;
        }
    })
}
