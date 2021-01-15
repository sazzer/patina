use std::{collections::HashMap, sync::Arc};

use super::HealthCheckable;

mod check_health;

/// The actual service implementation for healthchecks.
pub struct HealthService {
    /// The components whos health can be checked.
    components: HashMap<String, Arc<dyn HealthCheckable>>,
}

impl HealthService {
    /// Create a new health service.
    pub fn new(components: HashMap<String, Arc<dyn HealthCheckable>>) -> Self {
        Self { components }
    }
}
