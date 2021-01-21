use actix_http::http::header::CacheDirective;

use crate::{
    authentication::ProviderId,
    http::{
        hal::{HalPayload, HalResponse},
        response::Response,
    },
};

/// Endpoint to get a list of authentication providers.
pub async fn list_providers() -> Response<HalPayload<()>> {
    let mut providers = vec![
        ProviderId::new("google"),
        ProviderId::new("facebook"),
        ProviderId::new("twitter"),
    ];
    providers.sort();

    let mut payload = HalPayload::new(()).with_link("self", "/authentication");

    for provider in providers {
        payload = payload.with_link("tag:patina,2021,rels/authentication/start", provider);
    }

    HalResponse {
        body: Some(payload),
        cache_control: vec![CacheDirective::Public, CacheDirective::MaxAge(3600)],
        ..HalResponse::default()
    }
    .into()
}
