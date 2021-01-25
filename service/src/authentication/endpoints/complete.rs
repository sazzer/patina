use std::{collections::HashMap, sync::Arc};

use actix_http::{HttpMessage, Response};
use actix_web::{
    web::{Data, Path, Query},
    HttpRequest,
};

use crate::{
    authentication::{CompleteAuthenticationError, CompleteAuthenticationUseCase, ProviderId},
    http::problem::{Problem, NOT_FOUND},
};

/// Endpoint to complete authentication with the requested provider.
#[tracing::instrument(
    name = "GET /authentication/{provider}/complete",
    skip(req, authentication_service)
)]
pub async fn complete(
    path: Path<String>,
    params: Query<HashMap<String, String>>,
    req: HttpRequest,
    authentication_service: Data<Arc<dyn CompleteAuthenticationUseCase>>,
) -> Result<Response, Problem> {
    // Ensure the callback is for the expected provider
    let authentication_provider = req.cookie("authentication_provider");
    if let Some(cookie) = authentication_provider {
        if cookie.value() != path.0 {
            // Cookie is present but doesn't match the provider
            return Err(Problem::from(NOT_FOUND).with_extra("why", "Invalid provider"));
        }
    } else {
        // Cookie is missing
        return Err(Problem::from(NOT_FOUND).with_extra("why", "Missing provider"));
    }

    // Extract the nonce that we used when starting authentication
    let authentication_nonce = req
        .cookie("authentication_nonce")
        .map(|c| c.value().to_string())
        .ok_or_else(|| Problem::from(NOT_FOUND).with_extra("why", "Missing nonce"))?; // Cookie is missing

    // Actually complete authentication
    let _authenticated_user = authentication_service
        .complete_authentication(&ProviderId::new(&path.0), &authentication_nonce, params.0)
        .await?;

    Err(Problem::from(NOT_FOUND).with_extra("why", "Success"))
}

impl From<CompleteAuthenticationError> for Problem {
    fn from(err: CompleteAuthenticationError) -> Self {
        Self::from(NOT_FOUND).with_extra("why", format!("{:?}", err))
    }
}
