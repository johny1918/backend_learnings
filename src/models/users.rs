use axum::{http::StatusCode, response::IntoResponse};
use serde::{Serialize, Deserialize};
use axum::Json;

#[derive(Serialize)]
pub struct Message {
    pub text: String,
}

#[derive(Deserialize)]
pub struct UserInput {
    pub username: String,
    pub age: u8,
}

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
}

impl <T> IntoResponse for ApiResponse<T> where T: Serialize {
    fn into_response(self) -> axum::response::Response {
        let status = if self.success {
            StatusCode::OK
        }
        else {
            StatusCode::BAD_REQUEST
        };
        (status, Json(self)).into_response()
    }
}