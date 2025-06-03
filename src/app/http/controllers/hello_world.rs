use std::sync::Arc;

use axum::{extract::State, Json};
use serde_json::json;
use crate::app::http::core::{app_state::AppState, result::AppResult};

pub async fn get_hello_world(
    State(_state): State<Arc<AppState>>
) -> AppResult<Json<serde_json::Value>> {
    Ok(Json(json!({ "message": "Hello World, Rust" })))
}