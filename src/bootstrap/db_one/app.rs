use crate::app::{http::core::result::AppResult};
use dotenv::dotenv;
use sqlx::{Pool, Postgres};
use crate::routes;
use super::app_serve;
use crate::bootstrap::addr;
use std::sync::Arc;
use crate::app::http::core::app_state::AppState;
use std::env;
use crate::config::db_one_create_pool::create_pool;

pub async fn run() -> AppResult<()> {
    dotenv().ok();
    let state = set_state().await?;
    let addr = addr::addr()?;
    let app = routes::api::api(state);
    app_serve::run(addr, app).await?;

    Ok(())
}
 
pub async fn set_state() -> AppResult<Arc<AppState>> {
    let pool = set_pool().await?;

    let state = Arc::new(AppState {
        db: Some(Arc::new(pool)),
    });

    Ok(state)
}

pub async fn set_pool() -> AppResult<Pool<Postgres>> {
    let db_url = env::var("DATABASE_URL")?;
    let pool = create_pool(&db_url).await;

    Ok(pool)
}