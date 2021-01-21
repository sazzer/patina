use async_trait::async_trait;

use super::Provider;

/// Authentication provider for working with Google.
pub struct GoogleProvider {}

impl GoogleProvider {
    /// Create a new instance of the Google authentication provider.
    pub const fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Provider for GoogleProvider {}
