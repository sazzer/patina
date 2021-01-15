use std::collections::HashMap;

use async_trait::async_trait;

/// Trait that other components can implement if they can check their health.
#[async_trait]
pub trait HealthCheckable: Send + Sync {
    /// Check the health of the component.
    async fn check_health(&self) -> Result<(), String>;
}

/// Representation of the health of a single component.
#[derive(Debug, PartialEq)]
pub enum ComponentHealth {
    /// The component is healthy.
    Healthy,
    /// The component is unhealthy, and has a reason as to why.
    Unhealthy(String),
}

/// Representation of the health of the entire system.
#[derive(Debug)]
pub struct SystemHealth {
    /// The health of the individual components in the system.
    pub components: HashMap<String, ComponentHealth>,
}

impl SystemHealth {
    /// Test if the overall system health is healthy.
    /// The system is healthy if and only if all of the components are healthy. If any are unhealthy then
    /// the overall system is unhealthy.
    pub fn healthy(&self) -> bool {
        self.components.iter().all(|(_, value)| value == &ComponentHealth::Healthy)
    }
}

#[cfg(test)]
mod tests {
    use assert2::check;

    use super::*;

    #[test]
    fn empty_system_healthy() {
        let components = HashMap::new();
        let system = SystemHealth { components };

        check!(system.healthy() == true);
    }

    #[test]
    fn healthy_system_healthy() {
        let mut components = HashMap::new();
        components.insert("healthy".to_string(), ComponentHealth::Healthy);
        let system = SystemHealth { components };

        check!(system.healthy() == true);
    }

    #[test]
    fn unhealthy_system_not_healthy() {
        let mut components = HashMap::new();
        components.insert("unhealthy".to_string(), ComponentHealth::Unhealthy("Oops".to_string()));
        let system = SystemHealth { components };

        check!(system.healthy() == false);
    }

    #[test]
    fn mixed_system_not_healthy() {
        let mut components = HashMap::new();
        components.insert("healthy".to_string(), ComponentHealth::Healthy);
        components.insert("unhealthy".to_string(), ComponentHealth::Unhealthy("Oops".to_string()));
        let system = SystemHealth { components };

        check!(system.healthy() == false);
    }
}
