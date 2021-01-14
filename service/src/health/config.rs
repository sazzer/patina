use std::sync::Arc;

use super::endpoints::configure_server;
use crate::server::Configurer;
use actix_web::web::ServiceConfig;

/// Configuration component for the Healthchecks.
pub struct Component {}

impl Component {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {})
    }
}

impl Configurer for Component {
    fn configure_server(&self, config: &mut ServiceConfig) {
        configure_server(config);
    }
}
