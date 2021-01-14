use std::collections::BTreeMap;

use actix_http::http::StatusCode;
use serde::Serialize;

use crate::http::response::Response;

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
    pub message: Option<String>,
}

impl From<&str> for Response<SystemHealthResponse> {
    fn from(_: &str) -> Self {
        let body = SystemHealthResponse {
            healthy:    false,
            components: BTreeMap::new(),
        };

        Self {
            body: Some(body),
            status: StatusCode::SERVICE_UNAVAILABLE,
            ..Self::default()
        }
    }
}
