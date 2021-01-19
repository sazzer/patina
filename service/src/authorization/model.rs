use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Representation of the principal that is being authenticated.
#[derive(Debug, PartialEq)]
pub enum Principal {
    User(String),
}

/// Representation of the unique ID of a Security Context.
#[derive(Debug, PartialEq)]
pub struct SecurityContextId(String);

impl Default for SecurityContextId {
    fn default() -> Self {
        let id = Uuid::new_v4();

        Self(id.to_string())
    }
}

/// A Security context issued to a principal for access to resources.
#[derive(Debug, PartialEq)]
pub struct SecurityContext {
    pub id:        SecurityContextId,
    pub principal: Principal,
    pub issued:    DateTime<Utc>,
    pub expires:   DateTime<Utc>,
}

/// Representation of an Access Token - a security context that has been signed and encoded.
#[derive(Debug)]
pub struct AccessToken(String);
