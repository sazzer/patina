use std::{collections::HashMap, sync::Arc};

use actix_web::web::ServiceConfig;

pub use super::providers::google::config::Settings as GoogleSettings;
use super::{
    endpoints::{configure_server, home_document_links},
    providers::Provider,
    service::AuthenticationService,
    ListProvidersUseCase, ProviderId,
};
use crate::{home::Contributor, http::hal::Link, server::Configurer};

/// Builder for building the authentication component.
pub struct Builder {
    /// The map of authentication providers.
    pub(super) providers: HashMap<ProviderId, Arc<dyn Provider>>,
}

impl Builder {
    /// Build the authentication component.
    pub fn build(self) -> Arc<Component> {
        tracing::debug!("Built authentication service");

        let service = Arc::new(AuthenticationService::new(self.providers));
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

    Builder {
        providers: HashMap::new(),
    }
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
