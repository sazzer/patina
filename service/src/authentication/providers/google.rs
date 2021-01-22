pub mod config;

use async_trait::async_trait;

use super::Provider;

/// Authentication provider for working with Google.
#[allow(dead_code)] // TODO: For now
pub struct GoogleProvider {
    /// Client ID to use for authentication with Google
    client_id:     String,
    /// Client Secret to use for authentication with Google
    client_secret: String,
    /// URL to redirect the user back to after authenticating with Google
    redirect_url:  String,
    /// URL pattern to start authenticating with Google
    auth_url:      String,
    /// URL to call to get an access token from Google
    token_url:     String,
}

#[async_trait]
impl Provider for GoogleProvider {
    fn start_authentication(&self, _nonce: String) -> url::Url {
        todo!()
    }
}
