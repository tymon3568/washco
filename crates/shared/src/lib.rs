pub mod auth;
pub mod error;
pub mod money;
pub mod pagination;

// Re-export commonly used types
pub use auth::{Claims, JwtConfig, Role, TenantContext};
pub use error::AppError;

/// Resolve tenant_id from a location_id (for public/driver endpoints).
pub async fn resolve_tenant_for_location(
    pool: &sqlx::PgPool,
    location_id: uuid::Uuid,
) -> Result<uuid::Uuid, AppError> {
    let tenant_id: Option<uuid::Uuid> = sqlx::query_scalar(
        "SELECT tenant_id FROM locations WHERE id = $1 AND deleted_at IS NULL",
    )
    .bind(location_id)
    .fetch_optional(pool)
    .await
    .map_err(|e| AppError::Internal(anyhow::anyhow!("DB: {e}")))?;

    tenant_id.ok_or(AppError::NotFound { entity: "Location" })
}
