use std::collections::HashMap;

use async_trait::async_trait;

use super::AuthenticationService;
use crate::{
    authentication::{CompleteAuthenticationError, CompleteAuthenticationUseCase, ProviderId},
    users::UserResource,
};

#[async_trait]
impl CompleteAuthenticationUseCase for AuthenticationService {
    #[tracing::instrument(skip(self))]
    async fn complete_authentication(
        &self,
        provider_id: &ProviderId,
        nonce: &str,
        params: HashMap<String, String>,
    ) -> Result<UserResource, CompleteAuthenticationError> {
        let provider = self
            .providers
            .get(provider_id)
            .ok_or(CompleteAuthenticationError::UnknownProvider)?;

        let _authenticated_user = provider.complete_authentication(nonce, params).await;

        Err(CompleteAuthenticationError::Unexpected)
    }
}
