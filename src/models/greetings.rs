use axum::extract::State;
use crate::AppState;
use std::sync::Arc;

pub async fn welcome_page(State(state): State<Arc<AppState>>) -> String {
    format!("Welcome to {}", state.app_name)
}