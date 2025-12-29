use serde::{Deserialize, Serialize};

// This struct represents what my application acts on.
#[derive(Debug, Clone)]
pub struct Product {
    pub id: i64,
    pub name: String,
    pub price: f64,
    pub description: Option<String>,
}

// DTO
#[derive(Deserialize)]
pub struct CreateProductInput {
    pub name: String,
    pub price: f64,
    pub description: Option<String>,
}

// DTO
#[derive(Deserialize)]
pub struct UpdateProductInput {
    pub name: Option<String>,
    pub price: Option<f64>,
    pub description: Option<String>,
}

#[derive(Serialize)]
pub struct ProductOutput {
    pub id: i64,
    pub name: String,
    pub price: f64,
    pub description: Option<String>,
}

// This protects my API from leaking internal fields such as hidden flags or sensitive values.
impl From<Product> for ProductOutput {
    fn from(value: Product) -> Self {
        Self {
            id: value.id,
            name: value.name,
            price: value.price,
            description: value.description,
        }
    }
}