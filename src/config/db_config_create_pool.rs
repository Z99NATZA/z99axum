use sqlx::{
    mysql::{MySqlConnectOptions, MySqlPoolOptions},
    postgres::{PgConnectOptions, PgPoolOptions},
};
use crate::app::http::core::result::AppResult;

use super::db_config::{DbConfig, DbPool, DbType};

pub async fn create_pool(config: DbConfig) -> AppResult<DbPool> {

    match config.db_type {
        DbType::MySql => {
            let options = MySqlConnectOptions::new()
                .host(&config.host)
                .port(config.port)
                .username(&config.username)
                .password(&config.password)
                .database(&config.database);

            let pool = MySqlPoolOptions::new()
                .connect_with(options)
                .await?;

            Ok(DbPool::MySql(pool))
        }

        DbType::Postgres => {
            let options = PgConnectOptions::new()
                .host(&config.host)
                .port(config.port)
                .username(&config.username)
                .password(&config.password)
                .database(&config.database);

            let pool = PgPoolOptions::new()
                .connect_with(options)
                .await?;

            Ok(DbPool::Postgres(pool))
        }
    }
}
