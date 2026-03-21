use chrono::{DateTime, NaiveDate, Utc};
use uuid::Uuid;

use super::services::{
    CommissionSummary, CreateCommissionRuleInput, CreateShiftInput, CreateStaffInput,
    RecordCommissionInput, UpdateStaffInput,
};
use crate::domain::{CommissionEntry, CommissionRule, Shift, StaffProfile};

pub trait StaffRepository: Send + Sync {
    // Staff profiles
    fn create_profile(
        &self,
        tenant_id: Uuid,
        input: &CreateStaffInput,
    ) -> impl std::future::Future<Output = anyhow::Result<StaffProfile>> + Send;

    fn get_profile(
        &self,
        tenant_id: Uuid,
        id: Uuid,
    ) -> impl std::future::Future<Output = anyhow::Result<Option<StaffProfile>>> + Send;

    fn list_by_location(
        &self,
        tenant_id: Uuid,
        location_id: Uuid,
    ) -> impl std::future::Future<Output = anyhow::Result<Vec<StaffProfile>>> + Send;

    fn update_profile(
        &self,
        tenant_id: Uuid,
        id: Uuid,
        input: &UpdateStaffInput,
    ) -> impl std::future::Future<Output = anyhow::Result<StaffProfile>> + Send;

    // Shifts
    fn create_shift(
        &self,
        tenant_id: Uuid,
        input: &CreateShiftInput,
    ) -> impl std::future::Future<Output = anyhow::Result<Shift>> + Send;

    fn list_shifts(
        &self,
        tenant_id: Uuid,
        location_id: Uuid,
        date: NaiveDate,
    ) -> impl std::future::Future<Output = anyhow::Result<Vec<Shift>>> + Send;

    fn update_shift_status(
        &self,
        tenant_id: Uuid,
        id: Uuid,
        status: &str,
        actual_start: Option<DateTime<Utc>>,
        actual_end: Option<DateTime<Utc>>,
    ) -> impl std::future::Future<Output = anyhow::Result<Shift>> + Send;

    // Commission rules
    fn create_commission_rule(
        &self,
        tenant_id: Uuid,
        input: &CreateCommissionRuleInput,
    ) -> impl std::future::Future<Output = anyhow::Result<CommissionRule>> + Send;

    fn list_commission_rules(
        &self,
        tenant_id: Uuid,
        location_id: Uuid,
    ) -> impl std::future::Future<Output = anyhow::Result<Vec<CommissionRule>>> + Send;

    // Commission entries
    fn record_commission(
        &self,
        tenant_id: Uuid,
        input: &RecordCommissionInput,
    ) -> impl std::future::Future<Output = anyhow::Result<CommissionEntry>> + Send;

    fn staff_commission_summary(
        &self,
        tenant_id: Uuid,
        staff_id: Uuid,
        from: NaiveDate,
        to: NaiveDate,
    ) -> impl std::future::Future<Output = anyhow::Result<CommissionSummary>> + Send;
}
