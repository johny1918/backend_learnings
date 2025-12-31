pub mod products;

use axum::Router;
use crate::database::AppState;
use crate::models::greetings::welcome_page;
use crate::models::{pagination::list_items, *};
use crate::errors::AppError;
use crate::routing::products::product_routes;
use axum::routing::{get, post};
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;
use std::sync::Arc;
use std::time::Duration;
use axum::http::StatusCode;
use crate::models::users::login;


pub fn router_logic(state: Arc<AppState>) -> Result<Router, AppError<String>>{

    //Route for Users
    let user_routes = Router::new()
                    .route("/users", get(get_items).post(create_items))
                    .route("/users/{user_id}", get(get_user))
                    .route("/users/create/{data}", post(create_user))
                    .route("/users/agent", get(get_user_agent))
                    .route("/users/html", get(html_page))
                    .route("/users/resp", get(consisten_response));
    
    let users_login = Router::new().route("/", post(login));


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
                    .nest("/shop", product_routes())
                    .nest("/login", users_login)
                    .layer(TimeoutLayer::with_status_code(StatusCode::REQUEST_TIMEOUT, Duration::from_secs(5)))
                    .layer(TraceLayer::new_for_http())
                    .with_state(state);
    Ok(app)
}
    
