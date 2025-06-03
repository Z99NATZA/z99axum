use std::sync::Arc;

use axum::{routing::get, Router};
use crate::app::http::{controllers::hello_world, core::app_state::AppState};

pub fn hello_world_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/hello-world", get(hello_world::get_hello_world))
}
