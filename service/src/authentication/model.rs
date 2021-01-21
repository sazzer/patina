/// Identifier of an authentication provider.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ProviderId(pub(super) String);

impl ProviderId {
    /// Construct a new Provider ID.
    pub fn new<S>(value: S) -> Self
    where
        S: Into<String>,
    {
        Self(value.into())
    }
}
