#[derive(Debug, thiserror::Error)]
pub enum IdentityError {
    #[error("Phone number already registered")]
    PhoneAlreadyExists,

    #[error("Invalid OTP code")]
    InvalidOtp,

    #[error("OTP expired")]
    OtpExpired,

    #[error("User not found")]
    UserNotFound,

    #[error("Account not verified")]
    NotVerified,

    #[error("No OTP request found for this phone")]
    NoOtpRequest,

    #[error("Database error")]
    Database(#[from] sqlx::Error),
}

impl From<IdentityError> for washco_shared::AppError {
    fn from(err: IdentityError) -> Self {
        match err {
            IdentityError::PhoneAlreadyExists => washco_shared::AppError::Conflict {
                message: err.to_string(),
            },
            IdentityError::InvalidOtp
            | IdentityError::OtpExpired
            | IdentityError::NoOtpRequest => washco_shared::AppError::Validation {
                message: err.to_string(),
            },
            IdentityError::UserNotFound => washco_shared::AppError::NotFound { entity: "User" },
            IdentityError::NotVerified => washco_shared::AppError::Forbidden,
            IdentityError::Database(e) => {
                washco_shared::AppError::Internal(anyhow::anyhow!("DB: {e}"))
            }
        }
    }
}
