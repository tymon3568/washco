use chrono::NaiveDate;
use uuid::Uuid;

use crate::domain::{BayUtilization, DailySummary, ServiceMetric};

pub trait AnalyticsRepository: Send + Sync {
    fn daily_summary(
        &self,
        tenant_id: Uuid,
        location_id: Uuid,
        date: NaiveDate,
    ) -> impl std::future::Future<Output = anyhow::Result<DailySummary>> + Send;

    fn bay_utilization(
        &self,
        tenant_id: Uuid,
        location_id: Uuid,
        date: NaiveDate,
        bay_count: i32,
        operating_hours_minutes: i32,
    ) -> impl std::future::Future<Output = anyhow::Result<BayUtilization>> + Send;

    fn service_breakdown(
        &self,
        tenant_id: Uuid,
        location_id: Uuid,
        date: NaiveDate,
    ) -> impl std::future::Future<Output = anyhow::Result<Vec<ServiceMetric>>> + Send;
}
