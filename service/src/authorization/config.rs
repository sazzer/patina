use std::sync::Arc;

use super::service::security_context_service::SecurityContextService;

/// Configuration component for working with users.
pub struct Component {
    _security_context_service: Arc<SecurityContextService>,
}

/// Construct a new authorization component.
pub fn new() -> Arc<Component> {
    tracing::debug!("Building authorization service");

    let security_context_service = Arc::new(SecurityContextService::new());

    Arc::new(Component {
        _security_context_service: security_context_service,
    })
}
