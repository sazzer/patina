use async_trait::async_trait;

use super::SystemHealth;

/// Use case for checking the health of the system/
#[async_trait]
pub trait CheckHealthUseCase {
    /// Check the health of the system.
    async fn check_health(&self) -> SystemHealth;
}
