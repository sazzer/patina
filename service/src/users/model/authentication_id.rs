use serde::{Deserialize, Serialize};

/// The identity of the user at the authentication service.
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthenticationId(String);

impl AuthenticationId {
    /// Create a new `AuthenticationId` from the provided value.
    ///
    /// # Parameters
    /// - `value` - The ID at the authentication service
    #[allow(dead_code)] // TODO: For now
    pub fn new<S>(value: S) -> Self
    where
        S: Into<String>,
    {
        Self(value.into())
    }
}
