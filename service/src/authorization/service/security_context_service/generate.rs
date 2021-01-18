use super::SecurityContextService;
use crate::authorization::{GenerateSecurityContextUseCase, Principal, SecurityContext};

impl GenerateSecurityContextUseCase for SecurityContextService {
    fn generate_use_case(&self, _principal: Principal) -> SecurityContext {
        todo!()
    }
}
