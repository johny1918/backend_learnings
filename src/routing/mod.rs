use axum::Router;
use crate::models::greetings::welcome_page;
use crate::models::{pagination::list_items, *};
use crate::errors::AppError;
use axum::routing::{get, post};
use tower_http::timeout::TimeoutLayer;
use std::time::Duration;
use axum::http::StatusCode;


pub fn router_logic() -> Result<Router, AppError<String>>{

    //Route for Users
    let user_routes = Router::new()
                    .route("/users", get(get_items).post(create_items))
                    .route("/users/{user_id}", get(get_user))
                    .route("/users/create/{data}", post(create_user))
                    .route("/users/agent", get(get_user_agent))
                    .route("/users/html", get(html_page))
                    .route("/users/resp", get(consisten_response));

    // Router for response pagination
    let pagination = Router::new()
                    .route("/", get(list_items));

    //Route for greetings
    let greeting_routes = Router::new()
    .route("/greet/{name}", get(greet));

    //Route for square values
    let square_routes = Router::new()
    .route("/{n}", post(square));

    //Routing for search
    let search_routes = Router::new()
    .route("/", get(search));

    //Nesting users under api path.
    let app = Router::new()
                    .route("/", get(welcome_page))
                    .nest("/welcome", greeting_routes)
                    .nest("/api", user_routes)
                    .nest("/search", search_routes)
                    .nest("/calculate-square", square_routes)
                    .nest("/page", pagination)
                    .layer(TimeoutLayer::with_status_code(StatusCode::REQUEST_TIMEOUT, Duration::from_secs(5)));
    Ok(app)
}
    
