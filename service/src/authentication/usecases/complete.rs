use std::collections::HashMap;

use async_trait::async_trait;

use crate::{authentication::ProviderId, users::UserResource};

/// Use case for completing authentication of a user.
#[async_trait]
pub trait CompleteAuthenticationUseCase {
    /// Attempt to complete authentication of the user.
    ///
    /// # Parameters
    /// - `provider` - The authentication provider
    /// - `nonce` - The nonce that was set when starting authentication
    /// - `params` - The parameters received from the provider
    ///
    /// # Returns
    /// The details of the user that has just authenticated.
    async fn complete_authentication(
        &self,
        provider: ProviderId,
        nonce: &str,
        params: HashMap<String, String>,
    ) -> Result<UserResource, CompleteAuthenticationError>;
}
/// Errors that can occur when completing authentication.
#[derive(Debug, PartialEq, thiserror::Error)]
pub enum CompleteAuthenticationError {
    /// The requested provider is unknown.
    #[error("The requested provider was unknown")]
    UnknownProvider,

    #[error("Authentication with the provider failed")]
    AuthentictionFailed,

    #[error("An unexpected error occurred")]
    Unexpected,
}
