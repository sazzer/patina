use std::sync::Arc;

use actix_web::web::ServiceConfig;

use super::{endpoints::configure_server, model::HomeDocument, Contributor};
use crate::{http::hal::HalPayload, server::Configurer};

/// Builder for building the home document component.
pub struct Builder {
    contributors: Vec<Arc<dyn Contributor>>,
}

impl Builder {
    /// Add a component to the home document.
    ///
    /// # Parameters
    /// - `contributor` - The component that will be contributing to the home document.
    pub fn with_component(mut self, contributor: Arc<dyn Contributor>) -> Self {
        self.contributors.push(contributor);

        self
    }

    /// Build the component.
    pub fn build(self) -> Arc<Component> {
        tracing::debug!("Built home document");

        let mut payload = HalPayload::new(());

        for contributor in self.contributors {
            let links = contributor.get_links();

            for (name, link) in links {
                payload = payload.with_link(name, link);
            }
        }

        let home_document = Arc::new(HomeDocument { payload });

        Arc::new(Component { home_document })
    }
}

/// Configuration component for the Healthchecks.
pub struct Component {
    home_document: Arc<HomeDocument>,
}

/// Provide a builder for building the component.
pub fn builder() -> Builder {
    tracing::debug!("Building home document");

    Builder {
        contributors: vec![],
    }
}

impl Configurer for Component {
    fn configure_server(&self, config: &mut ServiceConfig) {
        config.data(self.home_document.clone());

        configure_server(config);
    }
}
