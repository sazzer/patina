use std::sync::Arc;

use chrono::Duration;

use super::service::security_context_service::SecurityContextService;

/// Configuration component for working with users.
pub struct Component {
    #[allow(dead_code)] // TODO: For now
    security_context_service: Arc<SecurityContextService>,
}

/// Construct a new authorization component.
pub fn new() -> Arc<Component> {
    tracing::debug!("Building authorization service");

    let duration = Duration::days(365);

    let security_context_service = Arc::new(SecurityContextService::new(duration));

    Arc::new(Component {
        security_context_service,
    })
}
