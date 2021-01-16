use async_trait::async_trait;

use super::UsersService;
use crate::users::{GetUserUseCase, UserID, UserResource};

#[async_trait]
impl GetUserUseCase for UsersService {
    async fn get_user_by_id(&self, _user_id: UserID) -> Option<UserResource> {
        None
    }
}
