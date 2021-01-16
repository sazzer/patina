use std::str::FromStr;

use async_trait::async_trait;
use deadpool::managed::{Object, PoolError};
use deadpool_postgres::{ClientWrapper, Manager, ManagerConfig, Pool, RecyclingMethod};

use super::Database;
use crate::health::HealthCheckable;

/// Database connection that works in terms of Postgres.
pub struct Postgres {
    pool: Pool,
}

impl Postgres {
    pub async fn new(url: &str) -> Self {
        let pg_config = tokio_postgres::Config::from_str(url).expect("Invalid database URL");

        let mgr_config = ManagerConfig {
            recycling_method: RecyclingMethod::Fast,
        };
        let mgr = Manager::from_config(pg_config, tokio_postgres::NoTls, mgr_config);
        let pool = Pool::new(mgr, 16);

        pool.get().await.expect("Unable to open database connection");

        Self { pool }
    }
}

#[async_trait]
impl Database for Postgres {
    async fn checkout(&self) -> Result<Object<ClientWrapper, tokio_postgres::Error>, PoolError<tokio_postgres::Error>> {
        self.pool.get().await
    }
}

#[async_trait]
impl HealthCheckable for Postgres {
    async fn check_health(&self) -> Result<(), String> {
        let conn = self.checkout().await.map_err(|e| e.to_string())?;

        conn.simple_query("SELECT 1").await.map_err(|e| e.to_string())?;

        Ok(())
    }
}
