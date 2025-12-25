use axum::{Json, http::{StatusCode}, response::IntoResponse};
use serde::Serialize;
use axum::response::Response;


#[derive(Serialize, Debug)]
pub struct AppError <T> {
    pub error: String,
    pub details: Option<T>,
}

impl <T> IntoResponse for AppError<T> where T: Serialize {
    fn into_response(self) -> Response {
        let status: StatusCode = if self.error.is_empty() && self.details.is_none() {
            StatusCode::OK
        } 
        else {
            StatusCode::BAD_REQUEST
        };
        (status, Json(self)).into_response()
    }
}