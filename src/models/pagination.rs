use serde::Serialize;
use axum::Json;

#[derive(Serialize)]
pub struct Pagination<T> {
    pub items: Vec<T>,
    pub total: u64,
    pub page: u32,
    pub per_page: u32,
}

#[derive(Serialize)]
pub struct Item {
    pub id: u32,
    pub name: String,
}

pub async fn list_items() -> Json<Pagination<Item>> {
    Json(
        Pagination {
            items: vec![
                Item { id: 1, name: "Book".to_string()},
                Item { id: 2, name: "Pen".to_string()},
            ],
            total: 42,
            page: 1,
            per_page: 10,
        }
    )
}