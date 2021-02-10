use std::sync::Arc;

use chrono::Duration;

use super::service::{access_token::AccessTokenService, security_context::SecurityContextService};

/// Configuration component for working with users.
pub struct Component {
    pub security_context_service: Arc<SecurityContextService>,

    pub access_token_service: Arc<AccessTokenService>,
}

/// Construct a new authorization component.
pub fn new() -> Arc<Component> {
    tracing::debug!("Building authorization service");

    // TODO: Move these into configuration
    let duration = Duration::days(365);
    let secret = "secret";

    let security_context_service = Arc::new(SecurityContextService::new(duration));
    let access_token_service = Arc::new(AccessTokenService::new(secret));

    Arc::new(Component {
        security_context_service,
        access_token_service,
    })
}
