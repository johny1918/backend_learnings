use axum::{Json, http::{StatusCode}, response::IntoResponse};
use serde::Serialize;
use axum::response::Response;
use thiserror::Error;


#[derive(Serialize, Debug)]
pub struct AppError <T> {
    pub error: String,
    pub details: Option<T>,
}


// thiserror define application custom errors

#[derive(Error, Debug)]
pub enum ResponseErrors {
    #[error("Resource not found: {0}")]
    NotFound(String),
    #[error("Invalid input: {0}")]
    BadRequest(String),
    #[error("Unexpected server error: {0}")]
    Internal(String),
}

impl IntoResponse for ResponseErrors {
    fn into_response(self) -> Response {
        match self {
            ResponseErrors::NotFound(msg) => 
                (StatusCode::NOT_FOUND, Json(msg)).into_response(),
            ResponseErrors::BadRequest(msg) => 
                (StatusCode::BAD_REQUEST, Json(msg)).into_response(),
            ResponseErrors::Internal(msg) => 
                (StatusCode::INTERNAL_SERVER_ERROR, Json(msg)).into_response()

        }
    }
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