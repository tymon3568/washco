use chrono::{DateTime, Utc};
use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::application::QueueRepository;
use crate::domain::{QueueEntry, QueueStatus, WaitEstimate};

pub struct PgQueueRepository {
    pool: PgPool,
}

impl PgQueueRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

fn row_to_entry(row: &sqlx::postgres::PgRow) -> QueueEntry {
    QueueEntry {
        id: row.get("id"),
        tenant_id: row.get("tenant_id"),
        location_id: row.get("location_id"),
        queue_number: row.get("queue_number"),
        customer_name: row.get("customer_name"),
        customer_phone: row.get("customer_phone"),
        vehicle_type: row.get("vehicle_type"),
        service_id: row.get("service_id"),
        service_name: row.get("service_name"),
        bay_id: row.get("bay_id"),
        status: QueueStatus::from_str(row.get::<String, _>("status").as_str()),
        joined_at: row.get("joined_at"),
        started_at: row.get("started_at"),
        completed_at: row.get("completed_at"),
    }
}

const ENTRY_COLS: &str = "id, tenant_id, location_id, queue_number, customer_name, customer_phone, vehicle_type, service_id, service_name, bay_id, status, joined_at, started_at, completed_at";

impl QueueRepository for PgQueueRepository {
    async fn find_active_by_location(
        &self,
        tenant_id: Uuid,
        location_id: Uuid,
    ) -> anyhow::Result<Vec<QueueEntry>> {
        let q = format!(
            "SELECT {ENTRY_COLS} FROM queue_entries
             WHERE tenant_id = $1 AND location_id = $2 AND status IN ('waiting', 'in_progress')
             ORDER BY joined_at ASC"
        );
        let rows = sqlx::query(&q)
            .bind(tenant_id)
            .bind(location_id)
            .fetch_all(&self.pool)
            .await?;
        Ok(rows.iter().map(row_to_entry).collect())
    }

    async fn find_by_id(&self, tenant_id: Uuid, id: Uuid) -> anyhow::Result<Option<QueueEntry>> {
        let q = format!("SELECT {ENTRY_COLS} FROM queue_entries WHERE id = $1 AND tenant_id = $2");
        let row = sqlx::query(&q)
            .bind(id)
            .bind(tenant_id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(row.as_ref().map(row_to_entry))
    }

    async fn create(&self, entry: &QueueEntry) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO queue_entries
               (id, tenant_id, location_id, queue_number, customer_name, customer_phone,
                vehicle_type, service_id, service_name, bay_id, status, joined_at, started_at, completed_at)
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)"#,
        )
        .bind(entry.id)
        .bind(entry.tenant_id)
        .bind(entry.location_id)
        .bind(entry.queue_number)
        .bind(&entry.customer_name)
        .bind(&entry.customer_phone)
        .bind(&entry.vehicle_type)
        .bind(entry.service_id)
        .bind(&entry.service_name)
        .bind(entry.bay_id)
        .bind(entry.status.as_str())
        .bind(entry.joined_at)
        .bind(entry.started_at)
        .bind(entry.completed_at)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn update_status(&self, entry: &QueueEntry) -> anyhow::Result<()> {
        sqlx::query(
            "UPDATE queue_entries SET status = $1, bay_id = $2, started_at = $3, completed_at = $4 WHERE id = $5 AND tenant_id = $6",
        )
        .bind(entry.status.as_str())
        .bind(entry.bay_id)
        .bind(entry.started_at)
        .bind(entry.completed_at)
        .bind(entry.id)
        .bind(entry.tenant_id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn next_queue_number(&self, tenant_id: Uuid, location_id: Uuid) -> anyhow::Result<i32> {
        let row = sqlx::query(
            "SELECT COALESCE(MAX(queue_number), 0) + 1 as next_num FROM queue_entries WHERE tenant_id = $1 AND location_id = $2 AND joined_at::date = CURRENT_DATE",
        )
        .bind(tenant_id)
        .bind(location_id)
        .fetch_one(&self.pool)
        .await?;
        Ok(row.get::<i32, _>("next_num"))
    }

    async fn estimate_wait(
        &self,
        tenant_id: Uuid,
        location_id: Uuid,
        entry_joined_at: DateTime<Utc>,
    ) -> anyhow::Result<WaitEstimate> {
        let row = sqlx::query(
            "SELECT COUNT(*) as cnt FROM queue_entries WHERE tenant_id = $1 AND location_id = $2 AND status = 'waiting' AND joined_at < $3",
        )
        .bind(tenant_id)
        .bind(location_id)
        .bind(entry_joined_at)
        .fetch_one(&self.pool)
        .await?;
        let position: i64 = row.get("cnt");
        let estimated_minutes = (position * 15) / 1; // 15 min avg, 1 bay default
        Ok(WaitEstimate {
            position: position as i32,
            estimated_minutes: estimated_minutes as i32,
        })
    }

    async fn completed_today_count(&self, tenant_id: Uuid, location_id: Uuid) -> anyhow::Result<i64> {
        let row = sqlx::query(
            "SELECT COUNT(*) as cnt FROM queue_entries WHERE tenant_id = $1 AND location_id = $2 AND status = 'completed' AND completed_at::date = CURRENT_DATE",
        )
        .bind(tenant_id)
        .bind(location_id)
        .fetch_one(&self.pool)
        .await?;
        Ok(row.get::<i64, _>("cnt"))
    }
}
