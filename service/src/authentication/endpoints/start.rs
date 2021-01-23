use std::sync::Arc;

use actix_http::Response;
use actix_web::web::{Data, Path};

use crate::{
    authentication::{ProviderId, StartAuthenticationError, StartAuthenticationUseCase},
    http::problem::{Problem, NOT_FOUND},
};

/// Endpoint to start authentication with the requested provider.
#[tracing::instrument(skip(service))]
pub async fn start(
    path: Path<String>,
    service: Data<Arc<dyn StartAuthenticationUseCase>>,
) -> Result<Response, Problem> {
    let provider_id = ProviderId::new(path.0);

    let redirect_details = service.start_authentication(&provider_id)?;

    Ok(Response::SeeOther()
        .set_header("Location", redirect_details.redirect_url)
        .finish())
}

impl From<StartAuthenticationError> for Problem {
    fn from(err: StartAuthenticationError) -> Self {
        match err {
            StartAuthenticationError::UnknownProvider => NOT_FOUND.into(),
        }
    }
}
