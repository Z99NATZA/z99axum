mod app;
mod routes;
//
use app::http::core::result::AppResult;
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() -> AppResult<()> {
    dotenv().ok();

    let app = routes::api::api();

    let host = env::var("HOST")?;
    let port = env::var("PORT")?;
    let running = format!("{}:{}", host, port);
    
    println!("App runing on {}", running);

    let listener = tokio::net::TcpListener::bind(running).await?;
    axum::serve(listener, app).await?;

    Ok(())
}