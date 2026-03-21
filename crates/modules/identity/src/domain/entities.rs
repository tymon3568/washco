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
    pub attempts: u8,
}

/// Maximum number of failed OTP verification attempts before lockout.
pub const OTP_MAX_ATTEMPTS: u8 = 5;

impl OtpEntry {
    pub fn new(phone: String) -> Self {
        use rand::Rng;
        let code = format!("{:06}", rand::rng().random_range(0..1_000_000u32));
        Self {
            phone,
            code,
            expires_at: Utc::now() + chrono::Duration::minutes(5),
            attempts: 0,
        }
    }

    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }

    pub fn matches(&self, code: &str) -> bool {
        self.code == code
    }

    pub fn max_attempts_exceeded(&self) -> bool {
        self.attempts >= OTP_MAX_ATTEMPTS
    }

    pub fn increment_attempts(&mut self) {
        self.attempts += 1;
    }
}

/// Validates a Vietnamese phone number format (10 digits starting with 0).
pub fn validate_phone(phone: &str) -> bool {
    phone.len() >= 10
        && phone.len() <= 11
        && phone.starts_with('0')
        && phone[1..].chars().all(|c| c.is_ascii_digit())
}

/// Strips HTML tags from input to prevent stored XSS.
pub fn sanitize_text(input: &str) -> String {
    input.replace('<', "&lt;").replace('>', "&gt;")
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
            attempts: 0,
        };
        assert!(otp.is_expired());
    }

    #[test]
    fn otp_max_attempts_exceeded() {
        let mut otp = OtpEntry::new("0901234567".into());
        assert!(!otp.max_attempts_exceeded());
        for _ in 0..OTP_MAX_ATTEMPTS {
            otp.increment_attempts();
        }
        assert!(otp.max_attempts_exceeded());
    }

    #[test]
    fn validate_phone_format() {
        assert!(validate_phone("0901234567"));
        assert!(validate_phone("09012345678")); // 11 digits ok
        assert!(!validate_phone("")); // empty
        assert!(!validate_phone("abc")); // non-numeric
        assert!(!validate_phone("1234567890")); // doesn't start with 0
        assert!(!validate_phone("090123456")); // too short (9 digits)
    }

    #[test]
    fn sanitize_text_strips_html() {
        assert_eq!(
            sanitize_text("<script>alert(1)</script>"),
            "&lt;script&gt;alert(1)&lt;/script&gt;"
        );
        assert_eq!(sanitize_text("Normal text"), "Normal text");
    }
}
