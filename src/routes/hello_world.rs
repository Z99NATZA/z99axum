use axum::{routing::get, Router};
use crate::app::http::controllers::hello_world;

pub fn hello_world_routes() -> Router {
    Router::new()
        .route("/hello-world", get(hello_world::get_hello_world))
}
