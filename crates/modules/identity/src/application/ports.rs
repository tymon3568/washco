use uuid::Uuid;

use crate::domain::{OtpEntry, Tenant, User};

pub trait UserRepository: Send + Sync {
    fn find_by_id(
        &self,
        user_id: Uuid,
    ) -> impl std::future::Future<Output = Result<Option<User>, sqlx::Error>> + Send;

    fn find_by_phone(
        &self,
        phone: &str,
    ) -> impl std::future::Future<Output = Result<Option<User>, sqlx::Error>> + Send;

    fn create_tenant(
        &self,
        tenant: &Tenant,
    ) -> impl std::future::Future<Output = Result<(), sqlx::Error>> + Send;

    fn create_user(
        &self,
        user: &User,
    ) -> impl std::future::Future<Output = Result<(), sqlx::Error>> + Send;

    fn update_user(
        &self,
        user: &User,
    ) -> impl std::future::Future<Output = Result<(), sqlx::Error>> + Send;

    fn find_tenant_tier_features(
        &self,
        tenant_id: Uuid,
    ) -> impl std::future::Future<Output = Result<(Option<String>, Vec<String>), sqlx::Error>> + Send;
}

pub trait OtpStore: Send + Sync {
    fn store(
        &self,
        entry: OtpEntry,
    ) -> impl std::future::Future<Output = anyhow::Result<()>> + Send;

    fn get(
        &self,
        phone: &str,
    ) -> impl std::future::Future<Output = anyhow::Result<Option<OtpEntry>>> + Send;

    fn remove(&self, phone: &str) -> impl std::future::Future<Output = anyhow::Result<()>> + Send;

    /// Increment failed attempt counter and return updated entry.
    fn increment_attempts(
        &self,
        phone: &str,
    ) -> impl std::future::Future<Output = anyhow::Result<()>> + Send;
}
