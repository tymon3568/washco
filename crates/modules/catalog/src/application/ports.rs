use uuid::Uuid;

use crate::domain::Service;

pub trait ServiceRepository: Send + Sync {
    fn find_by_location(
        &self,
        tenant_id: Uuid,
        location_id: Uuid,
    ) -> impl std::future::Future<Output = anyhow::Result<Vec<Service>>> + Send;

    fn find_by_id(
        &self,
        tenant_id: Uuid,
        id: Uuid,
    ) -> impl std::future::Future<Output = anyhow::Result<Option<Service>>> + Send;

    fn create(
        &self,
        service: &Service,
    ) -> impl std::future::Future<Output = anyhow::Result<()>> + Send;

    fn update(
        &self,
        service: &Service,
    ) -> impl std::future::Future<Output = anyhow::Result<()>> + Send;

    fn soft_delete(
        &self,
        tenant_id: Uuid,
        id: Uuid,
    ) -> impl std::future::Future<Output = anyhow::Result<()>> + Send;
}
