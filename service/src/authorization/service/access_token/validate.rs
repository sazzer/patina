use super::AccessTokenService;
use crate::authorization::{
    AccessToken, SecurityContext, ValidateAccessTokenError, ValidateAccessTokenUseCase,
};

impl ValidateAccessTokenUseCase for AccessTokenService {
    fn validate_access_token(
        &self,
        access_token: AccessToken,
    ) -> Result<SecurityContext, ValidateAccessTokenError> {
        Err(ValidateAccessTokenError::UnexpectedError)
    }
}
