use uuid::Uuid;

use crate::domain::{Review, ReviewSummary};

pub trait ReviewRepository: Send + Sync {
    fn create(
        &self,
        review: &Review,
    ) -> impl std::future::Future<Output = anyhow::Result<()>> + Send;

    fn find_by_id(
        &self,
        tenant_id: Uuid,
        id: Uuid,
    ) -> impl std::future::Future<Output = anyhow::Result<Option<Review>>> + Send;

    fn list_by_location(
        &self,
        tenant_id: Uuid,
        location_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> impl std::future::Future<Output = anyhow::Result<Vec<Review>>> + Send;

    fn summary_by_location(
        &self,
        tenant_id: Uuid,
        location_id: Uuid,
    ) -> impl std::future::Future<Output = anyhow::Result<ReviewSummary>> + Send;

    fn set_reply(
        &self,
        tenant_id: Uuid,
        id: Uuid,
        reply: &str,
    ) -> impl std::future::Future<Output = anyhow::Result<()>> + Send;

    fn exists_for_queue_entry(
        &self,
        tenant_id: Uuid,
        queue_entry_id: Uuid,
    ) -> impl std::future::Future<Output = anyhow::Result<bool>> + Send;
}
