use async_trait::async_trait;

use super::{AuthenticationId, AuthenticationService, UserID, UserResource};

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

    /// Get the user with the given Authentication details.
    ///
    /// # Parameters
    /// - `authentication_service` - The ID of the service the user is authenticated at.
    /// - `authentication_id` - The ID of the user at this service
    ///
    /// # Returns
    /// The user, or `None` if it doesn't exist.
    async fn get_user_by_authentication(
        &self,
        authentication_service: AuthenticationService,
        authentication_id: AuthenticationId,
    ) -> Option<UserResource>;
}
