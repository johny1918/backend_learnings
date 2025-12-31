use sqlx::PgPool;
use std::sync::{Mutex,Arc};



#[derive(Clone, Debug)]
pub struct AppState {
    pub db: PgPool,
    pub app_name: String,
    pub counter: Arc<Mutex<u32>>,
    pub app_version: String,
}

pub async fn create_pool(database_url: &str) -> PgPool {
    PgPool::connect(database_url)
    .await
    .expect("Failed to create Postgres pool")
}