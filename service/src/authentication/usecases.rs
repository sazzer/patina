use super::ProviderId;

/// Use case to get the list of authentication providers that can be used.
pub trait ListProvidersUseCase {
    /// Get the list of authentication providers to use.
    fn list_providers(&self) -> Vec<ProviderId>;
}
