use super::{Principal, SecurityContext};

/// Use case for generating a security context for a principal.
pub trait GenerateSecurityContextUseCase {
    /// Generate the security context for the principal
    ///
    /// # Parameters
    /// - `principal` - The principal to generate the security context for
    ///
    /// # Returns
    /// The security context.
    fn generate_use_case(&self, principal: Principal) -> SecurityContext;
}
