use biscuit::jws::Secret;

mod generate;
mod validate;

/// Service implementation for working with access tokens.
pub struct AccessTokenService {
    signing_secret: Secret,
}

impl AccessTokenService {
    /// Create a new Access Token service.
    ///
    /// # Parameters
    /// - `secret` - The secret to use for signing the access tokens.
    pub fn new<S>(secret: S) -> Self
    where
        S: Into<String>,
    {
        let signing_secret = Secret::Bytes(secret.into().into_bytes());

        Self { signing_secret }
    }
}
