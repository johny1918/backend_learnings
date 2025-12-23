use axum::{Json, Router, routing::{get, post}};
use serde::{Deserialize, Serialize};
use axum::extract::{Path, Query};

#[derive(Serialize)]
struct Message {
    text: String,
}

#[derive(Serialize)]
struct Output {
    result: i32,
}

#[derive(Deserialize)]
struct SearchQuery {
    term: String,
    limit: Option<u32>,
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

async fn greet(Path(name): Path<String>) -> String {
    format!("Hello, and welcome {}", name)
}

async fn square(Path(n): Path<i32>) -> Json<Output> {
    Json(Output { result: n * 2 })
}

// Query is good when you need to filter, search, pagination and optional parameters
async fn search(Query(params): Query<SearchQuery>) -> String {
    format!(
        "Search for '{}' with limit {}", params.term, params.limit.unwrap_or(0)
    )
}

#[tokio::main]
async fn main() {

    //Route for Users
    let user_routes = Router::new()
                    .route("/users", get(get_items).post(create_items))
                    .route("/users/{user_id}", get(get_user));

    //Route for greetings
    let greeting_routes = Router::new()
    .route("/greet/{name}", get(greet));

    //Route for square values
    let square_routes = Router::new()
    .route("/{n}", post(square));

    //Routing for search
    let search_routes = Router::new()
    .route("/", get(search));

    //Nesting users under api path.
    let app = Router::new()
                    .nest("/welcome", greeting_routes)
                    .nest("/api", user_routes)
                    .nest("/search", search_routes)
                    .nest("/calculate-square", square_routes);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Failed to bind");
    println!("Server running at http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.expect("Failed to start the axum server");
}
