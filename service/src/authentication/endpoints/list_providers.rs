use actix_http::http::header::CacheDirective;

use crate::http::{
    hal::{HalPayload, HalResponse},
    response::Response,
};

/// Endpoint to get a list of authentication providers.
pub async fn list_providers() -> Response<HalPayload<()>> {
    let payload = HalPayload::new(()).with_link("self", "/authentication");

    HalResponse {
        body: Some(payload),
        cache_control: vec![CacheDirective::Public, CacheDirective::MaxAge(3600)],
        ..HalResponse::default()
    }
    .into()
}
