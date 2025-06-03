use std::sync::Arc;

use axum::{routing::{get, post}, Router};
use crate::app::http::{controllers::users, core::app_state::AppState};

pub fn users_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/api/users/user-get-config", post(users::user_get))
        .route("/api/users/user-get-one/{id}", get(users::user_get2))
}
