mod routing;
mod models;
mod errors;
mod config;
mod services;

use std::sync::{Arc, Mutex};

use tracing_subscriber;
use crate::{models::{config::AppConfig, functions::AppState}, routing::router_logic};

#[tokio::main]
async fn main() {
    
    //Load environment variables from .env(if present)
    dotenvy::dotenv().ok();
    let config = AppConfig::from_env();


    // Initialize tracing subscriber for console logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let routes_counter = Arc::new(Mutex::new(0));
    let state = Arc::new(AppState{ 
        app_name: "My Axum App".to_string(),
        counter: routes_counter.clone(),
        app_version: "v1.0".to_string(),
});
    
    let app = router_logic(state).expect("Failed to get routes");
    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", &config.port))
        .await
        .expect("Failed to bind");
    println!("Server running at http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.expect("Failed to start the axum server");
}
