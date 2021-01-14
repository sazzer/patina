mod check_health;

use actix_web::web::{get, resource, ServiceConfig};

/// Configure the HTTP Server for the Healthcheck routes
pub fn configure_server(config: &mut ServiceConfig) {
    config.service(resource("/health").route(get().to(check_health::check_health)));
}
