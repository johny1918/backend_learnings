use sqlx::{PgPool, postgres::PgPoolOptions};
use std::sync::{Mutex,Arc};



#[derive(Clone, Debug)]
pub struct AppState {
    pub db: PgPool,
    pub app_name: String,
    pub counter: Arc<Mutex<u32>>,
    pub app_version: String,
}

pub async fn create_pool(database_url: &str) -> PgPool {
    PgPoolOptions::new()
    .max_connections(20) // max numbers of active connections to pool
    .min_connections(5) // the pool will try to maintain this idle connection count
    .idle_timeout(Some(std::time::Duration::from_secs(300))) // how long idle connections stay open before closed
    .connect(database_url)
    .await
    .expect("Failed to connect to database")
}