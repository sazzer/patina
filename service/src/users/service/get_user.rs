use async_trait::async_trait;

use super::UsersService;
use crate::{
    model::Identity,
    users::{Authentication, AuthenticationService, GetUserUseCase, UserData, UserID, UserResource},
};

#[async_trait]
impl GetUserUseCase for UsersService {
    async fn get_user_by_id(&self, user_id: UserID) -> Option<UserResource> {
        Some(UserResource {
            identity: Identity {
                id: user_id,
                ..Identity::default()
            },
            data:     UserData {
                email:                  "test@example.com".parse().unwrap(),
                display_name:           "Test User".to_string(),
                authentication_details: vec![Authentication {
                    id:           "123456".parse().unwrap(),
                    service:      AuthenticationService::new("fake"),
                    display_name: "test@example.com".to_string(),
                }],
            },
        })
    }
}
