use axum::{
    extract::FromRequestParts,
    http::{header::AUTHORIZATION, request::Parts},
};
use shared::AppError;
use uuid::Uuid;

use crate::services::auth::AuthService;

/// Add `current_user: CurrentUser` to a handler to require a valid access token.
/// Requests without a valid `Authorization: Bearer <token>` header get 401.
pub struct CurrentUser {
    pub id: Uuid,
    pub role: String,
}

impl FromRequestParts<AuthService> for CurrentUser {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        service: &AuthService,
    ) -> Result<Self, Self::Rejection> {
        let token = match read_bearer_token(parts) {
            Some(token) => token,
            None => return Err(AppError::Unauthorized),
        };

        let claims = service.verify_access_token(token)?;

        Ok(CurrentUser {
            id: claims.sub,
            role: claims.role,
        })
    }
}

fn read_bearer_token(parts: &Parts) -> Option<&str> {
    let header = parts.headers.get(AUTHORIZATION)?;
    let value = header.to_str().ok()?;
    value.strip_prefix("Bearer ")
}
