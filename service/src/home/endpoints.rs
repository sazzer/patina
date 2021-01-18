mod home;

use actix_web::web::{get, resource, ServiceConfig};

/// Configure the HTTP Server for the home document
pub fn configure_server(config: &mut ServiceConfig) {
    config.service(resource("/").route(get().to(home::home)));
}
