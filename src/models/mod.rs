pub mod functions;
pub mod greetings;
pub mod users;
pub mod search;

use users::Message;
use users::UserInput;
use functions::Output;
use search::SearchQuery;
use axum_extra::headers::UserAgent;

use axum::Json;
use axum::extract::{Path, Query};
use axum_extra::TypedHeader;



pub async fn get_items() -> Json<Message> {
    Json(Message {
        text: "Listing items".to_string(),
    })
}

pub async fn create_items() -> Json<Message> {
    Json( Message { text: "Creating an item".to_string() })
}

pub async fn get_user(Path(user_id): Path<i32>) -> Json<Message> {
    Json( Message { text: format!("Welcome user id: {}", user_id) })
}

pub async fn create_user(Json(payload): Json<UserInput>) -> String {
    format!("User {} is {} years old.", payload.username, payload.age)
}

pub async fn greet(Path(name): Path<String>) -> String {
    format!("Hello, and welcome {}", name)
}

pub async fn square(Path(n): Path<i32>) -> Json<Output> {
    Json(Output { result: n * 2 })
}

// Query is good when you need to filter, search, pagination and optional parameters
pub async fn search(Query(params): Query<SearchQuery>) -> String {
    format!(
        "Search for '{}' with limit {}", params.term, params.limit.unwrap_or(0)
    )
}

pub async fn get_user_agent(user_agent: Option<TypedHeader<UserAgent>>) -> String {
    match user_agent {
        Some(agent) => format!("Your user agent is: {}", agent.as_str()),
        None => "No user agent provided".to_string()
    }
    
}