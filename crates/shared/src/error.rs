use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Authentication required")]
    Unauthorized,

    #[error("Insufficient permissions")]
    Forbidden,

    #[error("Validation error: {message}")]
    Validation { message: String },

    #[error("{entity} not found")]
    NotFound { entity: &'static str },

    #[error("Conflict: {message}")]
    Conflict { message: String },

    #[error("Rate limited")]
    RateLimited,

    #[error("Internal server error")]
    Internal(#[from] anyhow::Error),

    #[error("Database error")]
    Database(#[from] sqlx::Error),
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    code: u16,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, self.to_string()),
            AppError::Forbidden => (StatusCode::FORBIDDEN, self.to_string()),
            AppError::Validation { .. } => (StatusCode::BAD_REQUEST, self.to_string()),
            AppError::NotFound { .. } => (StatusCode::NOT_FOUND, self.to_string()),
            AppError::Conflict { .. } => (StatusCode::CONFLICT, self.to_string()),
            AppError::RateLimited => (StatusCode::TOO_MANY_REQUESTS, self.to_string()),
            AppError::Internal(err) => {
                tracing::error!("Internal error: {err:?}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                )
            }
            AppError::Database(err) => {
                tracing::error!("Database error: {err:?}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                )
            }
        };

        let body = ErrorResponse {
            error: message,
            code: status.as_u16(),
        };

        (status, axum::Json(body)).into_response()
    }
}
