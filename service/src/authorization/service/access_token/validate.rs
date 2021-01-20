use std::ops::Deref;

use biscuit::{jwa::SignatureAlgorithm, jws::Compact, ClaimsSet, Empty};

use super::AccessTokenService;
use crate::authorization::{
    AccessToken, Principal, SecurityContext, SecurityContextId, ValidateAccessTokenError,
    ValidateAccessTokenUseCase,
};

impl ValidateAccessTokenUseCase for AccessTokenService {
    fn validate_access_token(
        &self,
        access_token: AccessToken,
    ) -> Result<SecurityContext, ValidateAccessTokenError> {
        let claims: Compact<ClaimsSet<()>, Empty> = Compact::new_encoded(&access_token.0)
          .into_decoded(&self.signing_secret, SignatureAlgorithm::HS512)
          .map_err(|e| {
            tracing::warn!(e = ?e, access_token = ?access_token, "Failed to decode access token");

            ValidateAccessTokenError::Malformed
          })?;

        let payload = claims.payload().unwrap().clone();

        let jti = payload.registered.id.map(SecurityContextId);
        let nbf = payload.registered.not_before.map(|ts| *ts.deref());
        let exp = payload.registered.expiry.map(|ts| *ts.deref());
        let sub = payload.registered.subject.map(Principal::User);

        if let (Some(id), Some(issued), Some(expires), Some(principal)) = (jti, nbf, exp, sub) {
            let security_context = SecurityContext {
                id,
                issued,
                expires,
                principal,
            };

            tracing::debug!(security_context = ?security_context, access_token = ?access_token, "Verified security context");

            Ok(security_context)
        } else {
            tracing::warn!(access_token = ?access_token, "Decoded access token was missing required parts");

            Err(ValidateAccessTokenError::Malformed)
        }
    }
}

#[cfg(test)]
mod tests {
    use assert2::{check, let_assert};
    use chrono::{Timelike, Utc};
    use test_case::test_case;

    use super::*;
    use crate::authorization::GenerateAccessTokenUseCase;
    use crate::users::UserID;

    #[test_case("", &ValidateAccessTokenError::Malformed ; "Blank")]
    #[test_case("   ", &ValidateAccessTokenError::Malformed ; "Whitespace")]
    #[test_case("Invalid", &ValidateAccessTokenError::Malformed ; "Invalid String")]
    // The below are all generaetd using jwt.io
    #[test_case("eyJhbGciOiJIUzUxMiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.PivFwlv3V9e09vkfkShG99nBs9tBQBCBF417HO_LzqUZ-1cs-vymR2fi1njkaQiGncl7BJDPBvmg8_4Iu2iB5g", &ValidateAccessTokenError::Malformed ; "Invalid Signature")] // Signed with key: "different"
    #[test_case("eyJhbGciOiJIUzM4NCIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.hO2sthNQUSfvI9ylUdMKDxcrm8jB3KL6Rtkd3FOskL-jVqYh2CK1es8FKCQO8_tW", &ValidateAccessTokenError::Malformed ; "Invalid Algorithm")] // Signed with algorithm: HS384
    #[test_case("eyJhbGciOiJIUzUxMiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJ0YWc6cGF0aW5hLDIwMjEsYXV0aG9yaXphdGlvbiIsImF1ZCI6InRhZzpwYXRpbmEsMjAyMSxhdXRob3JpemF0aW9uIiwiZXhwIjoxNjExMTI5NjcxLCJuYmYiOjE2MTExMjk2NzEsImlhdCI6MTYxMTEyOTY3MSwianRpIjoiYmQ3MDE5M2YtNmZjMy00ZTJlLTg0YjctYjA1ZDAxYTk2MTViIn0.ShvodGUiY3wvLPTOKZ2ZbItmk3kKmpCJOV7HYZK3vYqxll5_YDv_6BGvVNnJhBTi2ZbaIX0_q_7aeKstNP0GaQ", &ValidateAccessTokenError::Malformed ; "Missing field: sub")]
    #[test_case("eyJhbGciOiJIUzUxMiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJ0YWc6cGF0aW5hLDIwMjEsYXV0aG9yaXphdGlvbiIsInN1YiI6ImE1NDk5ODQ1LWU3YmMtNGE2Ni1iMTc5LTkyOGMwZWVhNzRhMSIsImF1ZCI6InRhZzpwYXRpbmEsMjAyMSxhdXRob3JpemF0aW9uIiwiZXhwIjoxNjExMTI5NjcxLCJpYXQiOjE2MTExMjk2NzEsImp0aSI6ImJkNzAxOTNmLTZmYzMtNGUyZS04NGI3LWIwNWQwMWE5NjE1YiJ9.2Ta46WCYF4rRNDOSHhbHDXeEYddWEpUWalPuRIj_rcQPYt1hx-4eEwBCvyc_hxzD0fguL0t9w3ifcobaQsRamA", &ValidateAccessTokenError::Malformed ; "Missing field: nbf")]
    #[test_case("eyJhbGciOiJIUzUxMiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJ0YWc6cGF0aW5hLDIwMjEsYXV0aG9yaXphdGlvbiIsInN1YiI6ImE1NDk5ODQ1LWU3YmMtNGE2Ni1iMTc5LTkyOGMwZWVhNzRhMSIsImF1ZCI6InRhZzpwYXRpbmEsMjAyMSxhdXRob3JpemF0aW9uIiwibmJmIjoxNjExMTI5NjcxLCJpYXQiOjE2MTExMjk2NzEsImp0aSI6ImJkNzAxOTNmLTZmYzMtNGUyZS04NGI3LWIwNWQwMWE5NjE1YiJ9.IN7b20uNci1mgJEeeSkHH9X1-hJ1IYCKdd5ymhZs7ylbdhpSBjGP9UbDwMhDaviRXWh2RMWMiDxQfQPztNPULA", &ValidateAccessTokenError::Malformed ; "Missing field: exp")]
    #[test_case("eyJhbGciOiJIUzUxMiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJ0YWc6cGF0aW5hLDIwMjEsYXV0aG9yaXphdGlvbiIsInN1YiI6ImE1NDk5ODQ1LWU3YmMtNGE2Ni1iMTc5LTkyOGMwZWVhNzRhMSIsImF1ZCI6InRhZzpwYXRpbmEsMjAyMSxhdXRob3JpemF0aW9uIiwiZXhwIjoxNjExMTI5NjcxLCJuYmYiOjE2MTExMjk2NzEsImlhdCI6MTYxMTEyOTY3MX0.5b4eR48gcHi93Bz6VR-qkobLO8JwdifmxuqkauGIA38qzdpYy90tyGwYNF4XTpc0BvzMK2JycEWCKQPNyypLxQ", &ValidateAccessTokenError::Malformed ; "Missing field: jti")]
    fn validate_access_token_given_malformed_input_then_error(
        input: &str,
        err: &ValidateAccessTokenError,
    ) {
        let access_token = AccessToken(input.to_string());

        let sut = AccessTokenService::new("secret");

        let result = sut.validate_access_token(access_token);

        let_assert!(Err(e) = result);
        check!(&e == err);
    }

    #[test]
    fn validate_access_token_given_valid_input_then_success() {
        let sut = AccessTokenService::new("secret");

        let now = Utc::now().with_nanosecond(0).unwrap();
        let user_id: UserID = "a5499845-e7bc-4a66-b179-928c0eea74a1".parse().unwrap();

        let security_context = SecurityContext {
            issued:    now,
            expires:   now,
            principal: user_id.into(),
            id:        SecurityContextId("171f3599-4562-4768-bcc7-f96a80224430".to_string()),
        };

        let access_token = sut.generate_access_token(security_context);

        let result = sut.validate_access_token(access_token);

        let_assert!(Ok(sc) = result);
        check!(sc.issued == now);
        check!(sc.expires == now);
        check!(sc.id == SecurityContextId("171f3599-4562-4768-bcc7-f96a80224430".to_string()));

        let_assert!(Principal::User(user) = sc.principal);
        check!(user == "a5499845-e7bc-4a66-b179-928c0eea74a1");
    }
}
