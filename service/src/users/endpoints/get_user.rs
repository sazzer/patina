use std::sync::Arc;

use actix_web::web::Data;

use crate::{
    http::problem::{Problem, NOT_FOUND},
    users::GetUserUseCase,
};

/// Endpoint for getting a single User by ID.
pub async fn get_user(_health_service: Data<Arc<dyn GetUserUseCase>>) -> Result<String, Problem> {
    Err(NOT_FOUND.into())
}
