use axum::{routing::post, Router};
use crate::app::http::controllers::users;

pub fn users_routes() -> Router {
    Router::new()
        .route("/api/users/user-get", post(users::user_get))
}
