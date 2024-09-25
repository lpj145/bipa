use std::{env, time::Duration};

use tokio::task::JoinHandle;

use crate::{
    app_context::AppContext,
    lightning_fetcher::{self, LightningResponse},
    node_data::NodeDocument,
};

pub async fn ingest_data(context: AppContext, response: Vec<LightningResponse>) {
    let node_collection = context.database.collection::<NodeDocument>("nodes");
    let mut documents: Vec<NodeDocument> = Vec::with_capacity(10);

    println!("Parsing data as node format");

    for data in response {
        if let Ok(d) = NodeDocument::try_from(data) {
            documents.push(d);
        }
    }

    println!("Saving data on mongodb");
    let result = node_collection.insert_many(documents).await;
    if result.is_err() {
        println!(
            "Unable to save data on mongo, error: {}",
            result.unwrap_err()
        );
    }
}

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
                ingest_data(context.clone(), data.unwrap()).await;
            } else {
                println!(
                    "An error occurred when fetching data: {}",
                    data.unwrap_err()
                );
            }

            println!("Waiting for {:?} secs", interval_in_secs);
            tokio::time::sleep(interval_in_secs).await;
        }
    })
}
