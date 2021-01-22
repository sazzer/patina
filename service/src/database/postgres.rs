use std::str::FromStr;

use async_trait::async_trait;
use deadpool::managed::Object;
use deadpool_postgres::{ClientWrapper, Manager, ManagerConfig, Pool, RecyclingMethod};
use prometheus::{opts, IntCounter, Registry};

use super::Database;
use crate::health::HealthCheckable;

/// Database connection that works in terms of Postgres.
pub struct Postgres {
    pool:             Pool,
    checkout_counter: IntCounter,
}

impl Postgres {
    pub async fn new(url: &str, prometheus: &Registry) -> Self {
        let pg_config = tokio_postgres::Config::from_str(url).expect("Invalid database URL");

        let mgr_config = ManagerConfig {
            recycling_method: RecyclingMethod::Fast,
        };
        let mgr = Manager::from_config(pg_config, tokio_postgres::NoTls, mgr_config);
        let pool = Pool::new(mgr, 16);

        pool.get()
            .await
            .expect("Unable to open database connection");

        let counter_opts =
            opts!("checkout", "Number of connections checked out").namespace("postgres");
        let checkout_counter = IntCounter::with_opts(counter_opts).unwrap();

        prometheus
            .register(Box::new(checkout_counter.clone()))
            .unwrap();

        Self {
            pool,
            checkout_counter,
        }
    }
}

#[async_trait]
impl Database for Postgres {
    async fn checkout(&self) -> Object<ClientWrapper, tokio_postgres::Error> {
        self.checkout_counter.inc();

        self.pool
            .get()
            .await
            .expect("Failed to get database connection")
    }
}

#[async_trait]
impl HealthCheckable for Postgres {
    async fn check_health(&self) -> Result<(), String> {
        self.checkout_counter.inc();

        let conn = self.pool.get().await.map_err(|e| e.to_string())?;

        conn.simple_query("SELECT 1")
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}
