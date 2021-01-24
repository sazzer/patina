use std::{collections::HashMap, sync::Arc};

use prometheus::{IntCounterVec, Opts, Registry};

use super::HealthCheckable;

mod check_health;

/// The actual service implementation for healthchecks.
pub struct HealthService {
    /// The components whos health can be checked.
    components:        HashMap<String, Arc<dyn HealthCheckable>>,
    component_counter: IntCounterVec,
    system_counter:    IntCounterVec,
}

impl HealthService {
    /// Create a new health service.
    pub fn new(
        components: HashMap<String, Arc<dyn HealthCheckable>>,
        prometheus: &Registry,
    ) -> Self {
        let component_counter = IntCounterVec::new(
            Opts::new("health_components", "Health of individual components"),
            &["component", "status"],
        )
        .expect("metric can be created");
        let system_counter = IntCounterVec::new(
            Opts::new("health_system", "Health of the entire system"),
            &["status"],
        )
        .expect("metric can be created");

        prometheus
            .register(Box::new(component_counter.clone()))
            .unwrap();
        prometheus
            .register(Box::new(system_counter.clone()))
            .unwrap();

        Self {
            components,
            component_counter,
            system_counter,
        }
    }
}
