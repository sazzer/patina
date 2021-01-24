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
        _provider: ProviderId,
        _nonce: String,
        _params: HashMap<String, String>,
    ) -> Result<UserResource, CompleteAuthenticationError> {
        Err(CompleteAuthenticationError::Unexpected)
    }
}
