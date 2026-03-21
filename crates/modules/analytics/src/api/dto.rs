use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::domain::{BayUtilization, DailySummary, ServiceMetric};

#[derive(Debug, Deserialize)]
pub struct DateRangeQuery {
    pub date: Option<NaiveDate>,
}

impl DateRangeQuery {
    pub fn date_or_today(&self) -> NaiveDate {
        self.date.unwrap_or_else(|| chrono::Utc::now().date_naive())
    }
}

#[derive(Debug, Serialize)]
pub struct DailySummaryResponse {
    pub date: String,
    pub total_revenue: i64,
    pub completed_jobs: i32,
    pub walk_ins: i32,
    pub average_wait_minutes: f64,
    pub cancellations: i32,
}

impl From<DailySummary> for DailySummaryResponse {
    fn from(s: DailySummary) -> Self {
        Self {
            date: s.date.to_string(),
            total_revenue: s.total_revenue.amount(),
            completed_jobs: s.completed_jobs,
            walk_ins: s.walk_ins,
            average_wait_minutes: s.average_wait_minutes,
            cancellations: s.cancellations,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct UtilizationResponse {
    pub date: String,
    pub bay_utilization_percent: f64,
    pub total_service_minutes: f64,
}

impl From<BayUtilization> for UtilizationResponse {
    fn from(u: BayUtilization) -> Self {
        Self {
            date: u.date.to_string(),
            bay_utilization_percent: u.utilization_percent,
            total_service_minutes: u.total_service_minutes,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ServiceMetricResponse {
    pub service_name: String,
    pub count: i64,
    pub revenue: i64,
    pub average_duration_minutes: f64,
}

impl From<ServiceMetric> for ServiceMetricResponse {
    fn from(m: ServiceMetric) -> Self {
        Self {
            service_name: m.service_name,
            count: m.count,
            revenue: m.revenue.amount(),
            average_duration_minutes: m.average_duration_minutes,
        }
    }
}
