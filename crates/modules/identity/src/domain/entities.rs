use chrono::{DateTime, Utc};
use uuid::Uuid;
use washco_shared::Role;

#[derive(Debug, Clone)]
pub struct User {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub phone: String,
    pub name: String,
    pub role: Role,
    pub is_verified: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl User {
    pub fn new_owner(tenant_id: Uuid, phone: String, name: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::now_v7(),
            tenant_id,
            phone,
            name,
            role: Role::Owner,
            is_verified: false,
            created_at: now,
            updated_at: now,
            deleted_at: None,
        }
    }

    pub fn verify(&mut self) {
        self.is_verified = true;
        self.updated_at = Utc::now();
    }
}

#[derive(Debug, Clone)]
pub struct Tenant {
    pub id: Uuid,
    pub business_name: String,
    pub owner_name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Tenant {
    pub fn new(business_name: String, owner_name: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::now_v7(),
            business_name,
            owner_name,
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Debug, Clone)]
pub struct OtpEntry {
    pub phone: String,
    pub code: String,
    pub expires_at: DateTime<Utc>,
}

impl OtpEntry {
    pub fn new(phone: String) -> Self {
        use rand::Rng;
        let code = format!("{:06}", rand::rng().random_range(0..1_000_000u32));
        Self {
            phone,
            code,
            expires_at: Utc::now() + chrono::Duration::minutes(5),
        }
    }

    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }

    pub fn matches(&self, code: &str) -> bool {
        self.code == code
    }
}
