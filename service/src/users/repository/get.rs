use super::Repository;
use crate::users::{UserID, UserResource};

impl Repository {
    /// Get the single user with the given ID
    ///
    /// # Parameters
    /// - `id` - The ID of the user
    ///
    /// # Returns
    /// The user, or `None` if the user isn't found.
    pub async fn get_by_id(&self, id: UserID) -> Option<UserResource> {
        let conn = self.database.checkout().await;

        let row = conn
            .query_opt("SELECT * FROM users WHERE user_id = $1", &[&id])
            .await
            .expect("Failed to query database")?;

        Some(row.into())
    }
}