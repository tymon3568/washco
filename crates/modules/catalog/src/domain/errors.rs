#[derive(Debug, thiserror::Error)]
pub enum CatalogError {
    #[error("Service not found")]
    ServiceNotFound,

    #[error("Duplicate service name for this location")]
    DuplicateName,

    #[error("Invalid vehicle type: {0}")]
    InvalidVehicleType(String),

    #[error("Database error")]
    Database(#[from] sqlx::Error),
}

impl From<CatalogError> for washco_shared::AppError {
    fn from(err: CatalogError) -> Self {
        match err {
            CatalogError::ServiceNotFound => {
                washco_shared::AppError::NotFound { entity: "Service" }
            }
            CatalogError::DuplicateName => washco_shared::AppError::Conflict {
                message: err.to_string(),
            },
            CatalogError::InvalidVehicleType(_) => washco_shared::AppError::Validation {
                message: err.to_string(),
            },
            CatalogError::Database(e) => {
                washco_shared::AppError::Internal(anyhow::anyhow!("DB: {e}"))
            }
        }
    }
}
