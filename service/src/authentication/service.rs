mod list;
mod start;

use std::{collections::HashMap, sync::Arc};

use super::{providers::Provider, ProviderId};

/// Service for managing authentication.
pub struct AuthenticationService {
    /// The authentication providers to use.
    providers: HashMap<ProviderId, Arc<dyn Provider>>,
}

impl AuthenticationService {
    /// Create a new instance of the authentication service.
    pub fn new(providers: HashMap<ProviderId, Arc<dyn Provider>>) -> Self {
        Self { providers }
    }
}
