use std::sync::Arc;

use super::postgres::Postgres;

/// Settings needed for the Database component.
#[derive(Debug)]
pub struct Settings {
    /// The connection URL
    pub url: String,
}

/// Create a new instance of the Database component.
pub async fn new(settings: &Settings) -> Arc<Postgres> {
    tracing::debug!(settings = ?settings, "Creating database connection");

    let database = Postgres::new(&settings.url).await;

    Arc::new(database)
}
