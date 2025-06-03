mod app;
mod routes;
mod bootstrap;
mod config;

use app::http::core::result::AppResult;
use bootstrap::app as z99axum;

#[tokio::main]
async fn main() -> AppResult<()> {
    z99axum::run().await?;
    Ok(())
}
