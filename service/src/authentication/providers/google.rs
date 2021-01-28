mod claims;
pub mod config;

use std::{collections::HashMap, convert::TryInto};

use async_trait::async_trait;
use uritemplate::UriTemplate;

use self::claims::GoogleToken;
use super::{AuthenticatedUser, CompleteAuthenticationError, Provider};

/// Authentication provider for working with Google.
pub struct GoogleProvider {
    /// Client ID to use for authentication with Google
    client_id:     String,
    /// Client Secret to use for authentication with Google
    #[allow(dead_code)] // TODO: For now
    client_secret: String,
    /// URL to redirect the user back to after authenticating with Google
    redirect_url:  String,
    /// URL pattern to start authenticating with Google
    auth_url:      String,
    /// URL to call to get an access token from Google
    #[allow(dead_code)] // TODO: For now
    token_url:     String,
}

#[async_trait]
impl Provider for GoogleProvider {
    #[tracing::instrument(skip(self))]
    fn start_authentication(&self, nonce: &str) -> String {
        UriTemplate::new(&self.auth_url)
            .set("client_id", self.client_id.clone())
            .set("response_type", "code")
            .set("scope", "openid email profile")
            .set("redirect_uri", self.redirect_url.clone())
            .set("state", nonce)
            .build()
    }

    #[tracing::instrument(skip(self))]
    async fn complete_authentication(
        &self,
        nonce: &str,
        params: HashMap<String, String>,
    ) -> Result<AuthenticatedUser, CompleteAuthenticationError> {
        let state = params.get("state").ok_or_else(|| {
            tracing::warn!("State parameter is missing");
            CompleteAuthenticationError::MissingParameter("state".to_owned())
        })?;
        if state != nonce {
            tracing::warn!("State parameter is present but has the wrong value");
            return Err(CompleteAuthenticationError::InvalidNonce);
        }

        let auth_code = params.get("code").ok_or_else(|| {
            tracing::warn!("Authorization code parameter is missing");
            CompleteAuthenticationError::MissingParameter("code".to_owned())
        })?;
        let params = [
            ("grant_type", "authorization_code"),
            ("client_id", self.client_id.as_ref()),
            ("client_secret", self.client_secret.as_ref()),
            ("redirect_uri", self.redirect_url.as_ref()),
            ("code", auth_code),
        ];

        let client = reqwest::Client::new();
        let response = client
            .post(&self.token_url)
            .form(&params)
            .send()
            .await
            .map_err(|e| CompleteAuthenticationError::AuthenticationFailed(e.to_string()))?;

        tracing::debug!(response = ?response, "Response from Google");

        if response.status() != reqwest::StatusCode::OK {
            let body = response.text().await.unwrap();
            tracing::warn!("Unsuccessful response received from Google: {}", body);
            return Err(CompleteAuthenticationError::AuthenticationFailed(
                "Unsuccessful response received from Google".to_owned(),
            ));
        }
        let body: GoogleToken = response
            .json()
            .await
            .map_err(|e| CompleteAuthenticationError::AuthenticationFailed(e.to_string()))?;
        tracing::debug!("Response Body from Google: {:?}", body);

        let user = body.try_into()?;
        tracing::debug!(user = ?user, "Authenticated as user");

        Ok(user)
    }
}

#[cfg(test)]
mod tests {
    use assert2::{check, let_assert};

    use super::*;
    use crate::users::AuthenticationId;

    #[test]
    fn start_authentication_url_is_correct() {
        let sut = GoogleProvider {
            client_id: "GoogleClientId".to_string(),
            client_secret: "GoogleClientSecret".to_string(),
            redirect_url: "http://localhost:8000/authentication/google/redirect".to_string(),
            auth_url: "https://accounts.google.com/o/oauth2/v2/auth{?client_id,response_type,scope,redirect_uri,state}".to_string(),
            token_url: "https://www.googleapis.com/oauth2/v4/token".to_string()
        };

        let result = sut.start_authentication("GoogleNonce");
        check!(result == "https://accounts.google.com/o/oauth2/v2/auth?client_id=GoogleClientId&response_type=code&scope=openid%20email%20profile&redirect_uri=http%3A%2F%2Flocalhost%3A8000%2Fauthentication%2Fgoogle%2Fredirect&state=GoogleNonce");
    }

    #[actix_rt::test]
    async fn complete_authentication_when_no_state_then_error() {
        let sut = GoogleProvider {
            client_id: "GoogleClientId".to_string(),
            client_secret: "GoogleClientSecret".to_string(),
            redirect_url: "http://localhost:8000/authentication/google/redirect".to_string(),
            auth_url: "https://accounts.google.com/o/oauth2/v2/auth{?client_id,response_type,scope,redirect_uri,state}".to_string(),
            token_url: "https://www.googleapis.com/oauth2/v4/token".to_string()
        };

        let mut params = HashMap::new();
        params.insert("code".to_string(), "AuthCode".to_string());

        let result = sut.complete_authentication("Nonce", params).await;

        let_assert!(Err(e) = result);
        check!(e == CompleteAuthenticationError::MissingParameter("state".to_owned()));
    }

    #[actix_rt::test]
    async fn complete_authentication_when_no_code_then_error() {
        let sut = GoogleProvider {
            client_id: "GoogleClientId".to_string(),
            client_secret: "GoogleClientSecret".to_string(),
            redirect_url: "http://localhost:8000/authentication/google/redirect".to_string(),
            auth_url: "https://accounts.google.com/o/oauth2/v2/auth{?client_id,response_type,scope,redirect_uri,state}".to_string(),
            token_url: "https://www.googleapis.com/oauth2/v4/token".to_string()
        };

        let mut params = HashMap::new();
        params.insert("state".to_string(), "Nonce".to_string());

        let result = sut.complete_authentication("Nonce", params).await;

        let_assert!(Err(e) = result);
        check!(e == CompleteAuthenticationError::MissingParameter("code".to_owned()));
    }

    #[actix_rt::test]
    async fn complete_authentication_when_wrong_state_then_error() {
        let sut = GoogleProvider {
            client_id: "GoogleClientId".to_string(),
            client_secret: "GoogleClientSecret".to_string(),
            redirect_url: "http://localhost:8000/authentication/google/redirect".to_string(),
            auth_url: "https://accounts.google.com/o/oauth2/v2/auth{?client_id,response_type,scope,redirect_uri,state}".to_string(),
            token_url: "https://www.googleapis.com/oauth2/v4/token".to_string()
        };

        let mut params = HashMap::new();
        params.insert("code".to_string(), "AuthCode".to_string());
        params.insert("state".to_string(), "Wrong".to_string());

        let result = sut.complete_authentication("Nonce", params).await;

        let_assert!(Err(e) = result);
        check!(e == CompleteAuthenticationError::InvalidNonce);
    }

    #[actix_rt::test]
    async fn complete_authentication_when_http_error_then_error() {
        let _ = env_logger::try_init();

        let url = &mockito::server_url();

        let mock = mockito::mock("POST", "/oauth2/v4/token")
            .with_status(400)
            .with_header("content-type", "application/json")
            .with_body(r#"{"error": "invalid_grant", "error_description": "Malformed auth code."}"#)
            .create();

        let sut = GoogleProvider {
            client_id: "GoogleClientId".to_string(),
            client_secret: "GoogleClientSecret".to_string(),
            redirect_url: "http://localhost:8000/authentication/google/redirect".to_string(),
            auth_url: "https://accounts.google.com/o/oauth2/v2/auth{?client_id,response_type,scope,redirect_uri,state}".to_string(),
            token_url: format!("{}/oauth2/v4/token", url)
        };

        let mut params = HashMap::new();
        params.insert("code".to_string(), "AuthCode".to_string());
        params.insert("state".to_string(), "Nonce".to_string());

        let result = sut.complete_authentication("Nonce", params).await;

        let_assert!(Err(e) = result);
        check!(
            e == CompleteAuthenticationError::AuthenticationFailed(
                "Unsuccessful response received from Google".to_string()
            )
        );

        mock.assert();
    }

    #[actix_rt::test]
    async fn complete_authentication_when_empty_response_then_error() {
        let _ = env_logger::try_init();

        let url = &mockito::server_url();

        let mock = mockito::mock("POST", "/oauth2/v4/token")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{}"#)
            .create();

        let sut = GoogleProvider {
            client_id: "GoogleClientId".to_string(),
            client_secret: "GoogleClientSecret".to_string(),
            redirect_url: "http://localhost:8000/authentication/google/redirect".to_string(),
            auth_url: "https://accounts.google.com/o/oauth2/v2/auth{?client_id,response_type,scope,redirect_uri,state}".to_string(),
            token_url: format!("{}/oauth2/v4/token", url)
        };

        let mut params = HashMap::new();
        params.insert("code".to_string(), "AuthCode".to_string());
        params.insert("state".to_string(), "Nonce".to_string());

        let result = sut.complete_authentication("Nonce", params).await;

        let_assert!(Err(e) = result);
        check!(
            e == CompleteAuthenticationError::AuthenticationFailed(
                "error decoding response body: missing field `id_token` at line 1 column 2"
                    .to_string()
            )
        );

        mock.assert();
    }

    #[actix_rt::test]
    async fn complete_authentication_when_malformed_id_token_then_error() {
        let _ = env_logger::try_init();

        let url = &mockito::server_url();

        let mock = mockito::mock("POST", "/oauth2/v4/token")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"id_token": "Malformed"}"#)
            .create();

        let sut = GoogleProvider {
            client_id: "GoogleClientId".to_string(),
            client_secret: "GoogleClientSecret".to_string(),
            redirect_url: "http://localhost:8000/authentication/google/redirect".to_string(),
            auth_url: "https://accounts.google.com/o/oauth2/v2/auth{?client_id,response_type,scope,redirect_uri,state}".to_string(),
            token_url: format!("{}/oauth2/v4/token", url)
        };

        let mut params = HashMap::new();
        params.insert("code".to_string(), "AuthCode".to_string());
        params.insert("state".to_string(), "Nonce".to_string());

        let result = sut.complete_authentication("Nonce", params).await;

        let_assert!(Err(e) = result);
        check!(
            e == CompleteAuthenticationError::AuthenticationFailed(
                "Failed to parse ID Token".to_string()
            )
        );

        mock.assert();
    }

    #[actix_rt::test]
    async fn complete_authentication_when_valid_id_token_then_correct_user() {
        let _ = env_logger::try_init();

        let url = &mockito::server_url();

        let mock = mockito::mock("POST", "/oauth2/v4/token")
            .match_body("grant_type=authorization_code&client_id=GoogleClientId&client_secret=GoogleClientSecret&redirect_uri=http%3A%2F%2Flocalhost%3A8000%2Fauthentication%2Fgoogle%2Fredirect&code=AuthCode")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"id_token": "eyJhbGciOiJIUzUxMiIsImtpZCI6ImVlYTFiMWY0MjgwN2E4Y2MxMzZhMDNhM2MxNmQyOWRiODI5NmRhZjAiLCJ0eXAiOiJKV1QifQ.eyJzdWIiOiIxMjM0NTY3ODkwIiwiZW1haWwiOiJ0ZXN0dXNlckBleGFtcGxlLmNvbSIsIm5hbWUiOiJUZXN0IFVzZXIifQ.zJUutN7GkimWQ-gT8f-BVV15LbtJMs8EmNtO-CpJBN3tl0e-u2VQ793XGPDxPtdSeMWDPNexyYRaz9iceB_0zA"}"#)
            .create();

        let sut = GoogleProvider {
            client_id: "GoogleClientId".to_string(),
            client_secret: "GoogleClientSecret".to_string(),
            redirect_url: "http://localhost:8000/authentication/google/redirect".to_string(),
            auth_url: "https://accounts.google.com/o/oauth2/v2/auth{?client_id,response_type,scope,redirect_uri,state}".to_string(),
            token_url: format!("{}/oauth2/v4/token", url)
        };

        let mut params = HashMap::new();
        params.insert("code".to_string(), "AuthCode".to_string());
        params.insert("state".to_string(), "Nonce".to_string());

        let result = sut.complete_authentication("Nonce", params).await;

        let_assert!(Ok(user) = result);
        check!(user.authentication_id == AuthenticationId::new("1234567890"));
        check!(user.authentication_display_name == "testuser@example.com");
        check!(user.email == "testuser@example.com".parse().unwrap());
        check!(user.display_name == "Test User");

        mock.assert();
    }
}
