use serde::Deserialize;

#[derive(Deserialize)]
pub struct SearchQuery {
    pub term: String,
    pub limit: Option<u32>,
}