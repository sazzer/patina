use async_trait::async_trait;

use super::UsersService;
use crate::users::{GetUserUseCase, UserID, UserResource};

#[async_trait]
impl GetUserUseCase for UsersService {
    #[tracing::instrument(skip(self))]
    async fn get_user_by_id(&self, user_id: UserID) -> Option<UserResource> {
        tracing::debug!("Finding user");

        let user = self.repository.get_by_id(user_id).await;

        tracing::debug!(user = ?user, "Found user");

        user
    }
}
