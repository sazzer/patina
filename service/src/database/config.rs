use std::sync::Arc;

use prometheus::Registry;

use super::{migrate::migrate, postgres::Postgres};

/// Settings needed for the Database component.
#[derive(Debug)]
pub struct Settings {
    /// The connection URL
    pub url: String,
}

/// Create a new instance of the Database component.
pub async fn new(settings: &Settings, prometheus: &Registry) -> Arc<Postgres> {
    tracing::debug!(settings = ?settings, "Creating database connection");

    let database = Postgres::new(&settings.url, prometheus).await;
    migrate(&database).await;

    Arc::new(database)
}
