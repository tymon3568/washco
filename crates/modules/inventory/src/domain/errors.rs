#[derive(Debug, thiserror::Error)]
pub enum InventoryError {
    #[error("Material not found")]
    NotFound,

    #[error("Insufficient stock: current {current}, requested {requested}")]
    InsufficientStock { current: i64, requested: i64 },

    #[error("Invalid quantity: {0}")]
    InvalidQuantity(String),

    #[error("Norm already exists for this service and material")]
    NormAlreadyExists,

    #[error("Invalid category: {0}")]
    InvalidCategory(String),

    #[error("Invalid transaction type: {0}")]
    InvalidTransactionType(String),

    #[error("Database error")]
    Database(#[from] sqlx::Error),
}

impl From<InventoryError> for washco_shared::AppError {
    fn from(err: InventoryError) -> Self {
        match err {
            InventoryError::NotFound => washco_shared::AppError::NotFound { entity: "Material" },
            InventoryError::InsufficientStock { .. } => washco_shared::AppError::Conflict {
                message: err.to_string(),
            },
            InventoryError::InvalidQuantity(_) => washco_shared::AppError::Validation {
                message: err.to_string(),
            },
            InventoryError::NormAlreadyExists => washco_shared::AppError::Conflict {
                message: err.to_string(),
            },
            InventoryError::InvalidCategory(_) => washco_shared::AppError::Validation {
                message: err.to_string(),
            },
            InventoryError::InvalidTransactionType(_) => washco_shared::AppError::Validation {
                message: err.to_string(),
            },
            InventoryError::Database(e) => {
                washco_shared::AppError::Internal(anyhow::anyhow!("DB: {e}"))
            }
        }
    }
}
