use std::collections::HashMap;

use async_trait::async_trait;

use super::AuthenticationService;
use crate::{
    authentication::{CompleteAuthenticationError, CompleteAuthenticationUseCase, ProviderId},
    users::UserResource,
};

#[async_trait]
impl CompleteAuthenticationUseCase for AuthenticationService {
    async fn complete_authentication(
        &self,
        provider_id: &ProviderId,
        _nonce: &str,
        _params: HashMap<String, String>,
    ) -> Result<UserResource, CompleteAuthenticationError> {
        let _provider = self
            .providers
            .get(provider_id)
            .ok_or(CompleteAuthenticationError::UnknownProvider)?;

        Err(CompleteAuthenticationError::Unexpected)
    }
}
