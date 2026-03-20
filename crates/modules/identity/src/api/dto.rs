use serde::{Deserialize, Serialize};
use uuid::Uuid;
use washco_shared::Role;

use crate::domain::User;

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub phone: String,
    pub business_name: String,
    pub owner_name: String,
}

#[derive(Debug, Deserialize)]
pub struct OtpRequest {
    pub phone: String,
}

#[derive(Debug, Deserialize)]
pub struct OtpVerifyRequest {
    pub phone: String,
    pub code: String,
}

#[derive(Debug, Deserialize)]
pub struct RefreshRequest {
    pub refresh_token: String,
}

#[derive(Debug, Serialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub phone: String,
    pub name: String,
    pub role: Role,
    pub is_verified: bool,
}

impl From<User> for UserResponse {
    fn from(u: User) -> Self {
        Self {
            id: u.id,
            tenant_id: u.tenant_id,
            phone: u.phone,
            name: u.name,
            role: u.role,
            is_verified: u.is_verified,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct MessageResponse {
    pub message: String,
}
