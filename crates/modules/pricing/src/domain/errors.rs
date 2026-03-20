#[derive(Debug, thiserror::Error)]
pub enum PricingError {
    #[error("Pricing rule not found")]
    RuleNotFound,

    #[error("Invalid rule type: {0}")]
    InvalidRuleType(String),

    #[error("Conflicting rule: {0}")]
    ConflictingRule(String),

    #[error("Database error")]
    Database(#[from] sqlx::Error),
}

impl From<PricingError> for washco_shared::AppError {
    fn from(err: PricingError) -> Self {
        match err {
            PricingError::RuleNotFound => washco_shared::AppError::NotFound {
                entity: "PricingRule",
            },
            PricingError::InvalidRuleType(_) => washco_shared::AppError::Validation {
                message: err.to_string(),
            },
            PricingError::ConflictingRule(_) => washco_shared::AppError::Conflict {
                message: err.to_string(),
            },
            PricingError::Database(e) => {
                washco_shared::AppError::Internal(anyhow::anyhow!("DB: {e}"))
            }
        }
    }
}
