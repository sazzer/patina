use actix_web::web::ServiceConfig;

use crate::http::hal::Link;

/// Configure the HTTP Server for the Authentication routes
pub fn configure_server(config: &mut ServiceConfig) {}

/// Produce the links that should be contributed to the home document.
pub fn home_document_links() -> Vec<(&'static str, Link)> {
    vec![(
        "tag:patina,2021:rels/authentication",
        Link::from("/authentication"),
    )]
}
