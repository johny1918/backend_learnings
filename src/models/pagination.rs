use serde::{Deserialize};
use axum::{Json, extract::Query};

use crate::{errors::ResponseErrors, models::products::ProductOutput};

#[derive(Deserialize)]
pub struct Pagination {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}

//Apply pagination in DTO
impl Pagination {
    pub fn normalize(self) -> (u32, u32) {
        let page = self.page.unwrap_or(1).max(1);
        let per_page = self.per_page.unwrap_or(20).clamp(1, 100);
        (page, per_page)
    }
}

//Apply pagination in Handler
pub async fn list_items(Query(params): Query<Pagination>) 
-> Result<Json<String>, ResponseErrors> {
    let (page, per_page) = params.normalize();
    
    //fetch data from core function based on page, per_page

    Ok(Json("Your data".to_string()))
}

// Apply pagination in SQLx based on page,per_page.