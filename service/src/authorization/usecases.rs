use super::{AccessToken, Principal, SecurityContext};

/// Use case for generating a security context for a principal.
pub trait GenerateSecurityContextUseCase {
    /// Generate the security context for the principal
    ///
    /// # Parameters
    /// - `principal` - The principal to generate the security context for
    ///
    /// # Returns
    /// The security context.
    fn generate_security_context(&self, principal: Principal) -> SecurityContext;
}

/// Use Case for generating an access token for a security context.
pub trait GenerateAccessTokenUseCase {
    /// Generate an access token representing the provided Security Context.
    ///
    /// # Parameters
    /// - `security_context` - The security context
    ///
    /// # Returns
    /// The access token
    fn generate_access_token(&self, security_context: SecurityContext) -> AccessToken;
}

/// Use Case for validating an access token and returning the Security Context that generated it.
pub trait ValidateAccessTokenUseCase {
    /// Test if the provided access token is valid, and if so then rebuild the security context from
    /// it.
    ///
    /// # Parameters
    /// - `access_token` - The access token to validate
    ///
    /// # Returns
    /// The security context that the access token represents, or else an error indicating why it
    /// was invalid.
    fn validate_access_token(
        &self,
        access_token: AccessToken,
    ) -> Result<SecurityContext, ValidateAccessTokenError>;
}

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum ValidateAccessTokenError {
    #[error("The access token was malformed")]
    Malformed,
}
