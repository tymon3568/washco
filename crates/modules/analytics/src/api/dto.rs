use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::{
    BayUtilization, DailySummary, LocationComparison, PeriodSummary, ServiceMetric, TrendDataPoint,
};

#[derive(Debug, Deserialize)]
pub struct DateRangeQuery {
    pub date: Option<NaiveDate>,
}

impl DateRangeQuery {
    pub fn date_or_today(&self) -> NaiveDate {
        self.date.unwrap_or_else(|| chrono::Utc::now().date_naive())
    }
}

#[derive(Debug, Deserialize)]
pub struct TrendQuery {
    pub from: Option<NaiveDate>,
    pub to: Option<NaiveDate>,
}

impl TrendQuery {
    pub fn resolve(&self) -> (NaiveDate, NaiveDate) {
        let to = self.to.unwrap_or_else(|| chrono::Utc::now().date_naive());
        let from = self.from.unwrap_or_else(|| to - chrono::Duration::days(30));
        (from, to)
    }
}

#[derive(Debug, Deserialize)]
pub struct CompareQuery {
    pub location_ids: String,
    pub from: Option<NaiveDate>,
    pub to: Option<NaiveDate>,
}

impl CompareQuery {
    pub fn parse_location_ids(&self) -> Vec<Uuid> {
        self.location_ids
            .split(',')
            .filter_map(|s| s.trim().parse::<Uuid>().ok())
            .collect()
    }

    pub fn resolve_dates(&self) -> (NaiveDate, NaiveDate) {
        let to = self.to.unwrap_or_else(|| chrono::Utc::now().date_naive());
        let from = self.from.unwrap_or_else(|| to - chrono::Duration::days(30));
        (from, to)
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

#[derive(Debug, Serialize)]
pub struct TrendDataPointResponse {
    pub date: String,
    pub revenue: i64,
    pub completed_jobs: i32,
    pub walk_ins: i32,
    pub cancellations: i32,
    pub average_wait_minutes: f64,
}

impl From<TrendDataPoint> for TrendDataPointResponse {
    fn from(t: TrendDataPoint) -> Self {
        Self {
            date: t.date.to_string(),
            revenue: t.revenue.amount(),
            completed_jobs: t.completed_jobs,
            walk_ins: t.walk_ins,
            cancellations: t.cancellations,
            average_wait_minutes: t.average_wait_minutes,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct PeriodSummaryResponse {
    pub location_id: Uuid,
    pub from: String,
    pub to: String,
    pub total_revenue: i64,
    pub total_completed: i32,
    pub total_walk_ins: i32,
    pub total_cancellations: i32,
    pub average_wait_minutes: f64,
    pub busiest_day: Option<String>,
    pub peak_revenue: i64,
}

impl From<PeriodSummary> for PeriodSummaryResponse {
    fn from(p: PeriodSummary) -> Self {
        Self {
            location_id: p.location_id,
            from: p.from.to_string(),
            to: p.to.to_string(),
            total_revenue: p.total_revenue.amount(),
            total_completed: p.total_completed,
            total_walk_ins: p.total_walk_ins,
            total_cancellations: p.total_cancellations,
            average_wait_minutes: p.average_wait_minutes,
            busiest_day: p.busiest_day.map(|d| d.to_string()),
            peak_revenue: p.peak_revenue.amount(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct LocationComparisonResponse {
    pub location_id: Uuid,
    pub location_name: String,
    pub total_revenue: i64,
    pub total_completed: i32,
    pub average_wait_minutes: f64,
}

impl From<LocationComparison> for LocationComparisonResponse {
    fn from(c: LocationComparison) -> Self {
        Self {
            location_id: c.location_id,
            location_name: c.location_name,
            total_revenue: c.total_revenue.amount(),
            total_completed: c.total_completed,
            average_wait_minutes: c.average_wait_minutes,
        }
    }
}
