pub mod functions;
pub mod greetings;
pub mod users;
pub mod search;
pub mod pagination;
pub mod config;
pub mod products;
pub mod sorting;
pub mod filter;
pub mod jwt;

use axum::body::Body;
use axum::http::Response;
use axum::http::header::CONTENT_TYPE;
use axum::response::IntoResponse;
use jsonwebtoken::DecodingKey;
use jsonwebtoken::Validation;
use jsonwebtoken::decode;
use users::Message;
use users::UserInput;
use functions::Output;
use search::SearchQuery;
use axum_extra::headers::UserAgent;
use axum::extract::State;
use std::sync::Arc;
use crate::errors::ResponseErrors;
use crate::models::jwt::Claims;

use axum::Json;
use axum::extract::{Path, Query};
use axum_extra::TypedHeader;
use axum::http::StatusCode;
use crate::database::AppState;
use crate::models::users::ApiResponse;
use jsonwebtoken::{encode, Header, EncodingKey};
use time::{OffsetDateTime, Duration};



pub async fn get_items() -> Json<Message> {
    Json(Message {
        text: "Listing items".to_string(),
    })
}

pub async fn consisten_response() -> impl IntoResponse {

    ApiResponse {
        success: true,
        data: Some("Everything worked"),
        message: None,
    }
}

pub async fn create_items() -> (StatusCode, Json<Message>) {
    (StatusCode::CREATED, Json( Message { text: "Creating an item".to_string() }))
}

pub async fn get_user(Path(user_id): Path<i32>) -> Result<Json<Message>, ResponseErrors> {
    if user_id > 10 {
        return Err(ResponseErrors::Internal("Id not found".into()));
    }
    Ok(Json( Message { text: format!("Welcome user id: {}", user_id) }))
}

pub async fn create_user(Json(payload): Json<UserInput>) -> String {
    format!("User {} is {} years old.", payload.username, payload.age)
}

pub async fn greet(State(state): State<Arc<AppState>>, Path(name): Path<String>) -> String {
    let mut num = state.counter.lock().unwrap();
    *num +=1;
    format!("Hello, and welcome {}, Counter: {}", name, *num)
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

//Manual response, useful when want full control using trait Reponse/IntoResponse
pub async fn html_page() -> Response<Body> {
    (StatusCode::OK,
    [(CONTENT_TYPE, "text/html")],
    "<h1>Hello World!</h1>",
    ).into_response()
}

pub fn create_jwt(user_id: String, role: String, secret: &str) -> String {
    let now = OffsetDateTime::now_utc();
    let exp = now + Duration::hours(1);

    let claims = Claims {
        sub: user_id,
        role,
        iat: now.unix_timestamp() as usize,
        exp: exp.unix_timestamp() as usize,
    };

    encode(&Header::default(), 
           &claims, 
           &EncodingKey::from_secret(secret.as_ref())
    ).expect("Failed to encode JWT")
}

pub fn validate_jwt(token: &str, secret: &str) -> Result<Claims, ResponseErrors> {
    let validation = Validation::default();
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &validation,)
        .map_err(|_| ResponseErrors::BadRequest("Failed to validate jwt".to_string()))?;
    
    Ok(token_data.claims)
}