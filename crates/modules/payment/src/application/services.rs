use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use washco_shared::AppError;

use crate::domain::{Payment, PaymentError};

use super::ports::PaymentRepository;

/// Input data for creating a payment.
#[derive(Debug, Clone)]
pub struct CreatePaymentInput {
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
    pub collected_by: Uuid,
    pub staff_id: Option<Uuid>,
    pub assistant_id: Option<Uuid>,
    pub notes: Option<String>,
}

/// Daily revenue summary for a location.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyRevenueSummary {
    pub total_revenue: i64,
    pub completed_count: i64,
    pub cash_amount: i64,
    pub digital_amount: i64,
    pub avg_per_job: i64,
    pub pending_count: i64,
}

/// Staff earnings for a given period.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaffEarning {
    pub staff_id: Uuid,
    pub staff_name: String,
    pub job_count: i64,
    pub total_revenue: i64,
    pub total_commission: i64,
}

pub struct PaymentService<R> {
    repo: R,
}

impl<R: PaymentRepository> PaymentService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn create_payment(
        &self,
        tenant_id: Uuid,
        input: CreatePaymentInput,
    ) -> Result<Payment, AppError> {
        if input.final_amount < 0 {
            return Err(PaymentError::InvalidAmount {
                message: "Final amount cannot be negative".to_string(),
            }
            .into());
        }

        if input.base_price < 0 {
            return Err(PaymentError::InvalidAmount {
                message: "Base price cannot be negative".to_string(),
            }
            .into());
        }

        if input.discount_amount < 0 {
            return Err(PaymentError::InvalidAmount {
                message: "Discount amount cannot be negative".to_string(),
            }
            .into());
        }

        let payment = self
            .repo
            .create(tenant_id, &input)
            .await
            .map_err(AppError::Internal)?;

        Ok(payment)
    }

    pub async fn complete_payment(
        &self,
        tenant_id: Uuid,
        id: Uuid,
        verified_by: Option<Uuid>,
    ) -> Result<Payment, AppError> {
        let existing = self
            .repo
            .get_by_id(tenant_id, id)
            .await
            .map_err(AppError::Internal)?
            .ok_or(PaymentError::NotFound)?;

        if existing.payment_status == crate::domain::PaymentStatus::Completed {
            return Err(PaymentError::AlreadyPaid.into());
        }

        if existing.payment_status == crate::domain::PaymentStatus::Refunded {
            return Err(PaymentError::InvalidStatus {
                from: "refunded".to_string(),
                to: "completed".to_string(),
            }
            .into());
        }

        let payment = self
            .repo
            .mark_completed(tenant_id, id, verified_by)
            .await
            .map_err(AppError::Internal)?;

        Ok(payment)
    }

    pub async fn get_payment(&self, tenant_id: Uuid, id: Uuid) -> Result<Payment, AppError> {
        let payment = self
            .repo
            .get_by_id(tenant_id, id)
            .await
            .map_err(AppError::Internal)?
            .ok_or(PaymentError::NotFound)?;

        Ok(payment)
    }

    pub async fn list_by_location(
        &self,
        tenant_id: Uuid,
        location_id: Uuid,
        date: NaiveDate,
    ) -> Result<Vec<Payment>, AppError> {
        self.repo
            .list_by_location(tenant_id, location_id, date)
            .await
            .map_err(AppError::Internal)
    }

    pub async fn daily_revenue(
        &self,
        tenant_id: Uuid,
        location_id: Uuid,
        date: NaiveDate,
    ) -> Result<DailyRevenueSummary, AppError> {
        self.repo
            .daily_revenue(tenant_id, location_id, date)
            .await
            .map_err(AppError::Internal)
    }

    pub async fn staff_earnings(
        &self,
        tenant_id: Uuid,
        staff_id: Uuid,
        from: NaiveDate,
        to: NaiveDate,
    ) -> Result<Vec<StaffEarning>, AppError> {
        self.repo
            .staff_earnings(tenant_id, staff_id, from, to)
            .await
            .map_err(AppError::Internal)
    }
}
