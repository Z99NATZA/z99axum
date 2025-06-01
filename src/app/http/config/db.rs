use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum DbType {
    MySql,
    Postgres,
}

#[derive(Clone)]
pub enum DbPool {
    MySql(sqlx::MySqlPool),
    Postgres(sqlx::PgPool),
}

#[derive(Debug, Deserialize)]
pub struct DbConfig {
    pub db_type: DbType,
    pub host: String,
    pub port: u16,
    pub database: String,
    pub username: String,
    pub password: String,
}

