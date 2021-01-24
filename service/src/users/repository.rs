mod get;
mod model;

use std::sync::Arc;

use crate::database::Database;

/// Repository for accessing the users data
pub struct Repository {
    /// The database connection.
    database: Arc<Database>,
}

impl Repository {
    /// Create a new users repository.
    ///
    /// # Parameters
    /// - `database` - The database connection
    pub fn new(database: Arc<Database>) -> Self {
        Self { database }
    }
}
