use chrono::NaiveDate;
use sqlx::{PgPool, Row};
use uuid::Uuid;
use washco_shared::money::Money;

use crate::application::AnalyticsRepository;
use crate::domain::{BayUtilization, DailySummary, LocationComparison, PeriodSummary, ServiceMetric, TrendDataPoint};

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

    async fn trend(
        &self,
        tenant_id: Uuid,
        location_id: Uuid,
        from: NaiveDate,
        to: NaiveDate,
    ) -> anyhow::Result<Vec<TrendDataPoint>> {
        let rows = sqlx::query(
            r#"SELECT
                d.date,
                COALESCE(SUM(s.base_price), 0)::BIGINT as revenue,
                COALESCE(COUNT(*) FILTER (WHERE qe.status = 'completed'), 0)::BIGINT as completed_jobs,
                COALESCE(COUNT(*), 0)::BIGINT as walk_ins,
                COALESCE(COUNT(*) FILTER (WHERE qe.status = 'cancelled'), 0)::BIGINT as cancellations,
                COALESCE(AVG(EXTRACT(EPOCH FROM (qe.started_at - qe.joined_at)) / 60)
                    FILTER (WHERE qe.started_at IS NOT NULL), 0)::FLOAT8 as average_wait_minutes
            FROM generate_series($3::date, $4::date, '1 day'::interval) AS d(date)
            LEFT JOIN queue_entries qe
                ON qe.joined_at::date = d.date
                AND qe.location_id = $1
                AND qe.tenant_id = $2
            LEFT JOIN services s
                ON s.id = qe.service_id AND s.tenant_id = qe.tenant_id
            GROUP BY d.date
            ORDER BY d.date"#,
        )
        .bind(location_id)
        .bind(tenant_id)
        .bind(from)
        .bind(to)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .iter()
            .map(|r| TrendDataPoint {
                date: r.get("date"),
                revenue: Money::new(r.get::<i64, _>("revenue")),
                completed_jobs: r.get::<i64, _>("completed_jobs") as i32,
                walk_ins: r.get::<i64, _>("walk_ins") as i32,
                cancellations: r.get::<i64, _>("cancellations") as i32,
                average_wait_minutes: r.get::<f64, _>("average_wait_minutes"),
            })
            .collect())
    }

    async fn period_summary(
        &self,
        tenant_id: Uuid,
        location_id: Uuid,
        from: NaiveDate,
        to: NaiveDate,
    ) -> anyhow::Result<PeriodSummary> {
        let row = sqlx::query(
            r#"SELECT
                COALESCE(SUM(s.base_price), 0)::BIGINT as total_revenue,
                COALESCE(COUNT(*) FILTER (WHERE qe.status = 'completed'), 0)::BIGINT as total_completed,
                COALESCE(COUNT(*), 0)::BIGINT as total_walk_ins,
                COALESCE(COUNT(*) FILTER (WHERE qe.status = 'cancelled'), 0)::BIGINT as total_cancellations,
                COALESCE(AVG(EXTRACT(EPOCH FROM (qe.started_at - qe.joined_at)) / 60)
                    FILTER (WHERE qe.started_at IS NOT NULL), 0)::FLOAT8 as average_wait_minutes
            FROM queue_entries qe
            LEFT JOIN services s ON s.id = qe.service_id AND s.tenant_id = qe.tenant_id
            WHERE qe.location_id = $1 AND qe.tenant_id = $2
                AND qe.joined_at::date >= $3 AND qe.joined_at::date <= $4"#,
        )
        .bind(location_id)
        .bind(tenant_id)
        .bind(from)
        .bind(to)
        .fetch_one(&self.pool)
        .await?;

        let busiest = sqlx::query(
            r#"SELECT qe.joined_at::date as day, COUNT(*) as cnt,
                COALESCE(SUM(s.base_price), 0)::BIGINT as day_revenue
            FROM queue_entries qe
            LEFT JOIN services s ON s.id = qe.service_id AND s.tenant_id = qe.tenant_id
            WHERE qe.location_id = $1 AND qe.tenant_id = $2
                AND qe.joined_at::date >= $3 AND qe.joined_at::date <= $4
            GROUP BY qe.joined_at::date
            ORDER BY cnt DESC, day_revenue DESC
            LIMIT 1"#,
        )
        .bind(location_id)
        .bind(tenant_id)
        .bind(from)
        .bind(to)
        .fetch_optional(&self.pool)
        .await?;

        let (busiest_day, peak_revenue) = match busiest {
            Some(r) => (
                Some(r.get::<NaiveDate, _>("day")),
                Money::new(r.get::<i64, _>("day_revenue")),
            ),
            None => (None, Money::new(0)),
        };

        Ok(PeriodSummary {
            location_id,
            from,
            to,
            total_revenue: Money::new(row.get::<i64, _>("total_revenue")),
            total_completed: row.get::<i64, _>("total_completed") as i32,
            total_walk_ins: row.get::<i64, _>("total_walk_ins") as i32,
            total_cancellations: row.get::<i64, _>("total_cancellations") as i32,
            average_wait_minutes: row.get::<f64, _>("average_wait_minutes"),
            busiest_day,
            peak_revenue,
        })
    }

    async fn compare_locations(
        &self,
        tenant_id: Uuid,
        location_ids: &[Uuid],
        from: NaiveDate,
        to: NaiveDate,
    ) -> anyhow::Result<Vec<LocationComparison>> {
        let rows = sqlx::query(
            r#"SELECT
                l.id as location_id,
                l.name as location_name,
                COALESCE(SUM(s.base_price), 0)::BIGINT as total_revenue,
                COALESCE(COUNT(*) FILTER (WHERE qe.status = 'completed'), 0)::BIGINT as total_completed,
                COALESCE(AVG(EXTRACT(EPOCH FROM (qe.started_at - qe.joined_at)) / 60)
                    FILTER (WHERE qe.started_at IS NOT NULL), 0)::FLOAT8 as average_wait_minutes
            FROM locations l
            LEFT JOIN queue_entries qe
                ON qe.location_id = l.id AND qe.tenant_id = l.tenant_id
                AND qe.joined_at::date >= $3 AND qe.joined_at::date <= $4
            LEFT JOIN services s
                ON s.id = qe.service_id AND s.tenant_id = qe.tenant_id
            WHERE l.id = ANY($1) AND l.tenant_id = $2
            GROUP BY l.id, l.name
            ORDER BY total_revenue DESC"#,
        )
        .bind(location_ids)
        .bind(tenant_id)
        .bind(from)
        .bind(to)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .iter()
            .map(|r| LocationComparison {
                location_id: r.get("location_id"),
                location_name: r.get("location_name"),
                total_revenue: Money::new(r.get::<i64, _>("total_revenue")),
                total_completed: r.get::<i64, _>("total_completed") as i32,
                average_wait_minutes: r.get::<f64, _>("average_wait_minutes"),
            })
            .collect())
    }
}
