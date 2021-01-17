use super::repository::Repository;

mod get_user;

/// Service for working with users.
pub struct UsersService {
    repository: Repository,
}

impl UsersService {
    /// Create a new instance of the users service.
    pub const fn new(repository: Repository) -> Self {
        Self { repository }
    }
}
