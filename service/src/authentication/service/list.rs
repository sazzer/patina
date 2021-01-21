use super::AuthenticationService;
use crate::authentication::{ListProvidersUseCase, ProviderId};

impl ListProvidersUseCase for AuthenticationService {
    fn list_providers(&self) -> Vec<crate::authentication::ProviderId> {
        let mut providers = vec![ProviderId::new("twitter"), ProviderId::new("google")];
        providers.sort();

        providers
    }
}
