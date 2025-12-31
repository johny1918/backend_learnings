use std::fmt::{Debug, Display};
use std::sync::Arc;
use axum::{extract::State, http::StatusCode, response::IntoResponse};
use serde::{Serialize, Deserialize};
use axum::Json;

use crate::{database::AppState, errors::ResponseErrors, models::create_jwt};

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

#[derive(Debug,Serialize,Deserialize)]
pub enum Roles {
    Guest,
    User,
    Admin,
}

#[derive(Debug,Serialize,Deserialize)]
pub struct UserCredentials {
    pub username: String,
    pub password: String,
    pub role: Roles,
}

impl Display for Roles {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Roles::Guest => write!(f,"Guest"),
            Roles::User => write!(f, "User"),
            Roles::Admin => write!(f, "Admin"),
        }
    }
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

pub async fn login(State(state):State<Arc<AppState>>,
                Json(input): Json<UserCredentials>) 
                -> (StatusCode, String) {
    let user_data = state.validate_credentials(input).await.map_err(|msg| {
        ResponseErrors::NotFound("Failed to validate credentials".to_string());
    });

    match user_data {
        Ok(token) => {
            let _token = create_jwt(
                        token.username,
                        token.role.to_string(), 
                        "secretkeythatisverylongandshouldnotbeexposed");
        },
        Err(e) => { println!("Failed to get create jwt {:?}", e); }
    };
    
   
    
    (StatusCode::OK, "Login successfully".to_string())
}