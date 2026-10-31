use std::fmt::Display;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug)]
pub enum AppError {
    NotFound,
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
            Self::NotFound => (StatusCode::NOT_FOUND, "Resource not found"),
            Self::Internal => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "An internal server error occurred",
            ),
            Self::Database => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "A database operation failed",
            ),
        };

        (status, message.to_string()).into_response()
    }
}
