use async_trait::async_trait;

use super::UsersService;
use crate::{
    model::Identity,
    users::{Authentication, AuthenticationService, GetUserUseCase, UserData, UserID, UserResource},
};

#[async_trait]
impl GetUserUseCase for UsersService {
    async fn get_user_by_id(&self, user_id: UserID) -> Option<UserResource> {
        if (user_id == "3f92b3b0-4716-449a-a159-beabf3b59d99".parse().unwrap()) {
            None
        } else {
            Some(UserResource {
                identity: Identity {
                    id: user_id,
                    version: "a76b376a-9ca9-4b90-bb20-c5c5133d2ba7".parse().unwrap(),
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
}
