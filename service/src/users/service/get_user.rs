use async_trait::async_trait;

use super::UsersService;
use crate::users::{GetUserUseCase, UserID, UserResource};

#[async_trait]
impl GetUserUseCase for UsersService {
    async fn get_user_by_id(&self, user_id: UserID) -> Option<UserResource> {
        self.repository.get_by_id(user_id).await
    }
}
