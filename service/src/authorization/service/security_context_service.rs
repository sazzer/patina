use chrono::Duration;

mod generate;

/// Service implementation for working with Security Contexts.
pub struct SecurityContextService {
    duration: Duration,
}

impl SecurityContextService {
    /// Create a new Security Context Service.
    pub const fn new(duration: Duration) -> Self {
        Self { duration }
    }
}
