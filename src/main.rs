use axum::{Json, Router, routing::{get}};
use serde::Serialize;
use axum::extract::Path;

#[derive(Serialize)]
struct Message {
    text: String,
}

async fn get_items() -> Json<Message> {
    Json(Message {
        text: "Listing items".to_string(),
    })
}

async fn create_items() -> Json<Message> {
    Json( Message { text: "Creating an item".to_string() })
}

async fn get_user(Path(user_id): Path<i32>) -> Json<Message> {
    Json( Message { text: format!("Welcome user id: {}", user_id) })
}

#[tokio::main]
async fn main() {

    let user_routes = Router::new()
                    .route("/users", get(get_items).post(create_items))
                    .route("/users/{user_id}", get(get_user));
    //Nesting users under api path.
    let app = Router::new()
                    .nest("/api", user_routes);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Failed to bind");
    println!("Server running at http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.expect("Failed to start the axum server");
}
