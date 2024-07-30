use crate::database::postgres::config::Config;
use crate::database::provider::DatabaseReader;
use crate::database::provider::DatabaseWriter;
use crate::model::User;
use anyhow::Result;
use chrono::{NaiveDateTime, Utc};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./src/database/postgres/migrations/");

pub struct PostgresDB {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl PostgresDB {
    pub fn new(database_url: &str, pool_size: u32) -> anyhow::Result<Self> {
        let manager = ConnectionManager::new(database_url);
        let pool = Pool::builder().max_size(pool_size).build(manager)?;
        let connection = pool.get()?;
        Ok(PostgresDB { pool })
    }

    pub fn from_config(cfg: &Config) -> anyhow::Result<Self> {
        PostgresDB::new(&cfg.db_url, cfg.pool_size)
    }
}
impl DatabaseWriter for PostgresDB {
    fn create_user(&self, user: User) -> Result<()> {
        Ok(())
    }

    fn update_user(&self, user: User) -> Result<()> {
        Ok(())
    }
}

impl DatabaseReader for PostgresDB {
    fn get_info(&self, user: &User) -> Result<()> {
        Ok(())
    }
}
