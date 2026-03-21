use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::application::{DailyRevenueSummary, StaffEarning};
use crate::domain::Payment;

#[derive(Debug, Deserialize)]
pub struct CreatePaymentRequest {
    pub location_id: Uuid,
    pub queue_entry_id: Option<Uuid>,
    pub booking_id: Option<Uuid>,
    pub customer_name: String,
    pub customer_phone: Option<String>,
    pub service_id: Uuid,
    pub service_name: String,
    pub base_price: i64,
    pub discount_amount: i64,
    pub final_amount: i64,
    pub promotion_id: Option<Uuid>,
    pub payment_method: String,
    pub staff_id: Option<Uuid>,
    pub assistant_id: Option<Uuid>,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PaymentResponse {
    pub id: Uuid,
    pub location_id: Uuid,
    pub queue_entry_id: Option<Uuid>,
    pub booking_id: Option<Uuid>,
    pub customer_name: String,
    pub customer_phone: Option<String>,
    pub service_id: Uuid,
    pub service_name: String,
    pub base_price: i64,
    pub discount_amount: i64,
    pub final_amount: i64,
    pub promotion_id: Option<Uuid>,
    pub payment_method: String,
    pub payment_status: String,
    pub paid_at: Option<chrono::DateTime<chrono::Utc>>,
    pub collected_by: Uuid,
    pub verified_by: Option<Uuid>,
    pub staff_id: Option<Uuid>,
    pub assistant_id: Option<Uuid>,
    pub notes: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<Payment> for PaymentResponse {
    fn from(p: Payment) -> Self {
        Self {
            id: p.id,
            location_id: p.location_id,
            queue_entry_id: p.queue_entry_id,
            booking_id: p.booking_id,
            customer_name: p.customer_name,
            customer_phone: p.customer_phone,
            service_id: p.service_id,
            service_name: p.service_name,
            base_price: p.base_price,
            discount_amount: p.discount_amount,
            final_amount: p.final_amount,
            promotion_id: p.promotion_id,
            payment_method: p.payment_method.to_string(),
            payment_status: p.payment_status.to_string(),
            paid_at: p.paid_at,
            collected_by: p.collected_by,
            verified_by: p.verified_by,
            staff_id: p.staff_id,
            assistant_id: p.assistant_id,
            notes: p.notes,
            created_at: p.created_at,
            updated_at: p.updated_at,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct DailyRevenueResponse {
    pub total_revenue: i64,
    pub completed_count: i64,
    pub cash_amount: i64,
    pub digital_amount: i64,
    pub avg_per_job: i64,
    pub pending_count: i64,
}

impl From<DailyRevenueSummary> for DailyRevenueResponse {
    fn from(s: DailyRevenueSummary) -> Self {
        Self {
            total_revenue: s.total_revenue,
            completed_count: s.completed_count,
            cash_amount: s.cash_amount,
            digital_amount: s.digital_amount,
            avg_per_job: s.avg_per_job,
            pending_count: s.pending_count,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct StaffEarningResponse {
    pub staff_id: Uuid,
    pub staff_name: String,
    pub job_count: i64,
    pub total_revenue: i64,
    pub total_commission: i64,
}

impl From<StaffEarning> for StaffEarningResponse {
    fn from(e: StaffEarning) -> Self {
        Self {
            staff_id: e.staff_id,
            staff_name: e.staff_name,
            job_count: e.job_count,
            total_revenue: e.total_revenue,
            total_commission: e.total_commission,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct MessageResponse {
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct DateQuery {
    pub date: Option<NaiveDate>,
}

impl DateQuery {
    pub fn date_or_today(&self) -> NaiveDate {
        self.date.unwrap_or_else(|| chrono::Utc::now().date_naive())
    }
}

#[derive(Debug, Deserialize)]
pub struct DateRangeQuery {
    pub from: Option<NaiveDate>,
    pub to: Option<NaiveDate>,
}

impl DateRangeQuery {
    pub fn from_or_today(&self) -> NaiveDate {
        self.from.unwrap_or_else(|| chrono::Utc::now().date_naive())
    }

    pub fn to_or_today(&self) -> NaiveDate {
        self.to.unwrap_or_else(|| chrono::Utc::now().date_naive())
    }
}
