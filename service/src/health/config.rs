use std::sync::Arc;

use crate::server::Configurer;

/// Configuration component for the Healthchecks.
pub struct Component {}

impl Component {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {})
    }
}

impl Configurer for Component {
    fn configure_server(&self, _config: &mut actix_web::web::ServiceConfig) {}
}
