#[derive(Debug, thiserror::Error)]
pub enum QueueError {
    #[error("Queue entry not found")]
    EntryNotFound,

    #[error("Queue is full")]
    QueueFull,

    #[error("Invalid status transition from {from} to {to}")]
    InvalidTransition { from: String, to: String },

    #[error("No available bays")]
    NoBaysAvailable,

    #[error("Database error")]
    Database(#[from] sqlx::Error),
}

impl From<QueueError> for washco_shared::AppError {
    fn from(err: QueueError) -> Self {
        match err {
            QueueError::EntryNotFound => washco_shared::AppError::NotFound {
                entity: "QueueEntry",
            },
            QueueError::QueueFull | QueueError::NoBaysAvailable => {
                washco_shared::AppError::Conflict {
                    message: err.to_string(),
                }
            }
            QueueError::InvalidTransition { .. } => washco_shared::AppError::Validation {
                message: err.to_string(),
            },
            QueueError::Database(e) => {
                washco_shared::AppError::Internal(anyhow::anyhow!("DB: {e}"))
            }
        }
    }
}
