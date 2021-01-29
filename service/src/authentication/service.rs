mod complete;
mod list;
mod start;

use std::{collections::HashMap, sync::Arc};

use super::{providers::Provider, ProviderId};
use crate::users::GetUserUseCase;

/// Service for managing authentication.
pub struct AuthenticationService {
    /// The authentication providers to use.
    providers: HashMap<ProviderId, Arc<dyn Provider>>,

    /// Use case for getting the user with the provided authentication details.
    get_user_user_case: Arc<dyn GetUserUseCase>,
}

impl AuthenticationService {
    /// Create a new instance of the authentication service.
    pub fn new(
        providers: HashMap<ProviderId, Arc<dyn Provider>>,
        get_user_user_case: Arc<dyn GetUserUseCase>,
    ) -> Self {
        Self {
            providers,
            get_user_user_case,
        }
    }
}
