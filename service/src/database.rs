pub mod config;
mod postgres;

use async_trait::async_trait;

/// Wrapper around the database connection.
#[async_trait]
pub trait Database {}
