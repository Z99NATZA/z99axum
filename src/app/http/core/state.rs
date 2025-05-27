use std::collections::HashMap;
use sqlx::{MySqlPool, PgPool};

#[derive(Clone)]
pub struct _AppState {
    pub mysql_pools: HashMap<String, MySqlPool>,
    pub pgsql_pools: HashMap<String, PgPool>,
}
