use chrono::{NaiveDate, Utc};
use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::application::{
    CreatePaymentInput, DailyRevenueSummary, PaymentRepository, StaffEarning,
};
use crate::domain::{Payment, PaymentMethod, PaymentStatus};

pub struct PgPaymentRepository {
    pool: PgPool,
}

impl PgPaymentRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

const PAYMENT_COLS: &str = "id, tenant_id, location_id, queue_entry_id, booking_id, \
    customer_name, customer_phone, service_id, service_name, base_price, discount_amount, \
    final_amount, promotion_id, payment_method, payment_status, paid_at, collected_by, \
    verified_by, staff_id, assistant_id, notes, created_at, updated_at";

fn row_to_payment(row: &sqlx::postgres::PgRow) -> Payment {
    Payment {
        id: row.get("id"),
        tenant_id: row.get("tenant_id"),
        location_id: row.get("location_id"),
        queue_entry_id: row.get("queue_entry_id"),
        booking_id: row.get("booking_id"),
        customer_name: row.get("customer_name"),
        customer_phone: row.get("customer_phone"),
        service_id: row.get("service_id"),
        service_name: row.get("service_name"),
        base_price: row.get("base_price"),
        discount_amount: row.get("discount_amount"),
        final_amount: row.get("final_amount"),
        promotion_id: row.get("promotion_id"),
        payment_method: PaymentMethod::from_str(row.get::<String, _>("payment_method").as_str())
            .unwrap_or(PaymentMethod::Cash),
        payment_status: PaymentStatus::from_str(row.get::<String, _>("payment_status").as_str())
            .unwrap_or(PaymentStatus::Pending),
        paid_at: row.get("paid_at"),
        collected_by: row.get("collected_by"),
        verified_by: row.get("verified_by"),
        staff_id: row.get("staff_id"),
        assistant_id: row.get("assistant_id"),
        notes: row.get("notes"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }
}

impl PaymentRepository for PgPaymentRepository {
    async fn create(&self, tenant_id: Uuid, input: &CreatePaymentInput) -> anyhow::Result<Payment> {
        let id = Uuid::now_v7();
        let now = Utc::now();
        let method = PaymentMethod::from_str(&input.payment_method).unwrap_or(PaymentMethod::Cash);
        let status = PaymentStatus::Pending;

        let q = format!(
            "INSERT INTO payments ({PAYMENT_COLS}) \
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23) \
             RETURNING {PAYMENT_COLS}"
        );

        let row = sqlx::query(&q)
            .bind(id)
            .bind(tenant_id)
            .bind(input.location_id)
            .bind(input.queue_entry_id)
            .bind(input.booking_id)
            .bind(&input.customer_name)
            .bind(&input.customer_phone)
            .bind(input.service_id)
            .bind(&input.service_name)
            .bind(input.base_price)
            .bind(input.discount_amount)
            .bind(input.final_amount)
            .bind(input.promotion_id)
            .bind(method.as_str())
            .bind(status.as_str())
            .bind(None::<chrono::DateTime<Utc>>) // paid_at
            .bind(input.collected_by)
            .bind(None::<Uuid>) // verified_by
            .bind(input.staff_id)
            .bind(input.assistant_id)
            .bind(&input.notes)
            .bind(now) // created_at
            .bind(now) // updated_at
            .fetch_one(&self.pool)
            .await?;

        Ok(row_to_payment(&row))
    }

    async fn get_by_id(&self, tenant_id: Uuid, id: Uuid) -> anyhow::Result<Option<Payment>> {
        let q = format!("SELECT {PAYMENT_COLS} FROM payments WHERE id = $1 AND tenant_id = $2");
        let row = sqlx::query(&q)
            .bind(id)
            .bind(tenant_id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(row.as_ref().map(row_to_payment))
    }

    async fn list_by_location(
        &self,
        tenant_id: Uuid,
        location_id: Uuid,
        date: NaiveDate,
    ) -> anyhow::Result<Vec<Payment>> {
        let q = format!(
            "SELECT {PAYMENT_COLS} FROM payments \
             WHERE tenant_id = $1 AND location_id = $2 AND created_at::date = $3 \
             ORDER BY created_at DESC"
        );
        let rows = sqlx::query(&q)
            .bind(tenant_id)
            .bind(location_id)
            .bind(date)
            .fetch_all(&self.pool)
            .await?;
        Ok(rows.iter().map(row_to_payment).collect())
    }

    async fn mark_completed(
        &self,
        tenant_id: Uuid,
        id: Uuid,
        verified_by: Option<Uuid>,
    ) -> anyhow::Result<Payment> {
        let now = Utc::now();
        let q = format!(
            "UPDATE payments SET payment_status = $1, paid_at = $2, verified_by = $3, updated_at = $4 \
             WHERE id = $5 AND tenant_id = $6 \
             RETURNING {PAYMENT_COLS}"
        );
        let row = sqlx::query(&q)
            .bind(PaymentStatus::Completed.as_str())
            .bind(now)
            .bind(verified_by)
            .bind(now)
            .bind(id)
            .bind(tenant_id)
            .fetch_one(&self.pool)
            .await?;
        Ok(row_to_payment(&row))
    }

    async fn daily_revenue(
        &self,
        tenant_id: Uuid,
        location_id: Uuid,
        date: NaiveDate,
    ) -> anyhow::Result<DailyRevenueSummary> {
        let row = sqlx::query(
            "SELECT \
                COALESCE(SUM(CASE WHEN payment_status = 'completed' THEN final_amount ELSE 0 END), 0) as total_revenue, \
                COUNT(CASE WHEN payment_status = 'completed' THEN 1 END) as completed_count, \
                COALESCE(SUM(CASE WHEN payment_status = 'completed' AND payment_method = 'cash' THEN final_amount ELSE 0 END), 0) as cash_amount, \
                COALESCE(SUM(CASE WHEN payment_status = 'completed' AND payment_method != 'cash' THEN final_amount ELSE 0 END), 0) as digital_amount, \
                COUNT(CASE WHEN payment_status = 'pending' THEN 1 END) as pending_count \
             FROM payments \
             WHERE tenant_id = $1 AND location_id = $2 AND created_at::date = $3",
        )
        .bind(tenant_id)
        .bind(location_id)
        .bind(date)
        .fetch_one(&self.pool)
        .await?;

        let total_revenue: i64 = row.get("total_revenue");
        let completed_count: i64 = row.get("completed_count");
        let cash_amount: i64 = row.get("cash_amount");
        let digital_amount: i64 = row.get("digital_amount");
        let pending_count: i64 = row.get("pending_count");
        let avg_per_job = if completed_count > 0 {
            total_revenue / completed_count
        } else {
            0
        };

        Ok(DailyRevenueSummary {
            total_revenue,
            completed_count,
            cash_amount,
            digital_amount,
            avg_per_job,
            pending_count,
        })
    }

    async fn staff_earnings(
        &self,
        tenant_id: Uuid,
        staff_id: Uuid,
        from: NaiveDate,
        to: NaiveDate,
    ) -> anyhow::Result<Vec<StaffEarning>> {
        let rows = sqlx::query(
            "SELECT \
                staff_id, \
                MIN(customer_name) as staff_name, \
                COUNT(*) as job_count, \
                COALESCE(SUM(final_amount), 0) as total_revenue, \
                0::bigint as total_commission \
             FROM payments \
             WHERE tenant_id = $1 AND staff_id = $2 \
                AND payment_status = 'completed' \
                AND created_at::date >= $3 AND created_at::date <= $4 \
             GROUP BY staff_id",
        )
        .bind(tenant_id)
        .bind(staff_id)
        .bind(from)
        .bind(to)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .iter()
            .map(|row| StaffEarning {
                staff_id: row.get("staff_id"),
                staff_name: row.get("staff_name"),
                job_count: row.get("job_count"),
                total_revenue: row.get("total_revenue"),
                total_commission: row.get("total_commission"),
            })
            .collect())
    }
}
