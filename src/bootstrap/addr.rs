use crate::app::http::core::result::AppResult;
use std::env;

pub fn addr() -> AppResult<String> {
    let host = env::var("HOST")?;
    let port = env::var("PORT")?;
    let addr = format!("{}:{}", host, port);
    Ok(addr)
}