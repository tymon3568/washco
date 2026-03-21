#[derive(Debug, thiserror::Error)]
pub enum LocationError {
    #[error("Location not found")]
    NotFound,

    #[error("Location slug already exists")]
    SlugConflict,

    #[error("Invalid coordinates")]
    InvalidCoordinates,

    #[error("Location is not active")]
    NotActive,

    #[error("Invalid queue mode: {0}")]
    InvalidQueueMode(String),

    #[error("Invalid status: {0}")]
    InvalidStatus(String),

    #[error("Database error")]
    Database(#[from] sqlx::Error),
}

impl From<LocationError> for washco_shared::AppError {
    fn from(err: LocationError) -> Self {
        match err {
            LocationError::NotFound => washco_shared::AppError::NotFound { entity: "Location" },
            LocationError::SlugConflict => washco_shared::AppError::Conflict {
                message: err.to_string(),
            },
            LocationError::InvalidCoordinates
            | LocationError::InvalidQueueMode(_)
            | LocationError::InvalidStatus(_) => washco_shared::AppError::Validation {
                message: err.to_string(),
            },
            LocationError::NotActive => washco_shared::AppError::Validation {
                message: err.to_string(),
            },
            LocationError::Database(e) => {
                washco_shared::AppError::Internal(anyhow::anyhow!("DB: {e}"))
            }
        }
    }
}
