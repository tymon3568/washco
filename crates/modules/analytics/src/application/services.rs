use chrono::NaiveDate;
use uuid::Uuid;
use washco_shared::AppError;

use crate::domain::{
    BayUtilization, DailySummary, LocationComparison, PeriodSummary, ServiceMetric, TrendDataPoint,
};

use super::ports::AnalyticsRepository;

pub struct AnalyticsService<R> {
    repo: R,
}

impl<R: AnalyticsRepository> AnalyticsService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn daily_summary(
        &self,
        tenant_id: Uuid,
        location_id: Uuid,
        date: NaiveDate,
    ) -> Result<DailySummary, AppError> {
        self.repo
            .daily_summary(tenant_id, location_id, date)
            .await
            .map_err(AppError::Internal)
    }

    pub async fn bay_utilization(
        &self,
        tenant_id: Uuid,
        location_id: Uuid,
        date: NaiveDate,
        bay_count: i32,
        operating_hours_minutes: i32,
    ) -> Result<BayUtilization, AppError> {
        self.repo
            .bay_utilization(
                tenant_id,
                location_id,
                date,
                bay_count,
                operating_hours_minutes,
            )
            .await
            .map_err(AppError::Internal)
    }

    pub async fn service_breakdown(
        &self,
        tenant_id: Uuid,
        location_id: Uuid,
        date: NaiveDate,
    ) -> Result<Vec<ServiceMetric>, AppError> {
        self.repo
            .service_breakdown(tenant_id, location_id, date)
            .await
            .map_err(AppError::Internal)
    }

    pub async fn trend(
        &self,
        tenant_id: Uuid,
        location_id: Uuid,
        from: NaiveDate,
        to: NaiveDate,
    ) -> Result<Vec<TrendDataPoint>, AppError> {
        self.repo
            .trend(tenant_id, location_id, from, to)
            .await
            .map_err(AppError::Internal)
    }

    pub async fn period_summary(
        &self,
        tenant_id: Uuid,
        location_id: Uuid,
        from: NaiveDate,
        to: NaiveDate,
    ) -> Result<PeriodSummary, AppError> {
        self.repo
            .period_summary(tenant_id, location_id, from, to)
            .await
            .map_err(AppError::Internal)
    }

    pub async fn compare_locations(
        &self,
        tenant_id: Uuid,
        location_ids: &[Uuid],
        from: NaiveDate,
        to: NaiveDate,
    ) -> Result<Vec<LocationComparison>, AppError> {
        self.repo
            .compare_locations(tenant_id, location_ids, from, to)
            .await
            .map_err(AppError::Internal)
    }
}
