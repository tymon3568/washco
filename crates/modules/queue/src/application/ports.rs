use uuid::Uuid;

use crate::domain::{QueueEntry, WaitEstimate};

pub trait QueueRepository: Send + Sync {
    fn find_active_by_location(
        &self,
        tenant_id: Uuid,
        location_id: Uuid,
    ) -> impl std::future::Future<Output = anyhow::Result<Vec<QueueEntry>>> + Send;

    fn find_by_id(
        &self,
        tenant_id: Uuid,
        id: Uuid,
    ) -> impl std::future::Future<Output = anyhow::Result<Option<QueueEntry>>> + Send;

    fn create(
        &self,
        entry: &QueueEntry,
    ) -> impl std::future::Future<Output = anyhow::Result<()>> + Send;

    fn update_status(
        &self,
        entry: &QueueEntry,
    ) -> impl std::future::Future<Output = anyhow::Result<()>> + Send;

    fn next_queue_number(
        &self,
        tenant_id: Uuid,
        location_id: Uuid,
    ) -> impl std::future::Future<Output = anyhow::Result<i32>> + Send;

    fn estimate_wait(
        &self,
        tenant_id: Uuid,
        location_id: Uuid,
        entry_joined_at: chrono::DateTime<chrono::Utc>,
    ) -> impl std::future::Future<Output = anyhow::Result<WaitEstimate>> + Send;

    fn completed_today_count(
        &self,
        tenant_id: Uuid,
        location_id: Uuid,
    ) -> impl std::future::Future<Output = anyhow::Result<i64>> + Send;
}
