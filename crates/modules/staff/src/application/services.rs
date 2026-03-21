use chrono::{DateTime, NaiveDate, NaiveTime, Utc};
use uuid::Uuid;
use washco_shared::AppError;

use super::ports::StaffRepository;
use crate::domain::{CommissionEntry, CommissionRule, Shift, SkillLevel, StaffError, StaffProfile};

pub struct CreateStaffInput {
    pub user_id: Uuid,
    pub location_id: Uuid,
    pub display_name: String,
    pub skill_level: String,
    pub hourly_rate: i64,
}

pub struct UpdateStaffInput {
    pub display_name: Option<String>,
    pub skill_level: Option<String>,
    pub hourly_rate: Option<i64>,
    pub is_active: Option<bool>,
}

pub struct CreateShiftInput {
    pub location_id: Uuid,
    pub staff_id: Uuid,
    pub shift_date: NaiveDate,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
}

pub struct CreateCommissionRuleInput {
    pub location_id: Uuid,
    pub name: String,
    pub service_id: Option<Uuid>,
    pub skill_level: Option<String>,
    pub role_in_job: String,
    pub commission_type: String,
    pub commission_value: i64,
}

pub struct RecordCommissionInput {
    pub payment_id: Uuid,
    pub staff_id: Uuid,
    pub rule_id: Option<Uuid>,
    pub role_in_job: String,
    pub payment_amount: i64,
    pub commission_amount: i64,
}

#[derive(Debug, Clone)]
pub struct CommissionSummary {
    pub staff_id: Uuid,
    pub total_jobs: i64,
    pub total_revenue: i64,
    pub total_commission: i64,
    pub period_from: NaiveDate,
    pub period_to: NaiveDate,
}

pub struct StaffService<R> {
    repo: R,
}

impl<R: StaffRepository> StaffService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    // --- Staff profiles ---

    pub async fn create_profile(
        &self,
        tenant_id: Uuid,
        input: CreateStaffInput,
    ) -> Result<StaffProfile, AppError> {
        // Validate skill level
        SkillLevel::from_str(&input.skill_level)
            .ok_or_else(|| StaffError::InvalidSkillLevel(input.skill_level.clone()))?;

        self.repo
            .create_profile(tenant_id, &input)
            .await
            .map_err(AppError::Internal)
    }

    pub async fn get_profile(&self, tenant_id: Uuid, id: Uuid) -> Result<StaffProfile, AppError> {
        self.repo
            .get_profile(tenant_id, id)
            .await
            .map_err(AppError::Internal)?
            .ok_or_else(|| StaffError::NotFound.into())
    }

    pub async fn list_by_location(
        &self,
        tenant_id: Uuid,
        location_id: Uuid,
    ) -> Result<Vec<StaffProfile>, AppError> {
        self.repo
            .list_by_location(tenant_id, location_id)
            .await
            .map_err(AppError::Internal)
    }

    pub async fn update_profile(
        &self,
        tenant_id: Uuid,
        id: Uuid,
        input: UpdateStaffInput,
    ) -> Result<StaffProfile, AppError> {
        // Validate skill level if provided
        if let Some(ref sl) = input.skill_level {
            SkillLevel::from_str(sl).ok_or_else(|| StaffError::InvalidSkillLevel(sl.clone()))?;
        }

        self.repo
            .update_profile(tenant_id, id, &input)
            .await
            .map_err(AppError::Internal)
    }

    // --- Shifts ---

    pub async fn create_shift(
        &self,
        tenant_id: Uuid,
        input: CreateShiftInput,
    ) -> Result<Shift, AppError> {
        self.repo
            .create_shift(tenant_id, &input)
            .await
            .map_err(AppError::Internal)
    }

    pub async fn list_shifts(
        &self,
        tenant_id: Uuid,
        location_id: Uuid,
        date: NaiveDate,
    ) -> Result<Vec<Shift>, AppError> {
        self.repo
            .list_shifts(tenant_id, location_id, date)
            .await
            .map_err(AppError::Internal)
    }

    pub async fn update_shift_status(
        &self,
        tenant_id: Uuid,
        id: Uuid,
        status: String,
        actual_start: Option<DateTime<Utc>>,
        actual_end: Option<DateTime<Utc>>,
    ) -> Result<Shift, AppError> {
        self.repo
            .update_shift_status(tenant_id, id, &status, actual_start, actual_end)
            .await
            .map_err(AppError::Internal)
    }

    // --- Commission rules ---

    pub async fn create_commission_rule(
        &self,
        tenant_id: Uuid,
        input: CreateCommissionRuleInput,
    ) -> Result<CommissionRule, AppError> {
        self.repo
            .create_commission_rule(tenant_id, &input)
            .await
            .map_err(AppError::Internal)
    }

    pub async fn list_commission_rules(
        &self,
        tenant_id: Uuid,
        location_id: Uuid,
    ) -> Result<Vec<CommissionRule>, AppError> {
        self.repo
            .list_commission_rules(tenant_id, location_id)
            .await
            .map_err(AppError::Internal)
    }

    // --- Commission entries ---

    pub async fn record_commission(
        &self,
        tenant_id: Uuid,
        input: RecordCommissionInput,
    ) -> Result<CommissionEntry, AppError> {
        self.repo
            .record_commission(tenant_id, &input)
            .await
            .map_err(AppError::Internal)
    }

    pub async fn staff_commission_summary(
        &self,
        tenant_id: Uuid,
        staff_id: Uuid,
        from: NaiveDate,
        to: NaiveDate,
    ) -> Result<CommissionSummary, AppError> {
        self.repo
            .staff_commission_summary(tenant_id, staff_id, from, to)
            .await
            .map_err(AppError::Internal)
    }
}
