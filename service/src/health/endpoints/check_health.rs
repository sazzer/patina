use std::sync::Arc;

use actix_web::web::Data;

use super::model::SystemHealthResponse;
use crate::{health::CheckHealthUseCase, http::response::Response};

/// Endpoint for checking the health of the system.
pub async fn check_health(
    health_service: Data<Arc<dyn CheckHealthUseCase>>,
) -> Response<SystemHealthResponse> {
    let health = health_service.check_health().await;

    health.into()
}
