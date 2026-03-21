use chrono::NaiveDate;
use uuid::Uuid;

use crate::domain::Payment;

use super::services::{CreatePaymentInput, DailyRevenueSummary, StaffEarning};

pub trait PaymentRepository: Send + Sync {
    fn create(
        &self,
        tenant_id: Uuid,
        input: &CreatePaymentInput,
    ) -> impl std::future::Future<Output = anyhow::Result<Payment>> + Send;

    fn get_by_id(
        &self,
        tenant_id: Uuid,
        id: Uuid,
    ) -> impl std::future::Future<Output = anyhow::Result<Option<Payment>>> + Send;

    fn list_by_location(
        &self,
        tenant_id: Uuid,
        location_id: Uuid,
        date: NaiveDate,
    ) -> impl std::future::Future<Output = anyhow::Result<Vec<Payment>>> + Send;

    fn mark_completed(
        &self,
        tenant_id: Uuid,
        id: Uuid,
        verified_by: Option<Uuid>,
    ) -> impl std::future::Future<Output = anyhow::Result<Payment>> + Send;

    fn daily_revenue(
        &self,
        tenant_id: Uuid,
        location_id: Uuid,
        date: NaiveDate,
    ) -> impl std::future::Future<Output = anyhow::Result<DailyRevenueSummary>> + Send;

    fn staff_earnings(
        &self,
        tenant_id: Uuid,
        staff_id: Uuid,
        from: NaiveDate,
        to: NaiveDate,
    ) -> impl std::future::Future<Output = anyhow::Result<Vec<StaffEarning>>> + Send;
}
