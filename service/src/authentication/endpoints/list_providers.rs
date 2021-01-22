use std::sync::Arc;

use actix_http::http::header::CacheDirective;
use actix_web::web::Data;

use crate::{
    authentication::ListProvidersUseCase,
    http::{
        hal::{HalPayload, HalResponse},
        response::Response,
    },
};

/// Endpoint to get a list of authentication providers.
#[tracing::instrument(skip(service))]
pub async fn list_providers(
    service: Data<Arc<dyn ListProvidersUseCase>>,
) -> Response<HalPayload<()>> {
    let providers = service.list_providers();

    let mut payload = HalPayload::new(()).with_link("self", "/authentication");

    for provider in providers {
        payload = payload.with_link(
            "tag:patina,2021,rels/authentication/start",
            provider.clone(),
        );
    }

    HalResponse {
        body: Some(payload),
        cache_control: vec![CacheDirective::Public, CacheDirective::MaxAge(3600)],
        ..HalResponse::default()
    }
    .into()
}
