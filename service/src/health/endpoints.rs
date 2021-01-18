mod check_health;
mod model;

use actix_web::web::{get, resource, ServiceConfig};

use crate::http::hal::Link;

/// Configure the HTTP Server for the Healthcheck routes
pub fn configure_server(config: &mut ServiceConfig) {
    config.service(resource("/health").route(get().to(check_health::check_health)));
}

/// Produce the links that should be contributed to the home document.
pub fn home_document_links() -> Vec<(&'static str, Link)> {
    vec![("tag:patina,2021:rels/health", Link::from("/health"))]
}
