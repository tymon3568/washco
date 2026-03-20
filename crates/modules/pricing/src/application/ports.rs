use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::domain::PricingRule;

pub trait PricingRepository: Send + Sync {
    fn create(
        &self,
        rule: &PricingRule,
    ) -> impl std::future::Future<Output = anyhow::Result<()>> + Send;

    fn find_by_id(
        &self,
        tenant_id: Uuid,
        id: Uuid,
    ) -> impl std::future::Future<Output = anyhow::Result<Option<PricingRule>>> + Send;

    fn update(
        &self,
        rule: &PricingRule,
    ) -> impl std::future::Future<Output = anyhow::Result<()>> + Send;

    fn delete(
        &self,
        tenant_id: Uuid,
        id: Uuid,
    ) -> impl std::future::Future<Output = anyhow::Result<()>> + Send;

    fn list_by_location(
        &self,
        tenant_id: Uuid,
        location_id: Uuid,
    ) -> impl std::future::Future<Output = anyhow::Result<Vec<PricingRule>>> + Send;

    fn find_active_rules(
        &self,
        tenant_id: Uuid,
        location_id: Uuid,
        service_id: Option<Uuid>,
        now: DateTime<Utc>,
    ) -> impl std::future::Future<Output = anyhow::Result<Vec<PricingRule>>> + Send;
}
