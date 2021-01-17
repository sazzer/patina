/// The identity of the authentication service the user is authenticated with.
#[derive(Debug)]
pub struct AuthenticationService(String);

impl AuthenticationService {
    /// Create a new `AuthenticationService` from the provided value.
    ///
    /// # Parameters
    /// - `value` - The name of the authentication service
    pub fn new<S>(value: S) -> Self
    where
        S: Into<String>,
    {
        Self(value.into())
    }
}
