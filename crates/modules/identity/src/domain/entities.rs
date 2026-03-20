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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn user_new_owner_creates_unverified_user() {
        let tenant_id = Uuid::now_v7();
        let user = User::new_owner(tenant_id, "0901234567".into(), "Test".into());
        assert_eq!(user.tenant_id, tenant_id);
        assert_eq!(user.role, Role::Owner);
        assert!(!user.is_verified);
        assert!(user.deleted_at.is_none());
    }

    #[test]
    fn user_verify_sets_verified_and_updates_timestamp() {
        let mut user = User::new_owner(Uuid::now_v7(), "0901234567".into(), "Test".into());
        let before = user.updated_at;
        user.verify();
        assert!(user.is_verified);
        assert!(user.updated_at >= before);
    }

    #[test]
    fn tenant_new_generates_uuid_v7() {
        let t = Tenant::new("Wash Co".into(), "Owner".into());
        assert_eq!(t.business_name, "Wash Co");
        assert_eq!(t.owner_name, "Owner");
    }

    #[test]
    fn otp_code_is_6_digits() {
        let otp = OtpEntry::new("0901234567".into());
        assert_eq!(otp.code.len(), 6);
        assert!(otp.code.chars().all(|c| c.is_ascii_digit()));
    }

    #[test]
    fn otp_matches_correct_code() {
        let otp = OtpEntry::new("0901234567".into());
        let code = otp.code.clone();
        assert!(otp.matches(&code));
        assert!(!otp.matches("000000"));
    }

    #[test]
    fn otp_not_expired_when_fresh() {
        let otp = OtpEntry::new("0901234567".into());
        assert!(!otp.is_expired());
    }

    #[test]
    fn otp_expired_when_past() {
        let otp = OtpEntry {
            phone: "0901234567".into(),
            code: "123456".into(),
            expires_at: Utc::now() - chrono::Duration::minutes(1),
        };
        assert!(otp.is_expired());
    }
}
