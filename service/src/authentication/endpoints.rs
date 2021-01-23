mod list_providers;
mod model;
mod start;

use actix_web::web::{get, resource, ServiceConfig};

use crate::http::hal::Link;

/// Configure the HTTP Server for the Authentication routes
pub fn configure_server(config: &mut ServiceConfig) {
    config.service(resource("/authentication").route(get().to(list_providers::list_providers)));
    config.service(resource("/authentication/{provider}").route(get().to(start::start)));
}

/// Produce the links that should be contributed to the home document.
pub fn home_document_links() -> Vec<(&'static str, Link)> {
    vec![(
        "tag:patina,2021:rels/authentication",
        Link::from("/authentication"),
    )]
}
