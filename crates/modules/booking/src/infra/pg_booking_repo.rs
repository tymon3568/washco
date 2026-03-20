use chrono::{NaiveDate, NaiveTime};
use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::application::BookingRepository;
use crate::domain::{Booking, BookingStatus};

pub struct PgBookingRepository {
    pool: PgPool,
}

impl PgBookingRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

fn row_to_booking(row: &sqlx::postgres::PgRow) -> Booking {
    Booking {
        id: row.get("id"),
        tenant_id: row.get("tenant_id"),
        location_id: row.get("location_id"),
        service_id: row.get("service_id"),
        customer_name: row.get("customer_name"),
        customer_phone: row.get("customer_phone"),
        vehicle_type: row.get("vehicle_type"),
        booking_date: row.get("booking_date"),
        time_slot: row.get("time_slot"),
        status: BookingStatus::from_str(row.get::<String, _>("status").as_str()),
        notes: row.get("notes"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
        cancelled_at: row.get("cancelled_at"),
    }
}

const BOOKING_COLS: &str = "id, tenant_id, location_id, service_id, customer_name, customer_phone, vehicle_type, booking_date, time_slot, status, notes, created_at, updated_at, cancelled_at";

impl BookingRepository for PgBookingRepository {
    async fn create(&self, booking: &Booking) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO bookings
               (id, tenant_id, location_id, service_id, customer_name, customer_phone,
                vehicle_type, booking_date, time_slot, status, notes, created_at, updated_at, cancelled_at)
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)"#,
        )
        .bind(booking.id)
        .bind(booking.tenant_id)
        .bind(booking.location_id)
        .bind(booking.service_id)
        .bind(&booking.customer_name)
        .bind(&booking.customer_phone)
        .bind(&booking.vehicle_type)
        .bind(booking.booking_date)
        .bind(booking.time_slot)
        .bind(booking.status.as_str())
        .bind(&booking.notes)
        .bind(booking.created_at)
        .bind(booking.updated_at)
        .bind(booking.cancelled_at)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn find_by_id(&self, tenant_id: Uuid, id: Uuid) -> anyhow::Result<Option<Booking>> {
        let q = format!(
            "SELECT {BOOKING_COLS} FROM bookings WHERE id = $1 AND tenant_id = $2"
        );
        let row = sqlx::query(&q)
            .bind(id)
            .bind(tenant_id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(row.as_ref().map(row_to_booking))
    }

    async fn update_status(&self, booking: &Booking) -> anyhow::Result<()> {
        sqlx::query(
            "UPDATE bookings SET status = $1, updated_at = $2, cancelled_at = $3 WHERE id = $4 AND tenant_id = $5",
        )
        .bind(booking.status.as_str())
        .bind(booking.updated_at)
        .bind(booking.cancelled_at)
        .bind(booking.id)
        .bind(booking.tenant_id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn list_by_location(
        &self,
        tenant_id: Uuid,
        location_id: Uuid,
        date: NaiveDate,
    ) -> anyhow::Result<Vec<Booking>> {
        let q = format!(
            "SELECT {BOOKING_COLS} FROM bookings
             WHERE tenant_id = $1 AND location_id = $2 AND booking_date = $3
             ORDER BY time_slot ASC"
        );
        let rows = sqlx::query(&q)
            .bind(tenant_id)
            .bind(location_id)
            .bind(date)
            .fetch_all(&self.pool)
            .await?;
        Ok(rows.iter().map(row_to_booking).collect())
    }

    async fn list_by_phone(
        &self,
        phone: &str,
        date: NaiveDate,
    ) -> anyhow::Result<Vec<Booking>> {
        let q = format!(
            "SELECT {BOOKING_COLS} FROM bookings
             WHERE customer_phone = $1 AND booking_date = $2
             ORDER BY time_slot ASC"
        );
        let rows = sqlx::query(&q)
            .bind(phone)
            .bind(date)
            .fetch_all(&self.pool)
            .await?;
        Ok(rows.iter().map(row_to_booking).collect())
    }

    async fn count_at_slot(
        &self,
        tenant_id: Uuid,
        location_id: Uuid,
        date: NaiveDate,
        time: NaiveTime,
    ) -> anyhow::Result<i64> {
        let row = sqlx::query(
            "SELECT COUNT(*) as cnt FROM bookings WHERE tenant_id = $1 AND location_id = $2 AND booking_date = $3 AND time_slot = $4 AND status NOT IN ('cancelled')",
        )
        .bind(tenant_id)
        .bind(location_id)
        .bind(date)
        .bind(time)
        .fetch_one(&self.pool)
        .await?;
        Ok(row.get::<i64, _>("cnt"))
    }
}
