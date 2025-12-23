use axum::Router;
use crate::models::*;

use axum::routing::{get, post};


pub async fn router_logic() -> Result<Router, std::io::Error>{

    //Route for Users
    let user_routes = Router::new()
                    .route("/users", get(get_items).post(create_items))
                    .route("/users/{user_id}", get(get_user));

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
                    .nest("/welcome", greeting_routes)
                    .nest("/api", user_routes)
                    .nest("/search", search_routes)
                    .nest("/calculate-square", square_routes);
    Ok(app)
}
    
