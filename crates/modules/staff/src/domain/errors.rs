#[derive(Debug, thiserror::Error)]
pub enum StaffError {
    #[error("Staff profile not found")]
    NotFound,

    #[error("Staff profile already exists for this user at this location")]
    AlreadyExists,

    #[error("Shift conflicts with an existing shift")]
    ShiftConflict,

    #[error("Invalid skill level: {0}")]
    InvalidSkillLevel(String),

    #[error("Database error")]
    Database(#[from] sqlx::Error),
}

impl From<StaffError> for washco_shared::AppError {
    fn from(err: StaffError) -> Self {
        match err {
            StaffError::NotFound => washco_shared::AppError::NotFound {
                entity: "StaffProfile",
            },
            StaffError::AlreadyExists => washco_shared::AppError::Conflict {
                message: err.to_string(),
            },
            StaffError::ShiftConflict => washco_shared::AppError::Conflict {
                message: err.to_string(),
            },
            StaffError::InvalidSkillLevel(_) => washco_shared::AppError::Validation {
                message: err.to_string(),
            },
            StaffError::Database(e) => {
                washco_shared::AppError::Internal(anyhow::anyhow!("DB: {e}"))
            }
        }
    }
}
