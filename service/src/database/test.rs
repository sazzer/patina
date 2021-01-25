use std::sync::Arc;

use prometheus::Registry;

use super::Database;

/// Wrapper around a database to use for testing purposes.
pub struct TestDatabase {
    pub test_database: patina_testdatabase::TestDatabase,
    pub database:      Arc<Database>,
    pub prometheus:    Registry,
}

impl TestDatabase {
    /// Construct a new test database.
    pub async fn new() -> Self {
        let test_database = patina_testdatabase::TestDatabase::new();
        let prometheus = Registry::new();
        let settings = crate::database::config::Settings {
            url: test_database.url.clone(),
        };
        let database = crate::database::config::new(&settings, &prometheus).await;

        Self {
            test_database,
            database,
            prometheus,
        }
    }
}
