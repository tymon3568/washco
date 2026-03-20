use uuid::Uuid;

use crate::domain::Promotion;

pub trait PromotionRepository: Send + Sync {
    fn create(
        &self,
        promo: &Promotion,
    ) -> impl std::future::Future<Output = anyhow::Result<()>> + Send;

    fn find_by_id(
        &self,
        tenant_id: Uuid,
        id: Uuid,
    ) -> impl std::future::Future<Output = anyhow::Result<Option<Promotion>>> + Send;

    fn find_by_code(
        &self,
        tenant_id: Uuid,
        code: &str,
    ) -> impl std::future::Future<Output = anyhow::Result<Option<Promotion>>> + Send;

    fn update(
        &self,
        promo: &Promotion,
    ) -> impl std::future::Future<Output = anyhow::Result<()>> + Send;

    fn delete(
        &self,
        tenant_id: Uuid,
        id: Uuid,
    ) -> impl std::future::Future<Output = anyhow::Result<()>> + Send;

    fn list(
        &self,
        tenant_id: Uuid,
    ) -> impl std::future::Future<Output = anyhow::Result<Vec<Promotion>>> + Send;

    fn increment_used_count(
        &self,
        tenant_id: Uuid,
        id: Uuid,
    ) -> impl std::future::Future<Output = anyhow::Result<()>> + Send;
}
