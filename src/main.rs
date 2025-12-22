use axum::{Json, Router, routing::get};
use serde::Serialize;

#[derive(Serialize)]
struct Message {
    text: String,
}

async fn hello_world() -> Json<Message> {
    Json(Message {
        text: "Hello world!".to_string(),
    })
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(hello_world));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Failed to bind");
    println!("Server running at http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.expect("Failed to start the axum server");
}
