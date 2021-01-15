use std::collections::BTreeMap;

use actix_http::http::StatusCode;
use serde::Serialize;

use crate::{
    health::{ComponentHealth, SystemHealth},
    http::response::Response,
};

/// Reponse model representation of the System Health
#[derive(Debug, Serialize)]
pub struct SystemHealthResponse {
    pub healthy:    bool,
    pub components: BTreeMap<String, ComponentHealthResponse>,
}

/// Reponse model representation of the health of a single component
#[derive(Debug, Serialize)]
pub struct ComponentHealthResponse {
    pub healthy: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl From<ComponentHealth> for ComponentHealthResponse {
    fn from(health: ComponentHealth) -> Self {
        match health {
            ComponentHealth::Healthy => Self {
                healthy: true,
                message: None,
            },
            ComponentHealth::Unhealthy(msg) => Self {
                healthy: false,
                message: Some(msg),
            },
        }
    }
}

impl From<SystemHealth> for Response<SystemHealthResponse> {
    fn from(health: SystemHealth) -> Self {
        let mut body = SystemHealthResponse {
            healthy:    health.healthy(),
            components: BTreeMap::new(),
        };

        for (name, status) in health.components {
            body.components.insert(name, status.into());
        }

        let status = if body.healthy { StatusCode::OK } else { StatusCode::SERVICE_UNAVAILABLE };

        Self {
            body: Some(body),
            status,
            ..Self::default()
        }
    }
}
