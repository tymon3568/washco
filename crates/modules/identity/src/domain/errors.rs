#[derive(Debug, thiserror::Error)]
pub enum IdentityError {
    #[error("Phone number already registered")]
    PhoneAlreadyExists,

    #[error("Invalid OTP code")]
    InvalidOtp,

    #[error("OTP expired")]
    OtpExpired,

    #[error("Too many failed OTP attempts. Please request a new code.")]
    OtpMaxAttemptsExceeded,

    #[error("User not found")]
    UserNotFound,

    #[error("Account not verified")]
    NotVerified,

    #[error("No OTP request found for this phone")]
    NoOtpRequest,

    #[error("Invalid phone number format")]
    InvalidPhone,

    #[error("Name is required")]
    NameRequired,

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
            | IdentityError::NoOtpRequest
            | IdentityError::InvalidPhone
            | IdentityError::NameRequired => washco_shared::AppError::Validation {
                message: err.to_string(),
            },
            IdentityError::OtpMaxAttemptsExceeded => washco_shared::AppError::RateLimited,
            IdentityError::UserNotFound => washco_shared::AppError::NotFound { entity: "User" },
            IdentityError::NotVerified => washco_shared::AppError::Forbidden,
            IdentityError::Database(e) => {
                washco_shared::AppError::Internal(anyhow::anyhow!("DB: {e}"))
            }
        }
    }
}
