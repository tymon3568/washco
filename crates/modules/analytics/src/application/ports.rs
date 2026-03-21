use chrono::NaiveDate;
use uuid::Uuid;

use crate::domain::{
    BayUtilization, DailySummary, LocationComparison, PeriodSummary, ServiceMetric, TrendDataPoint,
};

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

    fn trend(
        &self,
        tenant_id: Uuid,
        location_id: Uuid,
        from: NaiveDate,
        to: NaiveDate,
    ) -> impl std::future::Future<Output = anyhow::Result<Vec<TrendDataPoint>>> + Send;

    fn period_summary(
        &self,
        tenant_id: Uuid,
        location_id: Uuid,
        from: NaiveDate,
        to: NaiveDate,
    ) -> impl std::future::Future<Output = anyhow::Result<PeriodSummary>> + Send;

    fn compare_locations(
        &self,
        tenant_id: Uuid,
        location_ids: &[Uuid],
        from: NaiveDate,
        to: NaiveDate,
    ) -> impl std::future::Future<Output = anyhow::Result<Vec<LocationComparison>>> + Send;
}
