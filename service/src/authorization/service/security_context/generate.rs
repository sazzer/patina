use chrono::Utc;

use super::SecurityContextService;
use crate::authorization::{
    GenerateSecurityContextUseCase, Principal, SecurityContext, SecurityContextId,
};

impl GenerateSecurityContextUseCase for SecurityContextService {
    #[tracing::instrument(skip(self))]
    fn generate_security_context(&self, principal: Principal) -> SecurityContext {
        let issued = Utc::now();
        let expires = issued + self.duration;
        let id = SecurityContextId::default();

        SecurityContext {
            id,
            issued,
            expires,
            principal,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use assert2::{check, let_assert};
    use chrono::Duration;

    use super::*;
    use crate::users::UserID;

    #[test]
    fn generate_is_successful() {
        let sut = SecurityContextService::new(Duration::days(5));
        let principal: Principal = UserID::from_str("c2a85f7d-5a78-44d0-90fc-31ed132845da")
            .unwrap()
            .into();

        let result = sut.generate_security_context(principal);

        check!(result.issued + Duration::days(5) == result.expires);

        let_assert!(Principal::User(id) = result.principal);
        check!(id == "c2a85f7d-5a78-44d0-90fc-31ed132845da");
    }
}
