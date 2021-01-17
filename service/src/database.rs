pub mod config;
mod migrate;
mod postgres;

use async_trait::async_trait;
use deadpool::managed::Object;
use deadpool_postgres::ClientWrapper;

/// Wrapper around the database connection.
#[async_trait]
pub trait Database: Send + Sync {
    /// Check out a connection from the database pool in order to make queries
    ///
    /// # Returns
    /// The connection to use
    ///
    /// # Errors
    /// If the pool is unable to return a viable connection
    async fn checkout(&self) -> Object<ClientWrapper, tokio_postgres::Error>;
}
