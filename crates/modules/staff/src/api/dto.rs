use chrono::{DateTime, NaiveDate, NaiveTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::application::CommissionSummary;
use crate::domain::{CommissionEntry, CommissionRule, Shift, StaffProfile};

// --- Staff profile DTOs ---

#[derive(Debug, Deserialize)]
pub struct CreateStaffRequest {
    pub user_id: Uuid,
    pub display_name: String,
    pub skill_level: String,
    pub hourly_rate: i64,
}

#[derive(Debug, Deserialize)]
pub struct UpdateStaffRequest {
    pub display_name: Option<String>,
    pub skill_level: Option<String>,
    pub hourly_rate: Option<i64>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct StaffResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub location_id: Uuid,
    pub display_name: String,
    pub skill_level: String,
    pub hourly_rate: i64,
    pub is_active: bool,
    pub joined_date: NaiveDate,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<StaffProfile> for StaffResponse {
    fn from(p: StaffProfile) -> Self {
        Self {
            id: p.id,
            user_id: p.user_id,
            location_id: p.location_id,
            display_name: p.display_name,
            skill_level: p.skill_level.to_string(),
            hourly_rate: p.hourly_rate,
            is_active: p.is_active,
            joined_date: p.joined_date,
            notes: p.notes,
            created_at: p.created_at,
            updated_at: p.updated_at,
        }
    }
}

// --- Shift DTOs ---

#[derive(Debug, Deserialize)]
pub struct CreateShiftRequest {
    pub staff_id: Uuid,
    pub shift_date: NaiveDate,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
}

#[derive(Debug, Deserialize)]
pub struct UpdateShiftStatusRequest {
    pub status: String,
    pub actual_start: Option<DateTime<Utc>>,
    pub actual_end: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct DateRangeQuery {
    pub date: NaiveDate,
}

#[derive(Debug, Serialize)]
pub struct ShiftResponse {
    pub id: Uuid,
    pub location_id: Uuid,
    pub staff_id: Uuid,
    pub shift_date: NaiveDate,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub actual_start: Option<DateTime<Utc>>,
    pub actual_end: Option<DateTime<Utc>>,
    pub status: String,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl From<Shift> for ShiftResponse {
    fn from(s: Shift) -> Self {
        Self {
            id: s.id,
            location_id: s.location_id,
            staff_id: s.staff_id,
            shift_date: s.shift_date,
            start_time: s.start_time,
            end_time: s.end_time,
            actual_start: s.actual_start,
            actual_end: s.actual_end,
            status: s.status.to_string(),
            notes: s.notes,
            created_at: s.created_at,
        }
    }
}

// --- Commission rule DTOs ---

#[derive(Debug, Deserialize)]
pub struct CreateCommissionRuleRequest {
    pub name: String,
    pub service_id: Option<Uuid>,
    pub skill_level: Option<String>,
    pub role_in_job: String,
    pub commission_type: String,
    pub commission_value: i64,
}

#[derive(Debug, Serialize)]
pub struct CommissionRuleResponse {
    pub id: Uuid,
    pub location_id: Uuid,
    pub name: String,
    pub service_id: Option<Uuid>,
    pub skill_level: Option<String>,
    pub role_in_job: String,
    pub commission_type: String,
    pub commission_value: i64,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<CommissionRule> for CommissionRuleResponse {
    fn from(r: CommissionRule) -> Self {
        Self {
            id: r.id,
            location_id: r.location_id,
            name: r.name,
            service_id: r.service_id,
            skill_level: r.skill_level.map(|sl| sl.to_string()),
            role_in_job: r.role_in_job.to_string(),
            commission_type: r.commission_type.to_string(),
            commission_value: r.commission_value,
            is_active: r.is_active,
            created_at: r.created_at,
            updated_at: r.updated_at,
        }
    }
}

// --- Commission entry DTOs ---

#[derive(Debug, Deserialize)]
pub struct RecordCommissionRequest {
    pub payment_id: Uuid,
    pub staff_id: Uuid,
    pub rule_id: Option<Uuid>,
    pub role_in_job: String,
    pub payment_amount: i64,
    pub commission_amount: i64,
}

#[derive(Debug, Serialize)]
pub struct CommissionEntryResponse {
    pub id: Uuid,
    pub payment_id: Uuid,
    pub staff_id: Uuid,
    pub rule_id: Option<Uuid>,
    pub role_in_job: String,
    pub payment_amount: i64,
    pub commission_amount: i64,
    pub created_at: DateTime<Utc>,
}

impl From<CommissionEntry> for CommissionEntryResponse {
    fn from(e: CommissionEntry) -> Self {
        Self {
            id: e.id,
            payment_id: e.payment_id,
            staff_id: e.staff_id,
            rule_id: e.rule_id,
            role_in_job: e.role_in_job.to_string(),
            payment_amount: e.payment_amount,
            commission_amount: e.commission_amount,
            created_at: e.created_at,
        }
    }
}

// --- Commission summary DTOs ---

#[derive(Debug, Deserialize)]
pub struct CommissionSummaryQuery {
    pub from: NaiveDate,
    pub to: NaiveDate,
}

#[derive(Debug, Serialize)]
pub struct CommissionSummaryResponse {
    pub staff_id: Uuid,
    pub total_jobs: i64,
    pub total_revenue: i64,
    pub total_commission: i64,
    pub period_from: NaiveDate,
    pub period_to: NaiveDate,
}

impl From<CommissionSummary> for CommissionSummaryResponse {
    fn from(s: CommissionSummary) -> Self {
        Self {
            staff_id: s.staff_id,
            total_jobs: s.total_jobs,
            total_revenue: s.total_revenue,
            total_commission: s.total_commission,
            period_from: s.period_from,
            period_to: s.period_to,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct MessageResponse {
    pub message: String,
}
