#[derive(Debug, thiserror::Error)]
pub enum PaymentError {
    #[error("Payment not found")]
    NotFound,

    #[error("Payment already completed")]
    AlreadyPaid,

    #[error("Invalid payment amount: {message}")]
    InvalidAmount { message: String },

    #[error("Invalid payment status transition: {from} -> {to}")]
    InvalidStatus { from: String, to: String },

    #[error("Database error")]
    Database(#[from] sqlx::Error),
}

impl From<PaymentError> for washco_shared::AppError {
    fn from(err: PaymentError) -> Self {
        match err {
            PaymentError::NotFound => washco_shared::AppError::NotFound { entity: "Payment" },
            PaymentError::AlreadyPaid => washco_shared::AppError::Conflict {
                message: err.to_string(),
            },
            PaymentError::InvalidAmount { .. } => washco_shared::AppError::Validation {
                message: err.to_string(),
            },
            PaymentError::InvalidStatus { .. } => washco_shared::AppError::Validation {
                message: err.to_string(),
            },
            PaymentError::Database(e) => {
                washco_shared::AppError::Internal(anyhow::anyhow!("DB: {e}"))
            }
        }
    }
}
