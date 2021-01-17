mod authentication_id;
mod authentication_service;
mod email;
mod user_id;

pub use authentication_id::*;
pub use authentication_service::*;
pub use email::*;
pub use user_id::*;

use crate::model::Resource;

/// A set of authentication details for the user.
#[derive(Debug)]
pub struct Authentication {
    /// The service that the details are for.
    pub service:      AuthenticationService,
    /// The ID of the user at this service.
    pub id:           AuthenticationId,
    /// The display name of these details.
    pub display_name: String,
}

/// Data to represent a user.
#[derive(Debug)]
pub struct UserData {
    /// The display name of the user.
    pub display_name:           String,
    /// The email address of the user.
    pub email:                  Email,
    /// The authentication details of the user.
    pub authentication_details: Vec<Authentication>,
}

/// Representation of a user resource.
pub type UserResource = Resource<UserID, UserData>;
