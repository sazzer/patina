use super::ProviderId;

/// Use case to get the list of authentication providers that can be used.
pub trait ListProvidersUseCase {
    /// Get the list of authentication providers to use.
    fn list_providers(&self) -> Vec<&ProviderId>;
}

/// Details needed to start authentication.
#[derive(Debug)]
pub struct StartAuthentication {
    /// The URL to redirect the client to.
    pub redirect_url: String,
    /// The unique nonce that represents this authentication request.
    pub nonce:        String,
}

/// Errors that can happen when starting authentication.
#[derive(Debug, PartialEq, thiserror::Error)]
pub enum StartAuthenticationError {
    /// The requested provider is unknown.
    #[error("The requested provider was unknown")]
    UnknownProvider,
}

/// Use Case to start authentication.
pub trait StartAuthenticationUseCase {
    /// Start authentication with the requested provider.
    ///
    /// # Parameters
    /// - `provider_id` - The authentication provider to use
    ///
    /// # Returns
    /// The details needed to start authentication.
    fn start_authentication(
        &self,
        provider_id: &ProviderId,
    ) -> Result<StartAuthentication, StartAuthenticationError>;
}
