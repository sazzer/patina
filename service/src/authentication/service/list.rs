use super::AuthenticationService;
use crate::authentication::{ListProvidersUseCase, ProviderId};

impl ListProvidersUseCase for AuthenticationService {
    fn list_providers(&self) -> Vec<&ProviderId> {
        let mut providers: Vec<&ProviderId> = self.providers.keys().into_iter().collect();
        providers.sort();

        providers
    }
}
