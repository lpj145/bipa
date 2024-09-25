use mongodb::{Client, Database};

// This module will shared context data to functions
// Like mongodb connection
#[derive(Clone)]
pub struct AppContext {
    pub database: Database,
}

impl AppContext {
    pub fn new(client: Client) -> Self {
        let database = client.database("lightning-nodes");
        AppContext { database }
    }
}
