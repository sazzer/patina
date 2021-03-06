use std::{collections::HashMap, sync::Arc};

use actix_web::web::ServiceConfig;
use prometheus::Registry;

use super::{
    endpoints::{configure_server, home_document_links},
    service::HealthService,
    CheckHealthUseCase, HealthCheckable,
};
use crate::{home::Contributor, http::hal::Link, server::Configurer};

/// Builder for building the health checks component.
pub struct Builder {
    components: HashMap<String, Arc<dyn HealthCheckable>>,
}

impl Builder {
    /// Register a new component with the health checker.
    pub fn with_component<S>(mut self, name: S, component: Arc<dyn HealthCheckable>) -> Self
    where
        S: Into<String>,
    {
        self.components.insert(name.into(), component);

        self
    }

    /// Build the component.
    pub fn build(self, prometheus: &Registry) -> Arc<Component> {
        tracing::debug!("Built health service");

        let service = Arc::new(HealthService::new(self.components, prometheus));
        Arc::new(Component { service })
    }
}

/// Configuration component for the Healthchecks.
pub struct Component {
    pub service: Arc<HealthService>,
}

/// Provide a builder for building the component.
pub fn builder() -> Builder {
    tracing::debug!("Building health checks");

    Builder {
        components: HashMap::new(),
    }
}

impl Configurer for Component {
    fn configure_server(&self, config: &mut ServiceConfig) {
        config.data(self.service.clone() as Arc<dyn CheckHealthUseCase>);

        configure_server(config);
    }
}

impl Contributor for Component {
    fn get_links(&self) -> Vec<(&'static str, Link)> {
        home_document_links()
    }
}
