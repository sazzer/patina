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
