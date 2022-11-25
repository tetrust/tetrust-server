use std::sync::Arc;

use mongodb::options::ClientOptions;
use mongodb::{Client, Database};

pub struct MongoClient {}

impl MongoClient {
    pub async fn get_database(database_name: &str) -> Arc<Database> {
        let mongodb_url = std::env::var("MONGODB_URL").expect("MONGODB_URL Not Found");

        let client_options = ClientOptions::parse(mongodb_url)
            .await
            .expect("parse failed");

        let client = Client::with_options(client_options).expect("connection failed");

        let database = client.database(database_name);

        Arc::new(database)
    }
}
