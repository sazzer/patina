use uuid::Uuid;

use super::AuthenticationService;
use crate::authentication::{
    ProviderId, StartAuthentication, StartAuthenticationError, StartAuthenticationUseCase,
};

impl StartAuthenticationUseCase for AuthenticationService {
    #[tracing::instrument(skip(self))]
    fn start_authentication(
        &self,
        provider_id: &ProviderId,
    ) -> Result<StartAuthentication, StartAuthenticationError> {
        let provider = self
            .providers
            .get(provider_id)
            .ok_or(StartAuthenticationError::UnknownProvider)?;

        let nonce = Uuid::new_v4().to_string();

        let redirect_url = provider.start_authentication(&nonce);

        Ok(StartAuthentication {
            redirect_url,
            nonce,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, sync::Arc};

    use assert2::{check, let_assert};
    use async_trait::async_trait;

    use super::*;
    use crate::authentication::providers::{
        AuthenticatedUser, CompleteAuthenticationError, Provider,
    };

    struct MockProvider {}

    #[async_trait]
    impl Provider for MockProvider {
        fn start_authentication(&self, nonce: &str) -> String {
            format!("http://result.example.com/{}", nonce)
        }

        async fn complete_authentication(
            &self,
            _nonce: &str,
            _params: HashMap<String, String>,
        ) -> Result<AuthenticatedUser, CompleteAuthenticationError> {
            todo!()
        }
    }

    #[test]
    fn start_authentication_given_unknown_provider_is_an_error() {
        let providers = HashMap::new();
        let sut = AuthenticationService::new(providers);

        let provider_id = ProviderId::new("unknown");

        let result = sut.start_authentication(&provider_id);

        let_assert!(Err(err) = result);
        check!(err == StartAuthenticationError::UnknownProvider);
    }

    #[test]
    fn start_authentication_given_known_provider_is_success() {
        let mut providers: HashMap<ProviderId, Arc<dyn Provider>> = HashMap::new();
        providers.insert(ProviderId::new("known"), Arc::new(MockProvider {}));

        let sut = AuthenticationService::new(providers);

        let provider_id = ProviderId::new("known");

        let result = sut.start_authentication(&provider_id);

        let_assert!(Ok(res) = result);
        check!(format!("http://result.example.com/{}", res.nonce) == res.redirect_url);
    }
}
