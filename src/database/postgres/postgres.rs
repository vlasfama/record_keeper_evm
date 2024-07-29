use crate::database::postgres::config::Config;
use chrono::{NaiveDateTime, Utc};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;

// embed_migrations!("./src/database/postgres/migrations/");

pub struct PostgresDB {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl PostgresDB {
    pub fn new(database_url: &str, pool_size: u32) -> anyhow::Result<Self> {
        let manager = ConnectionManager::new(database_url);
        let pool = Pool::builder().max_size(pool_size).build(manager)?;

        // embedded_migrations::run_with_output(&pool.get()?, &mut std::io::stdout())?;

        Ok(PostgresDB { pool })
    }
    pub fn from_config(cfg: &Config) -> anyhow::Result<Self> {
        PostgresDB::new(&cfg.db_url, cfg.pool_size)
    }
}
