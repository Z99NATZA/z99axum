use std::sync::Arc;

use axum::extract::Path;
use axum::extract::State;
use axum::Json;
use axum_macros::debug_handler;
use serde::Deserialize;
use serde::Serialize;
use sqlx::prelude::FromRow;
use crate::app::http::core::app_state::AppState;
use crate::app::http::core::error::AppError;
use crate::app::http::core::result::AppResult;
use crate::config::db_config;
use crate::config::db_config_create_pool;

#[derive(Debug, Deserialize)]
pub struct UserGetRequest {
    pub id: i32,
    pub db_config: db_config::DbConfig,
}

#[derive(Debug, FromRow, Serialize)]
pub struct UserGetResponse {
    pub id: i32,
    pub name: String,
    pub lname: String,
}

#[debug_handler]
pub async fn user_get(
    State(_state): State<Arc<AppState>>,
    Json(payload): Json<UserGetRequest>
) -> AppResult<Json<UserGetResponse>> {
    let id = payload.id;
    let db_config = payload.db_config;

    let pool = db_config_create_pool::create_pool(db_config).await?;

    let query = match &pool {
        db_config::DbPool::Postgres(pg_pool) => {
            sqlx::query_as::<_, UserGetResponse>(
                r#"SELECT id, name, lname FROM users WHERE id = $1"#
            )
            .bind(id)
            .fetch_optional(pg_pool)
            .await?
        }

        db_config::DbPool::MySql(mysql_pool) => {
            sqlx::query_as::<_, UserGetResponse>(
                r#"SELECT id, name, lname FROM users WHERE id = ?"#
            )
            .bind(id)
            .fetch_optional(mysql_pool)
            .await?
        }
    };

    let user = query.ok_or_else(|| AppError::NotFound(format!("User id {} not found", id)))?;

    Ok(Json(user))
}


#[debug_handler]
pub async fn user_get2(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> AppResult<Json<UserGetResponse>> {
    let db = state.db.as_ref().ok_or_else(|| {
        AppError::InternalError("Database not available".into())
    })?;

    let query = sqlx::query_as!(
        UserGetResponse,
        r#"SELECT id, name, lname FROM users WHERE id = $1"#,
        id
    )
    .fetch_optional(db.as_ref())
    .await?;

    let user = query.ok_or_else(|| AppError::NotFound(format!("User id {} not found", id)))?;
    Ok(Json(user))
}
