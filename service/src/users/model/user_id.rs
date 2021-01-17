use uuid::Uuid;

/// The ID of a user.
#[derive(Debug, PartialEq)]
pub struct UserID(Uuid);
