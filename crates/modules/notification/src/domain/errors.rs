#[derive(Debug, thiserror::Error)]
pub enum NotificationError {
    #[error("Notification template not found")]
    TemplateNotFound,

    #[error("Template render error: {message}")]
    RenderError { message: String },

    #[error("Database error")]
    Database(#[from] sqlx::Error),
}

impl From<NotificationError> for washco_shared::AppError {
    fn from(err: NotificationError) -> Self {
        match err {
            NotificationError::TemplateNotFound => washco_shared::AppError::NotFound {
                entity: "NotificationTemplate",
            },
            NotificationError::RenderError { .. } => washco_shared::AppError::Validation {
                message: err.to_string(),
            },
            NotificationError::Database(e) => {
                washco_shared::AppError::Internal(anyhow::anyhow!("DB: {e}"))
            }
        }
    }
}
