use super::model::SystemHealthResponse;
use crate::http::response::Response;

/// Endpoint for checking the health of the system.
pub async fn check_health() -> Response<SystemHealthResponse> {
    "".into()
}
