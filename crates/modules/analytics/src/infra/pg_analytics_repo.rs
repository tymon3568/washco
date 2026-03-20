use chrono::NaiveDate;
use sqlx::{PgPool, Row};
use uuid::Uuid;
use washco_shared::money::Money;

use crate::application::AnalyticsRepository;
use crate::domain::{BayUtilization, DailySummary, ServiceMetric};

pub struct PgAnalyticsRepository {
    pool: PgPool,
}

impl PgAnalyticsRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl AnalyticsRepository for PgAnalyticsRepository {
    async fn daily_summary(
        &self,
        tenant_id: Uuid,
        location_id: Uuid,
        date: NaiveDate,
    ) -> anyhow::Result<DailySummary> {
        let row = sqlx::query(
            r#"SELECT
                COALESCE(SUM(s.base_price), 0)::BIGINT as total_revenue,
                COALESCE(COUNT(*) FILTER (WHERE qe.status = 'completed'), 0)::BIGINT as completed_jobs,
                COALESCE(COUNT(*), 0)::BIGINT as walk_ins,
                COALESCE(AVG(EXTRACT(EPOCH FROM (qe.started_at - qe.joined_at)) / 60) FILTER (WHERE qe.started_at IS NOT NULL), 0)::FLOAT8 as average_wait_minutes,
                COALESCE(COUNT(*) FILTER (WHERE qe.status = 'cancelled'), 0)::BIGINT as cancellations
            FROM queue_entries qe
            LEFT JOIN services s ON s.id = qe.service_id AND s.tenant_id = qe.tenant_id
            WHERE qe.location_id = $1 AND qe.tenant_id = $2 AND qe.joined_at::date = $3"#,
        )
        .bind(location_id)
        .bind(tenant_id)
        .bind(date)
        .fetch_one(&self.pool)
        .await?;

        Ok(DailySummary {
            location_id,
            date,
            total_revenue: Money::new(row.get::<i64, _>("total_revenue")),
            completed_jobs: row.get::<i64, _>("completed_jobs") as i32,
            walk_ins: row.get::<i64, _>("walk_ins") as i32,
            average_wait_minutes: row.get::<f64, _>("average_wait_minutes"),
            cancellations: row.get::<i64, _>("cancellations") as i32,
        })
    }

    async fn bay_utilization(
        &self,
        tenant_id: Uuid,
        location_id: Uuid,
        date: NaiveDate,
        bay_count: i32,
        operating_hours_minutes: i32,
    ) -> anyhow::Result<BayUtilization> {
        let row = sqlx::query(
            r#"SELECT COALESCE(
                SUM(EXTRACT(EPOCH FROM (qe.completed_at - qe.started_at)) / 60)
                FILTER (WHERE qe.completed_at IS NOT NULL AND qe.started_at IS NOT NULL), 0
            )::FLOAT8 as total_service_minutes
            FROM queue_entries qe
            WHERE qe.location_id = $1 AND qe.tenant_id = $2 AND qe.joined_at::date = $3"#,
        )
        .bind(location_id)
        .bind(tenant_id)
        .bind(date)
        .fetch_one(&self.pool)
        .await?;

        let total_service_minutes: f64 = row.get("total_service_minutes");
        let capacity = (bay_count as f64) * (operating_hours_minutes as f64);
        let utilization_percent = if capacity > 0.0 {
            (total_service_minutes / capacity) * 100.0
        } else {
            0.0
        };

        Ok(BayUtilization {
            location_id,
            date,
            utilization_percent,
            total_service_minutes,
        })
    }

    async fn service_breakdown(
        &self,
        tenant_id: Uuid,
        location_id: Uuid,
        date: NaiveDate,
    ) -> anyhow::Result<Vec<ServiceMetric>> {
        let rows = sqlx::query(
            r#"SELECT
                s.name as service_name,
                COUNT(*)::BIGINT as count,
                COALESCE(SUM(s.base_price), 0)::BIGINT as revenue,
                COALESCE(AVG(EXTRACT(EPOCH FROM (qe.completed_at - qe.started_at)) / 60)
                    FILTER (WHERE qe.completed_at IS NOT NULL AND qe.started_at IS NOT NULL), 0)::FLOAT8 as average_duration_minutes
            FROM queue_entries qe
            JOIN services s ON s.id = qe.service_id AND s.tenant_id = qe.tenant_id
            WHERE qe.location_id = $1 AND qe.tenant_id = $2 AND qe.joined_at::date = $3
            GROUP BY s.name ORDER BY count DESC"#,
        )
        .bind(location_id)
        .bind(tenant_id)
        .bind(date)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .iter()
            .map(|r| ServiceMetric {
                service_name: r.get("service_name"),
                count: r.get::<i64, _>("count"),
                revenue: Money::new(r.get::<i64, _>("revenue")),
                average_duration_minutes: r.get::<f64, _>("average_duration_minutes"),
            })
            .collect())
    }
}
