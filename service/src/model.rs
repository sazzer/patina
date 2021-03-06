use chrono::{DateTime, Utc};
use uuid::Uuid;

/// The identity of some persisted resource.
#[derive(Debug)]
pub struct Identity<I> {
    /// The actual ID of the resource.
    pub id:      I,
    /// The version of the resource.
    pub version: Uuid,
    /// When the resource wsa created.
    pub created: DateTime<Utc>,
    /// When the resource was last updated.
    pub updated: DateTime<Utc>,
}

/// The details of a persisted resource.
#[derive(Debug)]
pub struct Resource<I, D> {
    /// The identity of the resource.
    pub identity: Identity<I>,
    /// The data of the resource.
    pub data:     D,
}

impl<I> Default for Identity<I>
where
    I: Default,
{
    fn default() -> Self {
        let now = Utc::now();
        let version = Uuid::new_v4();

        Self {
            id: I::default(),
            version,
            created: now,
            updated: now,
        }
    }
}
