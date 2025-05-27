use axum::{Router, http::{Method, header, HeaderValue}};
use tower_http::cors::CorsLayer;

pub fn api() -> Router {
    let cors = CorsLayer::new()
        .allow_origin(HeaderValue::from_static("http://localhost:3000"))
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS])
        .allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE, header::ACCEPT])
        .allow_credentials(true);

    Router::new()
        .merge(super::hello_world::hello_world_routes())
        .layer(cors)
}