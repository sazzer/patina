use uuid::Uuid;

use crate::model::Resource;

/// The ID of a user.
#[derive(Debug, PartialEq)]
pub struct UserID(Uuid);

/// The email address of a user.
#[derive(Debug)]
pub struct Email(String);

/// The identity of the authentication service the user is authenticated with.
#[derive(Debug)]
pub struct AuthenticationService(String);

/// The identity of the user at the authentication service.
#[derive(Debug)]
pub struct AuthenticationId(String);

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
