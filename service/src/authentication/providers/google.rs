pub mod config;

use std::collections::HashMap;

use async_trait::async_trait;
use uritemplate::UriTemplate;

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
    fn start_authentication(&self, nonce: &str) -> String {
        UriTemplate::new(&self.auth_url)
            .set("client_id", self.client_id.clone())
            .set("response_type", "code")
            .set("scope", "openid email profile")
            .set("redirect_uri", self.redirect_url.clone())
            .set("state", nonce)
            .build()
    }

    async fn complete_authentication(
        &self,
        _nonce: &str,
        _params: HashMap<String, String>,
    ) -> Result<AuthenticatedUser, CompleteAuthenticationError> {
        Err(CompleteAuthenticationError::Unexpected)
    }
}

#[cfg(test)]
mod tests {
    use assert2::check;

    use super::*;

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
}
