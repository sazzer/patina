pub mod google;

use async_trait::async_trait;

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
}
