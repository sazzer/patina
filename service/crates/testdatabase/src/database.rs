use std::str::FromStr;

use crate::seed::SeedData;

use super::postgres::Postgres;
use deadpool_postgres::{Manager, ManagerConfig, Pool, RecyclingMethod};
use lazy_static::lazy_static;
use testcontainers::{clients::Cli, Container, Docker};

lazy_static! {
    static ref DOCKER: Cli = Cli::default();
}

/// Wrapper arond a Postgres database container.
pub struct TestDatabase {
    #[allow(dead_code)]
    node: Container<'static, Cli, Postgres>,
    pub host: String,
    pub port: u16,
    pub url: String,
}

impl Default for TestDatabase {
    /// Construct a new test database.
    fn default() -> Self {
        tracing::info!("Starting Postgres database");
        let node = DOCKER.run(Postgres::default());

        let host = "localhost".to_owned();
        let port = node.get_host_port(5432).unwrap();
        let url = format!("postgres://postgres@{}:{}", host, port);
        tracing::info!(url = ?url, "Running postgres");

        Self {
            node,
            host,
            port,
            url,
        }
    }
}

impl TestDatabase {
    /// Seed some data into the database
    ///
    /// # Parameters
    /// - `data` - The data to seed
    pub async fn seed(&self, data: &dyn SeedData) -> &Self {
        tracing::debug!(data = ?data, "Seeding data");

        let pg_config = tokio_postgres::Config::from_str(&self.url).unwrap();

        let mgr_config = ManagerConfig {
            recycling_method: RecyclingMethod::Fast,
        };
        let mgr = Manager::from_config(pg_config, tokio_postgres::NoTls, mgr_config);
        let pool = Pool::new(mgr, 16);

        let conn = pool.get().await.unwrap();

        conn.execute(data.sql(), &data.binds()[..]).await.unwrap();

        self
    }
}
