use crate::app::http::core::result::AppResult;
use dotenv::dotenv;
use crate::routes;
use super::axum_serve;

pub async fn run() -> AppResult<()> {
    dotenv().ok();

    let addr = super::addr::addr()?;
    let router = routes::api::api();
    axum_serve::run(addr, router).await?;

    Ok(())
}
