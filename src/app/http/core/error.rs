#![allow(dead_code)]

use std::fmt::Display;
use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;
use std::env::VarError;

#[derive(Debug)]
pub enum AppError {
    IoError(String),
    InternalError(String),
    EnvVarError(VarError),
    NotFound(String),
}

impl From<std::io::Error> for AppError {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value.to_string())
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Self::IoError(e) => e,
            Self::InternalError(e) => e,
            Self::EnvVarError(e) => &e.to_string(),
            Self::NotFound(e) => e,
        };
        write!(f, "Error: {}", message)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            AppError::InternalError(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.clone()),
            AppError::IoError(e) => (StatusCode::BAD_GATEWAY, e.clone()),
            AppError::EnvVarError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                self.to_string(),
            ),
            AppError::NotFound(e) => (StatusCode::NOT_FOUND, e.clone()),
        };

        let body = Json(json!({ "message": message }));
        (status, body).into_response()
    }
}

impl From<VarError> for AppError {
    fn from(value: VarError) -> Self {
        AppError::EnvVarError(value)
    }
}

impl From<sqlx::Error> for AppError {
    fn from(value: sqlx::Error) -> Self {
        AppError::InternalError(value.to_string())
    }
}

