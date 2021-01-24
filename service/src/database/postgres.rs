use std::{
    ops::{Deref, DerefMut},
    str::FromStr,
};

use deadpool::managed::{Object, PoolError};
use deadpool_postgres::{ClientWrapper, Manager, ManagerConfig, Pool, RecyclingMethod};
use prometheus::{opts, IntGauge, Registry};

/// Database connection that works in terms of Postgres.
pub struct Database {
    pool:             Pool,
    checkout_counter: IntGauge,
}

impl Database {
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
        let checkout_counter = IntGauge::with_opts(counter_opts).unwrap();

        prometheus
            .register(Box::new(checkout_counter.clone()))
            .unwrap();

        Self {
            pool,
            checkout_counter,
        }
    }

    /// Check out a connection from the database pool in order to make queries
    ///
    /// # Returns
    /// The connection to use
    ///
    /// # Errors
    /// If the pool is unable to return a viable connection
    pub async fn try_checkout(&self) -> Result<Connection, PoolError<tokio_postgres::Error>> {
        self.checkout_counter.inc();

        self.pool
            .get()
            .await
            .map(|conn| Connection(conn, self.checkout_counter.clone()))
    }

    /// Check out a connection from the database pool in order to make queries
    ///
    /// # Returns
    /// The connection to use
    ///
    /// # Errors
    /// If the pool is unable to return a viable connection
    pub async fn checkout(&self) -> Connection {
        self.try_checkout()
            .await
            .expect("Failed to get database connection")
    }
}

/// Wrapper around a database connection.
pub struct Connection(Object<ClientWrapper, tokio_postgres::Error>, IntGauge);

impl Drop for Connection {
    fn drop(&mut self) {
        self.1.dec()
    }
}

impl Deref for Connection {
    type Target = Object<ClientWrapper, tokio_postgres::Error>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Connection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
