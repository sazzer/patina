use std::sync::Arc;

use actix_web::web::ServiceConfig;

use super::{endpoints::configure_server, service::HealthService, CheckHealthUseCase};
use crate::server::Configurer;

/// Builder for building the health checks component.
pub struct Builder {}

impl Builder {
    /// Build the component.
    pub fn build(self) -> Arc<Component> {
        let service = Arc::new(HealthService::new());
        Arc::new(Component { service })
    }
}

/// Configuration component for the Healthchecks.
pub struct Component {
    pub service: Arc<HealthService>,
}

impl Component {
    /// Provide a builder for building the component.
    pub fn builder() -> Builder {
        Builder {}
    }
}

impl Configurer for Component {
    fn configure_server(&self, config: &mut ServiceConfig) {
        config.data(self.service.clone() as Arc<dyn CheckHealthUseCase>);

        configure_server(config);
    }
}
