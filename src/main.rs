mod app;

use axum::{
    routing::get,
    Router,
};
use app::http::core::result::AppResult;
use app::http::controllers::hello_world_ctl::get_hello_world;

#[tokio::main]
async fn main() -> AppResult<()> {
    let app = Router::new().route("/", get(get_hello_world));

    let ip = "127.0.0.1".to_string();
    let port = "3030".to_string();
    let running = format!("{}:{}", ip, port);
    println!("App runing on {}", running);

    let listener = tokio::net::TcpListener::bind(running).await?;
    axum::serve(listener, app).await?;

    Ok(())
}