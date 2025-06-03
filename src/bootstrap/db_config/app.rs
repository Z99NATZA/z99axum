#![allow(dead_code)]

use std::sync::Arc;
use crate::app::http::core::{app_state::AppState, result::AppResult};
use dotenv::dotenv;
use crate::routes;
use super::app_serve;
use crate::bootstrap::addr;

pub async fn run() -> AppResult<()> {
    dotenv().ok();

    let state = set_state()?;
    let addr = addr::addr()?;
    let app = routes::api::api(state);
    app_serve::run(addr, app).await?;

    Ok(())
}
 
pub fn set_state() -> AppResult<Arc<AppState>> {
    let state = Arc::new(AppState { db: None });

    Ok(state)
}