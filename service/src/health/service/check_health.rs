use std::collections::HashMap;

use async_trait::async_trait;

use super::HealthService;
use crate::health::{CheckHealthUseCase, ComponentHealth, SystemHealth};

#[async_trait]
impl CheckHealthUseCase for HealthService {
    #[tracing::instrument(skip(self))]
    async fn check_health(&self) -> SystemHealth {
        let mut components = HashMap::new();

        for (name, component) in &self.components {
            let health = match component.check_health().await {
                Ok(()) => ComponentHealth::Healthy,
                Err(msg) => ComponentHealth::Unhealthy(msg),
            };

            tracing::info!(name = ?name, health = ?health, "Component health");
            self.component_counter
                .with_label_values(&[
                    name,
                    match health {
                        ComponentHealth::Healthy => "healthy",
                        ComponentHealth::Unhealthy(_) => "unhealthy",
                    },
                ])
                .inc();

            components.insert(name.clone(), health);
        }

        let result = SystemHealth { components };

        self.system_counter
            .with_label_values(&[if result.healthy() {
                "healthy"
            } else {
                "unhealthy"
            }])
            .inc();

        result
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use assert2::{check, let_assert};
    use prometheus::{proto::MetricFamily, Registry};

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
    async fn health_service_given_no_components_then_is_healthy() {
        let components = HashMap::new();
        let registry = Registry::new();
        let service = HealthService::new(components, &registry);

        let result = service.check_health().await;

        check!(result.healthy() == true);
        check!(result.components.len() == 0);
    }

    #[actix_rt::test]
    async fn health_service_given_healthy_components_then_is_healthy() {
        let mut components: HashMap<String, Arc<dyn HealthCheckable>> = HashMap::new();
        components.insert("healthy".to_string(), Arc::new(MockComponent(Ok(()))));
        let registry = Registry::new();
        let service = HealthService::new(components, &registry);

        let result = service.check_health().await;

        check!(result.healthy() == true);
        check!(result.components.len() == 1);
        check!(result.components["healthy"] == ComponentHealth::Healthy);
    }

    #[actix_rt::test]
    async fn health_service_given_unhealthy_components_then_is_unhealthy() {
        let mut components: HashMap<String, Arc<dyn HealthCheckable>> = HashMap::new();
        components.insert(
            "unhealthy".to_string(),
            Arc::new(MockComponent(Err("Oops".to_string()))),
        );
        let registry = Registry::new();
        let service = HealthService::new(components, &registry);

        let result = service.check_health().await;

        check!(result.healthy() == false);
        check!(result.components.len() == 1);
        check!(result.components["unhealthy"] == ComponentHealth::Unhealthy("Oops".to_string()));
    }

    #[actix_rt::test]
    async fn health_service_given_mixed_components_then_is_unhealthy() {
        let mut components: HashMap<String, Arc<dyn HealthCheckable>> = HashMap::new();
        components.insert("healthy".to_string(), Arc::new(MockComponent(Ok(()))));
        components.insert(
            "unhealthy".to_string(),
            Arc::new(MockComponent(Err("Oops".to_string()))),
        );
        let registry = Registry::new();
        let service = HealthService::new(components, &registry);

        let result = service.check_health().await;

        check!(result.healthy() == false);
        check!(result.components.len() == 2);
        check!(result.components["healthy"] == ComponentHealth::Healthy);
        check!(result.components["unhealthy"] == ComponentHealth::Unhealthy("Oops".to_string()));
    }

    #[actix_rt::test]
    async fn health_service_given_no_components_then_metrics_recorded() {
        let components = HashMap::new();
        let registry = Registry::new();
        let service = HealthService::new(components, &registry);

        service.check_health().await;

        check_metric_counter_value(&registry, "health_system", &[("status", "healthy")], 1.0);
        check_metric_counter_absent(&registry, "health_system", &[("status", "unhealthy")]);

        let metrics = registry.gather();
        let health_component = metrics.iter().find(|m| m.get_name() == "health_component");
        check!(health_component == None);
    }

    #[actix_rt::test]
    async fn health_service_given_healthy_component_then_metrics_recorded() {
        let mut components: HashMap<String, Arc<dyn HealthCheckable>> = HashMap::new();
        components.insert("healthy".to_string(), Arc::new(MockComponent(Ok(()))));
        let registry = Registry::new();
        let service = HealthService::new(components, &registry);

        service.check_health().await;

        check_metric_counter_value(&registry, "health_system", &[("status", "healthy")], 1.0);
        check_metric_counter_absent(&registry, "health_system", &[("status", "unhealthy")]);

        check_metric_counter_value(
            &registry,
            "health_components",
            &[("status", "healthy"), ("component", "healthy")],
            1.0,
        );
        check_metric_counter_absent(
            &registry,
            "health_components",
            &[("status", "unhealthy"), ("component", "healthy")],
        );
    }

    #[actix_rt::test]
    async fn health_service_given_unhealthy_component_then_metrics_recorded() {
        let mut components: HashMap<String, Arc<dyn HealthCheckable>> = HashMap::new();
        components.insert(
            "unhealthy".to_string(),
            Arc::new(MockComponent(Err("Oops".to_string()))),
        );
        let registry = Registry::new();
        let service = HealthService::new(components, &registry);

        service.check_health().await;

        check_metric_counter_value(&registry, "health_system", &[("status", "unhealthy")], 1.0);
        check_metric_counter_absent(&registry, "health_system", &[("status", "healthy")]);

        check_metric_counter_value(
            &registry,
            "health_components",
            &[("status", "unhealthy"), ("component", "unhealthy")],
            1.0,
        );
        check_metric_counter_absent(
            &registry,
            "health_components",
            &[("status", "healthy"), ("component", "unhealthy")],
        );
    }

    #[actix_rt::test]
    async fn health_service_given_mixed_components_then_metrics_recorded() {
        let mut components: HashMap<String, Arc<dyn HealthCheckable>> = HashMap::new();
        components.insert("healthy".to_string(), Arc::new(MockComponent(Ok(()))));
        components.insert(
            "unhealthy".to_string(),
            Arc::new(MockComponent(Err("Oops".to_string()))),
        );
        let registry = Registry::new();
        let service = HealthService::new(components, &registry);

        service.check_health().await;

        check_metric_counter_value(&registry, "health_system", &[("status", "unhealthy")], 1.0);
        check_metric_counter_absent(&registry, "health_system", &[("status", "healthy")]);

        check_metric_counter_value(
            &registry,
            "health_components",
            &[("status", "healthy"), ("component", "healthy")],
            1.0,
        );
        check_metric_counter_absent(
            &registry,
            "health_components",
            &[("status", "unhealthy"), ("component", "healthy")],
        );

        check_metric_counter_value(
            &registry,
            "health_components",
            &[("status", "unhealthy"), ("component", "unhealthy")],
            1.0,
        );
        check_metric_counter_absent(
            &registry,
            "health_components",
            &[("status", "healthy"), ("component", "unhealthy")],
        );
    }

    fn check_metric_counter_value(
        registry: &Registry,
        metric_name: &str,
        labels: &[(&str, &str)],
        expected: f64,
    ) {
        let metric_family = find_metric_family(registry, metric_name);

        let metric = metric_family.get_metric().iter().find(|m| {
            let mut is_match = true;

            for label in labels {
                is_match &= m
                    .get_label()
                    .iter()
                    .any(|l| l.get_name() == label.0 && l.get_value() == label.1);
            }

            is_match
        });

        let_assert!(Some(metric) = metric);
        check!((metric.get_counter().get_value() - expected) < f64::EPSILON);
    }

    fn check_metric_counter_absent(
        registry: &Registry,
        metric_name: &str,
        labels: &[(&str, &str)],
    ) {
        let metric_family = find_metric_family(registry, metric_name);

        let metric = metric_family.get_metric().iter().find(|m| {
            let mut is_match = true;

            for label in labels {
                is_match &= m
                    .get_label()
                    .iter()
                    .any(|l| l.get_name() == label.0 && l.get_value() == label.1);
            }

            is_match
        });

        check!(metric.is_none());
    }

    fn find_metric_family(registry: &Registry, metric_name: &str) -> MetricFamily {
        let metric_families = registry.gather();

        let metric_family = metric_families
            .into_iter()
            .find(|m| m.get_name() == metric_name);
        let_assert!(Some(metric_family) = metric_family);

        metric_family
    }
}
