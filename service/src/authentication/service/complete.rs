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
        provider_id: ProviderId,
        nonce: &str,
        params: HashMap<String, String>,
    ) -> Result<UserResource, CompleteAuthenticationError> {
        let provider = self
            .providers
            .get(&provider_id)
            .ok_or(CompleteAuthenticationError::UnknownProvider)?;

        let authenticated_user = provider
            .complete_authentication(nonce, params)
            .await
            .map_err(|e| {
                tracing::warn!(err = ?e, "Failed to authenticate with provider");
                CompleteAuthenticationError::AuthentictionFailed
            })?;

        let authentication_service = provider_id.into();

        let user = self
            .get_user_user_case
            .get_user_by_authentication(
                authentication_service,
                authenticated_user.authentication_id,
            )
            .await;

        #[allow(clippy::option_if_let_else)] // TODO: For now
        if let Some(user) = user {
            Ok(user)
        } else {
            Err(CompleteAuthenticationError::Unexpected)
        }
    }
}
