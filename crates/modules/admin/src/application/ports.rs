use uuid::Uuid;

use crate::domain::{AdminAction, AdminLocationView, PlatformMetrics, SubscriptionTier};

pub trait AdminRepository: Send + Sync {
    fn list_locations(
        &self,
        status_filter: Option<&str>,
        limit: i64,
        offset: i64,
    ) -> impl std::future::Future<Output = anyhow::Result<Vec<AdminLocationView>>> + Send;

    fn get_location(
        &self,
        id: Uuid,
    ) -> impl std::future::Future<Output = anyhow::Result<Option<AdminLocationView>>> + Send;

    fn update_location_status(
        &self,
        id: Uuid,
        status: &str,
    ) -> impl std::future::Future<Output = anyhow::Result<()>> + Send;

    fn platform_metrics(
        &self,
    ) -> impl std::future::Future<Output = anyhow::Result<PlatformMetrics>> + Send;

    fn log_action(
        &self,
        action: &AdminAction,
    ) -> impl std::future::Future<Output = anyhow::Result<()>> + Send;

    fn list_actions(
        &self,
        limit: i64,
        offset: i64,
    ) -> impl std::future::Future<Output = anyhow::Result<Vec<AdminAction>>> + Send;

    fn list_tiers(
        &self,
    ) -> impl std::future::Future<Output = anyhow::Result<Vec<SubscriptionTier>>> + Send;
}
