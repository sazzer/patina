use std::sync::Arc;

use actix_http::{cookie::Cookie, Response};
use actix_web::web::{Data, Path};

use crate::{
    authentication::{ProviderId, StartAuthenticationError, StartAuthenticationUseCase},
    http::problem::{Problem, NOT_FOUND},
};

/// Endpoint to start authentication with the requested provider.
#[tracing::instrument(name = "GET /authentication/{provider}", skip(service))]
pub async fn start(
    path: Path<String>,
    service: Data<Arc<dyn StartAuthenticationUseCase>>,
) -> Result<Response, Problem> {
    let provider_id = ProviderId::new(&path.0);

    let redirect_details = service.start_authentication(&provider_id)?;

    Ok(Response::SeeOther()
        .set_header("Location", redirect_details.redirect_url)
        .cookie(
            Cookie::build("authentication_provider", path.0)
                .http_only(true)
                .finish(),
        )
        .cookie(
            Cookie::build("authentication_nonce", redirect_details.nonce)
                .http_only(true)
                .finish(),
        )
        .finish())
}

impl From<StartAuthenticationError> for Problem {
    fn from(err: StartAuthenticationError) -> Self {
        match err {
            StartAuthenticationError::UnknownProvider => NOT_FOUND.into(),
        }
    }
}
