#[derive(Debug, thiserror::Error)]
pub enum PromotionError {
    #[error("Promotion not found")]
    NotFound,

    #[error("Promotion code already exists")]
    CodeAlreadyExists,

    #[error("Promotion has expired")]
    Expired,

    #[error("Promotion max uses reached")]
    MaxUsesReached,

    #[error("Minimum order amount not met")]
    MinOrderNotMet,

    #[error("Promotion not applicable to this location")]
    NotApplicableToLocation,

    #[error("Invalid promotion code")]
    InvalidCode,

    #[error("Database error")]
    Database(#[from] sqlx::Error),
}

impl From<PromotionError> for washco_shared::AppError {
    fn from(err: PromotionError) -> Self {
        match err {
            PromotionError::NotFound => washco_shared::AppError::NotFound {
                entity: "Promotion",
            },
            PromotionError::CodeAlreadyExists => washco_shared::AppError::Conflict {
                message: err.to_string(),
            },
            PromotionError::Expired
            | PromotionError::MaxUsesReached
            | PromotionError::MinOrderNotMet
            | PromotionError::NotApplicableToLocation
            | PromotionError::InvalidCode => washco_shared::AppError::Validation {
                message: err.to_string(),
            },
            PromotionError::Database(e) => {
                washco_shared::AppError::Internal(anyhow::anyhow!("DB: {e}"))
            }
        }
    }
}
