use std::sync::Arc;

use prometheus::Registry;

use super::{migrate::migrate, postgres::Database};

/// Settings needed for the Database component.
#[derive(Debug)]
pub struct Settings {
    /// The connection URL
    pub url: String,
}

/// Create a new instance of the Database component.
pub async fn new(settings: &Settings, prometheus: &Registry) -> Arc<Database> {
    tracing::debug!(settings = ?settings, "Creating database connection");

    let database = Database::new(&settings.url, prometheus).await;
    migrate(&database).await;

    Arc::new(database)
}
