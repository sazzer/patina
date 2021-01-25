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

    async fn get_user_by_authentication(
        &self,
        authentication_service: crate::users::AuthenticationService,
        authentication_id: crate::users::AuthenticationId,
    ) -> Option<UserResource> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use assert2::{check, let_assert};
    use patina_testdatabase::seed::SeedUser;

    use super::*;
    use crate::{
        database::test::TestDatabase,
        users::{repository::Repository, service::UsersService},
    };

    #[actix_rt::test]
    async fn get_user_by_id_when_user_is_known_then_found() {
        let seed_user = SeedUser {
            user_id: "384a7b7f-8ec2-4f73-9dae-4eb4f7b178b3".parse().unwrap(),
            version: "a76b376a-9ca9-4b90-bb20-c5c5133d2ba7".parse().unwrap(),
            display_name: "Test User".to_string(),
            email: "test@example.com".to_string(),
            ..SeedUser::default()
        };

        let test_database = TestDatabase::new().await;
        test_database.test_database.seed(&seed_user).await;

        let service = UsersService::new(Repository::new(test_database.database));

        let user = service
            .get_user_by_id("384a7b7f-8ec2-4f73-9dae-4eb4f7b178b3".parse().unwrap())
            .await;

        let_assert!(Some(user) = user);
        check!(user.identity.version == "a76b376a-9ca9-4b90-bb20-c5c5133d2ba7".parse().unwrap());
        check!(user.data.email == "test@example.com".parse().unwrap());
        check!(user.data.display_name == "Test User");
    }
}
