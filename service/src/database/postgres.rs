use std::str::FromStr;

use deadpool::managed::{Object, PoolError};
use deadpool_postgres::{ClientWrapper, Manager, ManagerConfig, Pool, RecyclingMethod};
use postgres_types::ToSql;
use prometheus::{opts, IntGauge, Registry};
use tokio_postgres::{Error, IsolationLevel, Row};

/// Database connection that works in terms of Postgres.
pub struct Database {
    pool:             Pool,
    checkout_counter: IntGauge,
}

impl Database {
    #[tracing::instrument(name = "Database::new", skip(prometheus))]
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
    #[tracing::instrument(name = "Database::try_checkout", skip(self))]
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
    #[tracing::instrument(name = "Connection::begin_transaction", skip(self))]
    pub async fn begin_transaction(&mut self) -> Transaction<'_> {
        Transaction(
            self.0
                .build_transaction()
                .isolation_level(IsolationLevel::Serializable)
                .read_only(false)
                .deferrable(false)
                .start()
                .await
                .expect("Failed to start transaction"),
        )
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

        let span = tracing::trace_span!(
            "Connection::query_opt",
            statement = statement.as_str(),
            found = tracing::field::Empty,
            error = tracing::field::Empty,
        );
        let _enter = span.enter();

        let result = self.0.query_opt(statement.as_str(), params).await;

        match &result {
            Ok(Some(_)) => span.record("found", &true),
            Ok(None) => span.record("found", &false),
            Err(e) => span.record("error", &e.to_string().as_str()),
        };

        result
    }
}

pub struct Transaction<'a>(deadpool_postgres::Transaction<'a>);

impl<'a> Transaction<'a> {
    #[tracing::instrument(name = "Transaction::commit", skip(self))]
    pub async fn commit(self) -> Result<(), tokio_postgres::Error> {
        self.0.commit().await
    }

    pub async fn execute<S>(
        &self,
        statement: S,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<u64, tokio_postgres::Error>
    where
        S: Into<String>,
    {
        let statement = statement.into();

        let span = tracing::trace_span!(
            "Transaction::execute",
            statement = statement.as_str(),
            rows = tracing::field::Empty,
            error = tracing::field::Empty
        );
        let _enter = span.enter();

        let result = self.0.execute(statement.as_str(), params).await;

        match &result {
            Ok(rows) => span.record("rows", &rows),
            Err(e) => span.record("error", &e.to_string().as_str()),
        };

        result
    }

    pub async fn batch_execute<S>(&self, statement: S) -> Result<(), tokio_postgres::Error>
    where
        S: Into<String>,
    {
        let statement = statement.into();

        let span = tracing::trace_span!(
            "Transaction::batch_execute",
            statement = statement.as_str(),
            error = tracing::field::Empty
        );
        let _enter = span.enter();

        let result = self.0.batch_execute(statement.as_str()).await;

        if let Err(e) = &result {
            span.record("error", &e.to_string().as_str());
        }

        result
    }

    /// Perform a query for a set of rows, returning all of the rows that matched.
    ///
    /// # Parameters
    /// - `statement` - The SQL statement to perform
    /// - `params` - The parameters to bind to the query
    ///
    /// # Returns
    /// The rows that are returned.
    pub async fn query<T>(
        &self,
        statement: T,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<Vec<Row>, Error>
    where
        T: Into<String>,
    {
        let statement = statement.into();

        let span = tracing::trace_span!(
            "Transaction::query",
            statement = statement.as_str(),
            rows = tracing::field::Empty,
            error = tracing::field::Empty
        );
        let _enter = span.enter();

        let result = self.0.query(statement.as_str(), params).await;

        match &result {
            Ok(rows) => span.record("rows", &rows.len()),
            Err(e) => span.record("error", &e.to_string().as_str()),
        };

        result
    }
}
