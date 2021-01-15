use async_trait::async_trait;

use super::HealthService;
use crate::health::CheckHealthUseCase;

#[async_trait]
impl CheckHealthUseCase for HealthService {
    async fn check_health(&self) -> crate::health::SystemHealth {
        todo!()
    }
}
