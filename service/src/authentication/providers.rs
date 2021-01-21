pub mod google;

use async_trait::async_trait;

/// Trait representing an authentication provider.
#[async_trait]
pub trait Provider: Sync + Send {}
