use std::fmt::Display;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug)]
pub enum AppError {
    NotFound,
    Unauthorized,
    Conflict(String),
    Validation(String),
    Internal,
    Database,
}

impl AppError {
    pub fn internal(error: impl Display) -> Self {
        eprintln!("Internal error: {error}");
        Self::Internal
    }

    pub fn database(error: impl Display) -> Self {
        eprintln!("Database error: {error}");
        Self::Database
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            Self::NotFound => (StatusCode::NOT_FOUND, "Resource not found".to_string()),
            Self::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized".to_string()),
            Self::Conflict(message) => (StatusCode::CONFLICT, message),
            Self::Validation(message) => (StatusCode::UNPROCESSABLE_ENTITY, message),
            Self::Internal => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "An internal server error occurred".to_string(),
            ),
            Self::Database => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "A database operation failed".to_string(),
            ),
        };

        (status, message).into_response()
    }
}
