use std::str::FromStr;

use async_trait::async_trait;
use deadpool_postgres::{Manager, ManagerConfig, Pool, RecyclingMethod};

use super::Database;

/// Database connection that works in terms of Postgres.
pub struct Postgres {
    #[allow(dead_code)] // TODO: FIX
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
impl Database for Postgres {}
