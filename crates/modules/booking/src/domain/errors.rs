#[derive(Debug, thiserror::Error)]
pub enum BookingError {
    #[error("Booking not found")]
    NotFound,

    #[error("Invalid status transition from {from} to {to}")]
    InvalidTransition { from: String, to: String },

    #[error("Time slot conflict: the requested slot is fully booked")]
    TimeSlotConflict,

    #[error("Cannot book a date in the past")]
    PastBookingDate,

    #[error("Database error")]
    Database(#[from] sqlx::Error),
}

impl From<BookingError> for washco_shared::AppError {
    fn from(err: BookingError) -> Self {
        match err {
            BookingError::NotFound => washco_shared::AppError::NotFound { entity: "Booking" },
            BookingError::InvalidTransition { .. } => washco_shared::AppError::Validation {
                message: err.to_string(),
            },
            BookingError::TimeSlotConflict => washco_shared::AppError::Conflict {
                message: err.to_string(),
            },
            BookingError::PastBookingDate => washco_shared::AppError::Validation {
                message: err.to_string(),
            },
            BookingError::Database(e) => {
                washco_shared::AppError::Internal(anyhow::anyhow!("DB: {e}"))
            }
        }
    }
}
