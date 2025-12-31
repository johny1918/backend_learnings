use serde::Deserialize;


// DTO example for filtering
#[derive(Deserialize)]
pub struct ProductFilter {
    pub name: Option<String>,
    pub min_price: Option<f64>,
    pub max_price: Option<f64>,
    pub in_stock: Option<bool>,
}

// pub async fn list_with_filters(&self, page: u32, per_page: u32, filters: ProductFilter) {
//     let mut conditions = Vec::new();
//     let mut params: Vec<Box<dyn sqlx::Encode<'_,sqlx::Postgres> + Send + Sync>> = Vec::new();

//     if let Some(name) = filters.name {
//         conditions.push(format!("name ILIKE'%'||${}||'%'", params.len()+1));
//         params.push(Box::new(name));
//     }

//     if let Some(min) = filters.min_price {
//         conditions.push(format!("price >=${}", params.len()+1));
//         params.push(Box::new(min));
//     }

//     // so on for all other files.
// }