use actix_http::http::header::{CacheDirective, EntityTag};
use serde::Serialize;

use crate::{
    http::hal::{HalPayload, HalResponse},
    users::{AuthenticationId, AuthenticationService, Email, UserResource},
};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticationResponse {
    pub service:      AuthenticationService,
    pub user_id:      AuthenticationId,
    pub display_name: String,
}

/// Response model to represent a User.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserResponse {
    pub display_name:    String,
    pub email:           Email,
    pub authentications: Vec<AuthenticationResponse>,
}

impl From<UserResource> for HalResponse<UserResponse> {
    fn from(user: UserResource) -> Self {
        let payload = HalPayload::new(UserResponse {
            display_name:    user.data.display_name,
            email:           user.data.email,
            authentications: user
                .data
                .authentication_details
                .into_iter()
                .map(|a| AuthenticationResponse {
                    service:      a.service,
                    user_id:      a.id,
                    display_name: a.display_name,
                })
                .collect(),
        })
        .with_link("self", user.identity.id);

        Self {
            body: Some(payload),
            cache_control: vec![CacheDirective::Public, CacheDirective::MaxAge(3600)],
            etag: Some(EntityTag::strong(user.identity.version.to_string())),
            ..Self::default()
        }
    }
}
