use std::str::FromStr;

use deadpool::managed::{Object, PoolError};
use deadpool_postgres::{
    ClientWrapper, Manager, ManagerConfig, Pool, RecyclingMethod, Transaction,
};
use postgres_types::ToSql;
use prometheus::{opts, IntGauge, Registry};
use tokio_postgres::{Error, IsolationLevel, Row};

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

impl Connection {
    /// Begin a new database transaction.
    ///
    /// # Returns
    /// The transaction in which to perform requests.
    pub async fn begin_transaction(&mut self) -> Transaction<'_> {
        self.0
            .build_transaction()
            .isolation_level(IsolationLevel::Serializable)
            .read_only(false)
            .deferrable(false)
            .start()
            .await
            .expect("Failed to start transaction")
    }

    /// Perform a query for a single row, returning an Option for the row or `None` if no row was
    /// found.
    ///
    /// # Parameters
    /// - `statement` - The SQL statement to perform
    /// - `params` - The parameters to bind to the query
    ///
    /// # Returns
    /// The row that is returned, or `None` if no row was returned.
    pub async fn query_opt<T>(
        &self,
        statement: T,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<Option<Row>, Error>
    where
        T: Into<String>,
    {
        let statement = statement.into();

        let span = tracing::trace_span!("query", statement = statement.as_str());
        let _enter = span.enter();

        self.0.query_opt(statement.as_str(), params).await
    }
}
