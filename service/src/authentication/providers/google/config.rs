use std::sync::Arc;

use super::GoogleProvider;
use crate::authentication::{config::Builder, ProviderId};

/// Default value for the Auth URL if not provided.
const AUTH_URL_DEFAULT_VALUE: &str =
    "https://accounts.google.com/o/oauth2/v2/auth{?client_id,response_type,scope,redirect_uri,state}";

/// Default value for the Token URL if not provided.
const TOKEN_URL_DEFAULT_VALUE: &str = "https://www.googleapis.com/oauth2/v4/token";

/// Settings required for authenticating against Google.
#[derive(Debug)]
pub struct Settings {
    /// Client ID to use for authentication with Google
    pub client_id:     String,
    /// Client Secret to use for authentication with Google
    pub client_secret: String,
    /// URL to redirect the user back to after authenticating with Google
    pub redirect_url:  String,
    /// URL pattern to start authenticating with Google
    pub auth_url:      Option<String>,
    /// URL to call to get an access token from Google
    pub token_url:     Option<String>,
}

impl Builder {
    pub fn with_google(mut self, settings: &Option<Settings>) -> Self {
        if let Some(settings) = settings {
            tracing::debug!(settings = ?settings, "Adding Google authentication provider");

            self.providers.insert(
                ProviderId::new("google"),
                Arc::new(GoogleProvider {
                    client_id:     settings.client_id.clone(),
                    client_secret: settings.client_secret.clone(),
                    redirect_url:  settings.redirect_url.clone(),
                    auth_url:      settings
                        .auth_url
                        .clone()
                        .unwrap_or_else(|| AUTH_URL_DEFAULT_VALUE.to_string()),
                    token_url:     settings
                        .token_url
                        .clone()
                        .unwrap_or_else(|| TOKEN_URL_DEFAULT_VALUE.to_string()),
                }),
            );
        } else {
            tracing::debug!("Google authentication provider is not configured");
        }

        self
    }
}
