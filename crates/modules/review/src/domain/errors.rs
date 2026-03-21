#[derive(Debug, thiserror::Error)]
pub enum ReviewError {
    #[error("Review not found")]
    NotFound,

    #[error("A review already exists for this queue entry")]
    AlreadyReviewed,

    #[error("Invalid rating: must be between 1 and 5")]
    InvalidRating,

    #[error("Database error")]
    Database(#[from] sqlx::Error),
}

impl From<ReviewError> for washco_shared::AppError {
    fn from(err: ReviewError) -> Self {
        match err {
            ReviewError::NotFound => washco_shared::AppError::NotFound { entity: "Review" },
            ReviewError::AlreadyReviewed => washco_shared::AppError::Conflict {
                message: err.to_string(),
            },
            ReviewError::InvalidRating => washco_shared::AppError::Validation {
                message: err.to_string(),
            },
            ReviewError::Database(e) => {
                washco_shared::AppError::Internal(anyhow::anyhow!("DB: {e}"))
            }
        }
    }
}
