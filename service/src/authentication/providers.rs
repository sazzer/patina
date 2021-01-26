pub mod google;

use std::collections::HashMap;

use async_trait::async_trait;

use crate::users::{AuthenticationId, Email};

/// Trait representing an authentication provider.
#[async_trait]
pub trait Provider: Sync + Send {
    /// Start authentication with the provider.
    ///
    /// # Parameters
    /// - `nonce` - A unique string to represent this exact attempt to start authentication. If
    ///   possible, this should be returned into the provider when completing authentication.
    ///
    /// # Returns
    /// The URL to redirect the user to in order to start authentication.
    fn start_authentication(&self, nonce: &str) -> String;

    /// Complete authentication with the provider, returning some details of the user that
    /// authenticated. Note that these details are from the provider and not from the local
    /// Users component.
    ///
    /// # Parameters
    /// - `nonce` - The unique nonce for this request
    /// - `params` - The parameters provided to the callback by the provider
    ///
    /// # Returns
    /// The details of the user from the provider
    async fn complete_authentication(
        &self,
        nonce: &str,
        params: HashMap<String, String>,
    ) -> Result<AuthenticatedUser, CompleteAuthenticationError>;
}

/// Details of an authenticated user from the provider
#[derive(Debug)]
pub struct AuthenticatedUser {
    /// The ID of the user at the provider
    authentication_id: AuthenticationId,
    /// The display name of the authentication details
    authentication_display_name: String,
    /// The email address of the user
    email: Email,
    /// The display name of the user
    display_name: String,
}

/// Errors that can occur when attempting to complete authentication
#[derive(Debug, PartialEq, thiserror::Error)]
pub enum CompleteAuthenticationError {
    #[error("An unexpected error occurred")]
    Unexpected,
}
