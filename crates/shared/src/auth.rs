use axum::{extract::FromRequestParts, http::request::Parts};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::AppError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub tenant_id: Uuid,
    pub role: Role,
    pub exp: i64,
    pub iat: i64,
    #[serde(default)]
    pub tier: Option<String>,
    #[serde(default)]
    pub features: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, sqlx::Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "text", rename_all = "snake_case")]
pub enum Role {
    Owner,
    Manager,
    Cashier,
    Staff,
    Driver,
    Admin,
}

impl std::fmt::Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Role::Owner => write!(f, "owner"),
            Role::Manager => write!(f, "manager"),
            Role::Cashier => write!(f, "cashier"),
            Role::Staff => write!(f, "staff"),
            Role::Driver => write!(f, "driver"),
            Role::Admin => write!(f, "admin"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TenantContext {
    pub tenant_id: Uuid,
    pub user_id: Uuid,
    pub role: Role,
    pub tier: Option<String>,
    pub features: Vec<String>,
}

#[derive(Clone)]
pub struct JwtConfig {
    pub encoding_key: EncodingKey,
    pub decoding_key: DecodingKey,
    pub expiry_seconds: i64,
    pub refresh_expiry_seconds: i64,
}

impl JwtConfig {
    pub fn new(secret: &str, expiry_seconds: i64, refresh_expiry_seconds: i64) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(secret.as_bytes()),
            decoding_key: DecodingKey::from_secret(secret.as_bytes()),
            expiry_seconds,
            refresh_expiry_seconds,
        }
    }

    pub fn generate_access_token(
        &self,
        user_id: Uuid,
        tenant_id: Uuid,
        role: Role,
    ) -> Result<String, AppError> {
        self.generate_access_token_with_tier(user_id, tenant_id, role, None, vec![])
    }

    pub fn generate_access_token_with_tier(
        &self,
        user_id: Uuid,
        tenant_id: Uuid,
        role: Role,
        tier: Option<String>,
        features: Vec<String>,
    ) -> Result<String, AppError> {
        let now = chrono::Utc::now().timestamp();
        let claims = Claims {
            sub: user_id,
            tenant_id,
            role,
            iat: now,
            exp: now + self.expiry_seconds,
            tier,
            features,
        };
        jsonwebtoken::encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(|e| AppError::Internal(anyhow::anyhow!("JWT encode error: {e}")))
    }

    pub fn generate_refresh_token(
        &self,
        user_id: Uuid,
        tenant_id: Uuid,
        role: Role,
    ) -> Result<String, AppError> {
        let now = chrono::Utc::now().timestamp();
        let claims = Claims {
            sub: user_id,
            tenant_id,
            role,
            iat: now,
            exp: now + self.refresh_expiry_seconds,
            tier: None,
            features: vec![],
        };
        jsonwebtoken::encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(|e| AppError::Internal(anyhow::anyhow!("JWT encode error: {e}")))
    }

    pub fn verify_token(&self, token: &str) -> Result<Claims, AppError> {
        jsonwebtoken::decode::<Claims>(token, &self.decoding_key, &Validation::default())
            .map(|data| data.claims)
            .map_err(|_| AppError::Unauthorized)
    }
}

impl std::fmt::Debug for JwtConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JwtConfig")
            .field("expiry_seconds", &self.expiry_seconds)
            .finish()
    }
}

impl TenantContext {
    /// Check if the current user has one of the allowed roles
    pub fn require_role(&self, allowed: &[Role]) -> Result<(), AppError> {
        if allowed.contains(&self.role) {
            Ok(())
        } else {
            Err(AppError::Forbidden)
        }
    }

    /// Check if user is owner or admin
    pub fn require_owner_or_admin(&self) -> Result<(), AppError> {
        self.require_role(&[Role::Owner, Role::Admin])
    }

    /// Check if user is at least manager level
    pub fn require_manager_or_above(&self) -> Result<(), AppError> {
        self.require_role(&[Role::Owner, Role::Admin, Role::Manager])
    }

    /// Check if the tenant's tier includes a specific feature
    pub fn require_feature(&self, feature: &str) -> Result<(), AppError> {
        // Admin bypasses feature gates
        if self.role == Role::Admin {
            return Ok(());
        }
        // If no features are set (legacy token), allow all
        if self.features.is_empty() {
            return Ok(());
        }
        if self.features.iter().any(|f| f == feature) {
            Ok(())
        } else {
            Err(AppError::Forbidden)
        }
    }
}

/// Axum extractor that validates JWT and provides TenantContext
impl<S> FromRequestParts<S> for TenantContext
where
    S: Send + Sync + AsRef<JwtConfig>,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get("authorization")
            .and_then(|v| v.to_str().ok())
            .ok_or(AppError::Unauthorized)?;

        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or(AppError::Unauthorized)?;

        let jwt_config: &JwtConfig = state.as_ref();
        let claims = jwt_config.verify_token(token)?;

        Ok(TenantContext {
            tenant_id: claims.tenant_id,
            user_id: claims.sub,
            role: claims.role,
            tier: claims.tier,
            features: claims.features,
        })
    }
}
