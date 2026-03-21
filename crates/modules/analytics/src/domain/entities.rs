use chrono::NaiveDate;
use uuid::Uuid;
use washco_shared::money::Money;

#[derive(Debug, Clone)]
pub struct DailySummary {
    pub location_id: Uuid,
    pub date: NaiveDate,
    pub total_revenue: Money,
    pub completed_jobs: i32,
    pub walk_ins: i32,
    pub average_wait_minutes: f64,
    pub cancellations: i32,
}

#[derive(Debug, Clone)]
pub struct BayUtilization {
    pub location_id: Uuid,
    pub date: NaiveDate,
    pub utilization_percent: f64,
    pub total_service_minutes: f64,
}

#[derive(Debug, Clone)]
pub struct ServiceMetric {
    pub service_name: String,
    pub count: i64,
    pub revenue: Money,
    pub average_duration_minutes: f64,
}

#[derive(Debug, Clone)]
pub struct TrendDataPoint {
    pub date: NaiveDate,
    pub revenue: Money,
    pub completed_jobs: i32,
    pub walk_ins: i32,
    pub cancellations: i32,
    pub average_wait_minutes: f64,
}

#[derive(Debug, Clone)]
pub struct PeriodSummary {
    pub location_id: Uuid,
    pub from: NaiveDate,
    pub to: NaiveDate,
    pub total_revenue: Money,
    pub total_completed: i32,
    pub total_walk_ins: i32,
    pub total_cancellations: i32,
    pub average_wait_minutes: f64,
    pub busiest_day: Option<NaiveDate>,
    pub peak_revenue: Money,
}

#[derive(Debug, Clone)]
pub struct LocationComparison {
    pub location_id: Uuid,
    pub location_name: String,
    pub total_revenue: Money,
    pub total_completed: i32,
    pub average_wait_minutes: f64,
}
