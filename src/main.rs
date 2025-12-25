mod routing;
mod models;
mod errors;

use tracing_subscriber;
use crate::routing::router_logic;

#[tokio::main]
async fn main() {

    // Initialize tracing subscriber for console logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    
    let app = router_logic().expect("Failed to get routes");
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Failed to bind");
    println!("Server running at http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.expect("Failed to start the axum server");
}
