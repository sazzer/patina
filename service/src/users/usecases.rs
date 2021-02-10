use async_trait::async_trait;
#[cfg(test)]
use mockall::predicate::*;
#[cfg(test)]
use mockall::*;

use super::{AuthenticationId, AuthenticationService, UserData, UserID, UserResource};

/// Use case for getting a user by ID.
#[cfg_attr(test, automock)]
#[async_trait]
pub trait GetUserUseCase: Send + Sync {
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

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum CreateUserError {
    #[error("An unexpected error occurred")]
    Unexpected,
}

/// Use case for creating a new user.
#[cfg_attr(test, automock)]
#[async_trait]
pub trait CreateUserUseCase: Send + Sync {
    /// Create a new user with the provided user data.
    ///
    /// # Parameters
    /// - `user` - The data of the user to create
    ///
    /// # Returns
    /// The newly created user, or else a reason why it couldn't be created.
    async fn create_user(&self, user: UserData) -> Result<UserResource, CreateUserError>;
}
