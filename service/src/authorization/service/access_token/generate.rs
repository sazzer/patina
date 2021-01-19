use biscuit::{
    jwa::SignatureAlgorithm, jws::RegisteredHeader, ClaimsSet, RegisteredClaims, SingleOrMultiple,
    JWT,
};

use super::AccessTokenService;
use crate::authorization::{AccessToken, GenerateAccessTokenUseCase, Principal, SecurityContext};

const ISSUER: &str = "tag:patina,2021,authorization";

impl GenerateAccessTokenUseCase for AccessTokenService {
    fn generate_access_token(&self, security_context: SecurityContext) -> AccessToken {
        let claims = ClaimsSet::<()> {
            registered: RegisteredClaims {
                issuer:     Some(ISSUER.to_string()),
                audience:   Some(SingleOrMultiple::Single(ISSUER.to_string())),
                issued_at:  Some(security_context.issued.into()),
                not_before: Some(security_context.issued.into()),
                expiry:     Some(security_context.expires.into()),
                id:         Some(security_context.id.0),
                subject:    match security_context.principal {
                    Principal::User(user_id) => Some(user_id),
                },
            },
            private:    (),
        };

        let header = RegisteredHeader {
            algorithm: SignatureAlgorithm::HS512,
            ..RegisteredHeader::default()
        };

        let decoded = JWT::new_decoded(header.into(), claims);
        let encoded = decoded
            .into_encoded(&self.signing_secret)
            .expect("Failed to encode JWT") // This can't happen
            .encoded()
            .expect("Failed to unwrap encoded JWT")
            .to_string();

        AccessToken(encoded)
    }
}

#[cfg(test)]
mod tests {
    use assert2::{check, let_assert};
    use chrono::Utc;

    use super::*;
    use crate::{authorization::SecurityContextId, users::UserID};

    #[test]
    fn generate_access_token_given_security_context_is_valid() {
        let now = Utc::now();
        let user_id: UserID = "a5499845-e7bc-4a66-b179-928c0eea74a1".parse().unwrap();

        let security_context = SecurityContext {
            issued:    now,
            expires:   now,
            principal: user_id.into(),
            id:        SecurityContextId::default(),
        };

        let sut = AccessTokenService::new("secret");

        let access_token = sut.generate_access_token(security_context);
        let_assert!(AccessToken(jwt) = access_token);

        check!(jwt != "");
    }
}
