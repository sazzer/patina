use std::sync::Arc;

/// Configuration component for working with users.
pub struct Component {}

/// Construct a new authorization component.
pub fn new() -> Arc<Component> {
    tracing::debug!("Building authorization service");
    Arc::new(Component {})
}
