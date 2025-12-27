use axum::extract::State;
use crate::AppState;
use std::sync::Arc;

pub async fn welcome_page(State(state): State<Arc<AppState>>) -> String {
    let mut num = state.counter.lock().unwrap();
    *num += 1;
    format!("Welcome to {}, you have visited this page {} times", state.app_name, *num)
}