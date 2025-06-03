#![allow(dead_code)]

use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

pub async fn create_pool(db_url: &str) -> PgPool {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(db_url)
        .await
        .expect("Failed to create pool")
}