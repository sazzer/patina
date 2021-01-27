use std::convert::TryFrom;

use biscuit::{jws::Compact, ClaimsSet, Empty};
use serde::{Deserialize, Serialize};

use crate::{
    authentication::providers::{AuthenticatedUser, CompleteAuthenticationError},
    users::AuthenticationId,
};

/// Representation of the Access Token details received from Google when authenticating a user
#[derive(Debug, Deserialize)]
pub struct GoogleToken {
    /// The OpenID Connect ID Token that contains some user details
    id_token: String,
}

/// Private claims that are required from the Google ID Token
#[derive(Debug, Deserialize, Serialize)]
pub struct GoogleClaims {
    /// The email address of the user
    email: String,
    /// The display name of the user
    name:  String,
}

impl TryFrom<GoogleToken> for AuthenticatedUser {
    type Error = CompleteAuthenticationError;

    fn try_from(value: GoogleToken) -> Result<Self, Self::Error> {
        let claims: Compact<ClaimsSet<GoogleClaims>, Empty> = Compact::new_encoded(&value.id_token);
        let payload = claims.unverified_payload().map_err(|e| {
            tracing::warn!(err = ?e, "Failed to parse ID Token");
            CompleteAuthenticationError::AuthenticationFailed(
                "Failed to parse ID Token".to_string(),
            )
        })?;

        let authentication_id = payload.registered.subject.map(AuthenticationId::new);
        let email = payload.private.email;
        let display_name = payload.private.name;

        Ok(Self {
            authentication_id: authentication_id.ok_or_else(|| {
                tracing::warn!("No subject in ID Token");
                CompleteAuthenticationError::AuthenticationFailed(
                    "No subject in ID Token".to_string(),
                )
            })?,
            authentication_display_name: email.clone(),
            email: email.parse().map_err(|e| {
                tracing::warn!(err = ?e, "Failed to parse email address from ID Token");
                CompleteAuthenticationError::AuthenticationFailed(
                    "Failed to parse email address from ID Token".to_string(),
                )
            })?,
            display_name,
        })
    }
}

#[cfg(test)]
mod tests {
    use assert2::{check, let_assert};

    use super::*;

    #[test]
    fn decode_authenticated_user_when_valid_token_then_valid_user() {
        let token = GoogleToken {
            id_token: "eyJhbGciOiJIUzUxMiIsImtpZCI6ImVlYTFiMWY0MjgwN2E4Y2MxMzZhMDNhM2MxNmQyOWRiODI5NmRhZjAiLCJ0eXAiOiJKV1QifQ.eyJzdWIiOiIxMjM0NTY3ODkwIiwiZW1haWwiOiJ0ZXN0dXNlckBleGFtcGxlLmNvbSIsIm5hbWUiOiJUZXN0IFVzZXIifQ.zJUutN7GkimWQ-gT8f-BVV15LbtJMs8EmNtO-CpJBN3tl0e-u2VQ793XGPDxPtdSeMWDPNexyYRaz9iceB_0zA".to_owned()
        };

        let result = AuthenticatedUser::try_from(token);

        let_assert!(Ok(user) = result);
        check!(user.authentication_id == AuthenticationId::new("1234567890"));
        check!(user.authentication_display_name == "testuser@example.com");
        check!(user.email == "testuser@example.com".parse().unwrap());
        check!(user.display_name == "Test User");
    }

    #[test]
    fn decode_authenticated_user_when_invalid_token_then_error() {
        let token = GoogleToken {
            id_token: "invalid".to_owned(),
        };

        let result = AuthenticatedUser::try_from(token);

        let_assert!(Err(err) = result);
        check!(
            err == CompleteAuthenticationError::AuthenticationFailed(
                "Failed to parse ID Token".to_string(),
            )
        );
    }

    #[test]
    fn decode_authenticated_user_when_missing_subject_then_error() {
        let token = GoogleToken {
            id_token: "eyJhbGciOiJIUzUxMiIsImtpZCI6ImVlYTFiMWY0MjgwN2E4Y2MxMzZhMDNhM2MxNmQyOWRiODI5NmRhZjAiLCJ0eXAiOiJKV1QifQ.eyJlbWFpbCI6InRlc3R1c2VyQGV4YW1wbGUuY29tIiwibmFtZSI6IlRlc3QgVXNlciJ9.nP6D_bThZ9Oac8odXfenwUGvccblxc79YDBGJRmDM81CHuUQ937o2WSblYSqodpyeSwO0O8ah5sCooQV2sLmIA".to_owned(),
        };

        let result = AuthenticatedUser::try_from(token);

        let_assert!(Err(err) = result);
        check!(
            err == CompleteAuthenticationError::AuthenticationFailed(
                "No subject in ID Token".to_string(),
            )
        );
    }

    #[test]
    fn decode_authenticated_user_when_invalid_email_then_error() {
        let token = GoogleToken {
            id_token: "eyJhbGciOiJIUzUxMiIsImtpZCI6ImVlYTFiMWY0MjgwN2E4Y2MxMzZhMDNhM2MxNmQyOWRiODI5NmRhZjAiLCJ0eXAiOiJKV1QifQ.eyJzdWIiOiIxMjM0NTY3ODkwIiwiZW1haWwiOiIiLCJuYW1lIjoiVGVzdCBVc2VyIn0.ZTCKfyDttUba2_10-Ik638GwEZ2as4ktPKN0jSc7NMDDudsP5qmO7udnKbtt2xlZlAGqn9Ui05kbqFyOWwZ1qw".to_owned(),
        };

        let result = AuthenticatedUser::try_from(token);

        let_assert!(Err(err) = result);
        check!(
            err == CompleteAuthenticationError::AuthenticationFailed(
                "Failed to parse email address from ID Token".to_string(),
            )
        );
    }

    #[test]
    fn decode_authenticated_user_when_missing_email_then_error() {
        let token = GoogleToken {
            id_token: "eyJhbGciOiJIUzUxMiIsImtpZCI6ImVlYTFiMWY0MjgwN2E4Y2MxMzZhMDNhM2MxNmQyOWRiODI5NmRhZjAiLCJ0eXAiOiJKV1QifQ.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IlRlc3QgVXNlciJ9.xf-iAA_HnpEVOWjIv4bh5okAyRvNGTqpeaJOHvMbpjj6D8jauNIogHOs5fZlzJ7kLssryA9XvOqcRsipQAse-Q".to_owned(),
        };

        let result = AuthenticatedUser::try_from(token);

        let_assert!(Err(err) = result);
        check!(
            err == CompleteAuthenticationError::AuthenticationFailed(
                "Failed to parse ID Token".to_string(),
            )
        );
    }

    #[test]
    fn decode_authenticated_user_when_missing_name_then_error() {
        let token = GoogleToken {
            id_token: "eyJhbGciOiJIUzUxMiIsImtpZCI6ImVlYTFiMWY0MjgwN2E4Y2MxMzZhMDNhM2MxNmQyOWRiODI5NmRhZjAiLCJ0eXAiOiJKV1QifQ.eyJzdWIiOiIxMjM0NTY3ODkwIiwiZW1haWwiOiJ0ZXN0dXNlckBleGFtcGxlLmNvbSJ9.WqcPl2-vWXNnHhVC9KHkxI5l_Q54dTxauJPeDqVgC-dH2cDdzEvFMGSo2ZFbNjSXCPRx5sDRG-fh2EWq-wrHwg".to_owned(),
        };

        let result = AuthenticatedUser::try_from(token);

        let_assert!(Err(err) = result);
        check!(
            err == CompleteAuthenticationError::AuthenticationFailed(
                "Failed to parse ID Token".to_string(),
            )
        );
    }
}
