use axum::Json;
use serde_json::json;
use crate::app::http::core::result::AppResult;

pub async fn get_hello_world() -> AppResult<Json<serde_json::Value>> {
    Ok(Json(json!({ "message": "Hello World" })))
}