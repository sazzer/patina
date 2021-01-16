use std::sync::Arc;

use actix_web::web::ServiceConfig;

use super::endpoints::configure_server;
use crate::{database::Database, server::Configurer};

/// Configuration component for working with users.
pub struct Component {}

/// Construct a new users component.
#[allow(clippy::needless_pass_by_value)] // TODO: FIX
pub fn new(_database: Arc<dyn Database>) -> Arc<Component> {
    tracing::debug!("Building users service");

    Arc::new(Component {})
}

impl Configurer for Component {
    fn configure_server(&self, config: &mut ServiceConfig) {
        configure_server(config);
    }
}
