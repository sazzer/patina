use std::sync::Arc;

use actix_http::http::header::CacheDirective;
use actix_web::web::Data;

use crate::{
    home::model::HomeDocument,
    http::{
        hal::{HalPayload, HalResponse},
        response::Response,
    },
};

/// Endpoint for building the home document.
#[tracing::instrument(skip(home_document))]
pub async fn home(home_document: Data<Arc<HomeDocument>>) -> Response<HalPayload<()>> {
    let payload = home_document.payload.clone();

    HalResponse {
        body: Some(payload),
        cache_control: vec![CacheDirective::Public, CacheDirective::MaxAge(3600)],
        ..HalResponse::default()
    }
    .into()
}
