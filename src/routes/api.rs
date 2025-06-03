use axum::{Router, http::{Method, header, HeaderValue}};
use tower_http::cors::CorsLayer;
use std::sync::Arc;
use crate::app::http::core::app_state::AppState;

pub fn api(state: Arc<AppState>) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(HeaderValue::from_static("http://localhost:3000"))
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS])
        .allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE, header::ACCEPT])
        .allow_credentials(true);

    Router::<Arc<AppState>>::new()
        .merge(super::hello_world::hello_world_routes().with_state(state.clone()))
        .merge(super::users::users_routes().with_state(state.clone()))
        .layer(cors)
        .with_state(state)
}