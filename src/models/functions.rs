use serde::Serialize;

#[derive(Serialize)]
pub struct Output {
    pub result: i32,
}

#[derive(Clone)]
pub struct AppState {
    pub app_name: String,
}
