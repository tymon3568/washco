#[derive(Debug, thiserror::Error)]
pub enum CustomerError {
    #[error("Customer not found")]
    NotFound,

    #[error("Customer with this phone already exists")]
    AlreadyExists,

    #[error("Vehicle not found")]
    VehicleNotFound,

    #[error("Membership has expired")]
    MembershipExpired,

    #[error("Invalid vehicle type: {0}")]
    InvalidVehicleType(String),

    #[error("Invalid membership type: {0}")]
    InvalidMembershipType(String),

    #[error("Invalid customer segment: {0}")]
    InvalidSegment(String),

    #[error("Membership not found")]
    MembershipNotFound,

    #[error("Membership usage limit reached")]
    MembershipUsageLimitReached,

    #[error("Database error")]
    Database(#[from] sqlx::Error),
}

impl From<CustomerError> for washco_shared::AppError {
    fn from(err: CustomerError) -> Self {
        match err {
            CustomerError::NotFound => washco_shared::AppError::NotFound { entity: "Customer" },
            CustomerError::AlreadyExists => washco_shared::AppError::Conflict {
                message: err.to_string(),
            },
            CustomerError::VehicleNotFound => {
                washco_shared::AppError::NotFound { entity: "Vehicle" }
            }
            CustomerError::MembershipExpired => washco_shared::AppError::Validation {
                message: err.to_string(),
            },
            CustomerError::InvalidVehicleType(_) => washco_shared::AppError::Validation {
                message: err.to_string(),
            },
            CustomerError::InvalidMembershipType(_) => washco_shared::AppError::Validation {
                message: err.to_string(),
            },
            CustomerError::InvalidSegment(_) => washco_shared::AppError::Validation {
                message: err.to_string(),
            },
            CustomerError::MembershipNotFound => washco_shared::AppError::NotFound {
                entity: "Membership",
            },
            CustomerError::MembershipUsageLimitReached => washco_shared::AppError::Validation {
                message: err.to_string(),
            },
            CustomerError::Database(e) => {
                washco_shared::AppError::Internal(anyhow::anyhow!("DB: {e}"))
            }
        }
    }
}
