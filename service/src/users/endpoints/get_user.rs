use std::sync::Arc;

use actix_web::web::{Data, Path};

use super::model::UserResponse;
use crate::{
    http::{
        hal::HalPayload,
        problem::{Problem, NOT_FOUND},
        response::Response,
    },
    users::{GetUserUseCase, UserID},
};

/// Endpoint for getting a single User by ID.
pub async fn get_user(path: Path<String>, user_service: Data<Arc<dyn GetUserUseCase>>) -> Result<Response<HalPayload<UserResponse>>, Problem> {
    let user_id: UserID = path.0.parse().map_err(|e| {
        tracing::warn!(e = ?e, user_id = ?path.0, "Failed to parse User ID");
        Problem::new(NOT_FOUND)
    })?;

    let user = user_service.get_user_by_id(user_id).await.ok_or_else(|| Problem::new(NOT_FOUND))?;

    Ok(user.into())
}
