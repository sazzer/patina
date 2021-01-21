use std::sync::Arc;

use actix_web::web::ServiceConfig;

use super::{
    endpoints::{configure_server, home_document_links},
    service::AuthenticationService,
    ListProvidersUseCase,
};
use crate::{home::Contributor, http::hal::Link, server::Configurer};

/// Builder for building the authentication component.
pub struct Builder {}

impl Builder {
    /// Build the authentication component.
    #[allow(clippy::unused_self)] // TODO: For now
    pub fn build(self) -> Arc<Component> {
        tracing::debug!("Built authentication service");

        let service = Arc::new(AuthenticationService::new());
        Arc::new(Component { service })
    }
}
/// Configuration component for authentication.
pub struct Component {
    service: Arc<AuthenticationService>,
}

/// Construct a new authentication component.
pub fn builder() -> Builder {
    tracing::debug!("Building authentication service");

    Builder {}
}

impl Configurer for Component {
    fn configure_server(&self, config: &mut ServiceConfig) {
        config.data(self.service.clone() as Arc<dyn ListProvidersUseCase>);

        configure_server(config);
    }
}

impl Contributor for Component {
    fn get_links(&self) -> Vec<(&'static str, Link)> {
        home_document_links()
    }
}
