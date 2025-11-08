use anyhow::Result;
use shared::config::DatabaseConfig;
use sqlx::{PgPool, postgres::PgConnectOptions};

pub mod manga;

#[derive(Clone)]
pub struct ConnectionPool(PgPool);

impl ConnectionPool {
    pub fn new(pool: PgPool) -> Self {
        Self(pool)
    }

    pub fn inner_ref(&self) -> &PgPool {
        &self.0
    }

    pub async fn begin(&self) -> Result<sqlx::Transaction<'_, sqlx::Postgres>> {
        self.0.begin().await.map_err(|e| e.into())
    }
}

pub fn connect_database_with(cfg: &DatabaseConfig) -> ConnectionPool {
    ConnectionPool(PgPool::connect_lazy_with(make_pg_connect_options(cfg)))
}

fn make_pg_connect_options(cfg: &DatabaseConfig) -> sqlx::postgres::PgConnectOptions {
    PgConnectOptions::new()
        .host(&cfg.host)
        .port(cfg.port)
        .username(&cfg.username)
        .password(&cfg.password)
        .database(&cfg.database)
}
