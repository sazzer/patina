mod get_user;

use actix_web::web::{get, resource, ServiceConfig};

/// Configure the HTTP Server for the Healthcheck routes
pub fn configure_server(config: &mut ServiceConfig) {
    config.service(resource("/users/{id}").route(get().to(get_user::get_user)));
}
