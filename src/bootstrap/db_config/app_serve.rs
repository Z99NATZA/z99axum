use axum::Router;
use crate::app::http::core::result::AppResult;

pub async fn run(addr: String, router: Router) -> AppResult<()> {
    println!("App runing on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, router).await?;

    Ok(())
}