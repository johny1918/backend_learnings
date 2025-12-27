use std::sync::{Arc, Mutex};

use serde::Serialize;

#[derive(Serialize)]
pub struct Output {
    pub result: i32,
}

#[derive(Clone, Debug)]
pub struct AppState {
    pub app_name: String,
    pub counter: Arc<Mutex<u32>>,
}
