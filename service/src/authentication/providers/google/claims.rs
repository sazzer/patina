use serde::Deserialize;

/// Representation of the Access Token details received from Google when authenticating a user
#[derive(Debug, Deserialize)]
pub struct GoogleToken {
    /// The OpenID Connect ID Token that contains some user details
    id_token: Option<String>,
}
