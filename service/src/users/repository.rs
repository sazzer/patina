mod get;
mod model;

use std::sync::Arc;

use crate::database::Database;

/// Repository for accessing the users data
pub struct Repository {
    /// The database connection.
    database: Arc<dyn Database>,
}

impl Repository {
    /// Create a new users repository.
    ///
    /// # Parameters
    /// - `database` - The database connection
    pub fn new(database: Arc<dyn Database>) -> Self {
        Self { database }
    }
}
