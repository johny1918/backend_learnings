use std::sync::Arc;

use axum::{Router, routing::{get, post}};
use crate::services::products::*;
use crate::database::AppState;

pub fn product_routes() -> Router<Arc<AppState>> {
    Router::new()
                .route("/products", post(create_product)
                                                        .get(list_products))
                .route("/products/{id}", get(get_product)
                                                        .put(update_product)
                                                        .delete(delete_product))
}