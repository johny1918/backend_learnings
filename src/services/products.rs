use std::sync::Arc;

use axum::{Json, extract::State};
use crate::{errors::AppError, models::{functions::AppState, products::*}};

pub async fn create_product(State(state): State<Arc<AppState>>,
Json(input): Json<CreateProductInput>) 
    -> Result<Json<ProductOutput>, AppError<String>> {
    if !state.app_version.contains("v1.0") {
        return Err(AppError { error: "Version is wrong".to_string(), details: None });
    }
    Ok(Json(ProductOutput { id: 123, 
        name: "Test".to_string(), 
        price: 52.5, 
        description: Some("This is a test".to_string()) }))
}

pub async fn list_products() {
    todo!()
}

pub async fn get_product() {
    todo!()
}

pub async fn update_product() {
    todo!()
}

pub async fn delete_product() {
    todo!()
}