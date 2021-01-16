use async_trait::async_trait;

use super::{UserID, UserResource};

/// Use case for getting a user by ID.
#[async_trait]
pub trait GetUserUseCase {
    /// Get the user with the given ID.
    ///
    /// # Parameters
    /// - `user_id` - The ID of the user to get.
    ///
    /// # Returns
    /// The user, or `None` if it doesn't exist.
    async fn get_user_by_id(&self, user_id: UserID) -> Option<UserResource>;
}
