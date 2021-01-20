use std::sync::Arc;

use actix_web::web::ServiceConfig;

use super::endpoints::{configure_server, home_document_links};
use crate::{home::Contributor, http::hal::Link, server::Configurer};

/// Configuration component for authentication.
pub struct Component {}

/// Construct a new authentication component.
pub fn new() -> Arc<Component> {
    tracing::debug!("Building authentication service");

    Arc::new(Component {})
}

impl Configurer for Component {
    fn configure_server(&self, config: &mut ServiceConfig) {
        configure_server(config);
    }
}

impl Contributor for Component {
    fn get_links(&self) -> Vec<(&'static str, Link)> {
        home_document_links()
    }
}
