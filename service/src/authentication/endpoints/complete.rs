use std::collections::HashMap;

use actix_http::Response;
use actix_web::web::{Path, Query};

use crate::http::problem::{Problem, NOT_FOUND};

/// Endpoint to complete authentication with the requested provider.
#[tracing::instrument(name = "GET /authentication/{provider}/complete", skip())]
pub async fn complete(
    path: Path<String>,
    params: Query<HashMap<String, String>>,
) -> Result<Response, Problem> {
    Err(NOT_FOUND.into())
}
