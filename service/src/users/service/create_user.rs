use async_trait::async_trait;

use super::UsersService;
use crate::users::{CreateUserError, CreateUserUseCase, UserData, UserResource};

#[async_trait]
impl CreateUserUseCase for UsersService {
    async fn create_user(&self, _user: UserData) -> Result<UserResource, CreateUserError> {
        Err(CreateUserError::Unexpected)
    }
}
