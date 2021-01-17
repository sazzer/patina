use std::collections::HashMap;

use async_trait::async_trait;

use super::HealthService;
use crate::health::{CheckHealthUseCase, ComponentHealth, SystemHealth};

#[async_trait]
impl CheckHealthUseCase for HealthService {
    async fn check_health(&self) -> SystemHealth {
        let mut components = HashMap::new();

        for (name, component) in &self.components {
            let health = match component.check_health().await {
                Ok(()) => ComponentHealth::Healthy,
                Err(msg) => ComponentHealth::Unhealthy(msg),
            };

            tracing::info!(name = ?name, health = ?health, "Component health");

            components.insert(name.clone(), health);
        }

        SystemHealth { components }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use assert2::check;

    use super::*;
    use crate::health::HealthCheckable;

    struct MockComponent(Result<(), String>);

    #[async_trait]
    impl HealthCheckable for MockComponent {
        async fn check_health(&self) -> Result<(), String> {
            self.0.clone()
        }
    }

    #[actix_rt::test]
    async fn health_service_given_no_components_is_healthy() {
        let components = HashMap::new();
        let service = HealthService::new(components);

        let result = service.check_health().await;

        check!(result.healthy() == true);
        check!(result.components.len() == 0);
    }

    #[actix_rt::test]
    async fn health_service_given_healthy_components_is_healthy() {
        let mut components: HashMap<String, Arc<dyn HealthCheckable>> = HashMap::new();
        components.insert("healthy".to_string(), Arc::new(MockComponent(Ok(()))));
        let service = HealthService::new(components);

        let result = service.check_health().await;

        check!(result.healthy() == true);
        check!(result.components.len() == 1);
        check!(result.components["healthy"] == ComponentHealth::Healthy);
    }

    #[actix_rt::test]
    async fn health_service_given_unhealthy_components_is_unhealthy() {
        let mut components: HashMap<String, Arc<dyn HealthCheckable>> = HashMap::new();
        components.insert(
            "unhealthy".to_string(),
            Arc::new(MockComponent(Err("Oops".to_string()))),
        );
        let service = HealthService::new(components);

        let result = service.check_health().await;

        check!(result.healthy() == false);
        check!(result.components.len() == 1);
        check!(result.components["unhealthy"] == ComponentHealth::Unhealthy("Oops".to_string()));
    }

    #[actix_rt::test]
    async fn health_service_given_mixed_components_is_unhealthy() {
        let mut components: HashMap<String, Arc<dyn HealthCheckable>> = HashMap::new();
        components.insert("healthy".to_string(), Arc::new(MockComponent(Ok(()))));
        components.insert(
            "unhealthy".to_string(),
            Arc::new(MockComponent(Err("Oops".to_string()))),
        );
        let service = HealthService::new(components);

        let result = service.check_health().await;

        check!(result.healthy() == false);
        check!(result.components.len() == 2);
        check!(result.components["healthy"] == ComponentHealth::Healthy);
        check!(result.components["unhealthy"] == ComponentHealth::Unhealthy("Oops".to_string()));
    }
}
