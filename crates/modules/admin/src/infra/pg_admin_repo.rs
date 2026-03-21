use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::application::AdminRepository;
use crate::domain::{AdminAction, AdminLocationView, PlatformMetrics, SubscriptionTier};

pub struct PgAdminRepository {
    pool: PgPool,
}

impl PgAdminRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl AdminRepository for PgAdminRepository {
    async fn list_locations(
        &self,
        status_filter: Option<&str>,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<Vec<AdminLocationView>> {
        let rows = if let Some(status) = status_filter {
            sqlx::query(
                r#"SELECT l.id, l.tenant_id, l.name, l.address, l.city, l.district,
                    l.status, l.bay_count, l.queue_mode, l.created_at,
                    COALESCE(AVG(r.rating), 0)::FLOAT8 as avg_rating,
                    COUNT(r.id)::BIGINT as review_count
                FROM locations l
                LEFT JOIN reviews r ON r.location_id = l.id
                WHERE l.deleted_at IS NULL AND l.status = $1
                GROUP BY l.id
                ORDER BY l.created_at DESC
                LIMIT $2 OFFSET $3"#,
            )
            .bind(status)
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool)
            .await?
        } else {
            sqlx::query(
                r#"SELECT l.id, l.tenant_id, l.name, l.address, l.city, l.district,
                    l.status, l.bay_count, l.queue_mode, l.created_at,
                    COALESCE(AVG(r.rating), 0)::FLOAT8 as avg_rating,
                    COUNT(r.id)::BIGINT as review_count
                FROM locations l
                LEFT JOIN reviews r ON r.location_id = l.id
                WHERE l.deleted_at IS NULL
                GROUP BY l.id
                ORDER BY l.created_at DESC
                LIMIT $1 OFFSET $2"#,
            )
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool)
            .await?
        };

        Ok(rows
            .iter()
            .map(|r| AdminLocationView {
                id: r.get("id"),
                tenant_id: r.get("tenant_id"),
                name: r.get("name"),
                address: r.get("address"),
                city: r.get("city"),
                district: r.get("district"),
                status: r.get("status"),
                bay_count: r.get("bay_count"),
                queue_mode: r.get("queue_mode"),
                created_at: r.get("created_at"),
                review_count: r.get::<i64, _>("review_count"),
                avg_rating: r.get::<f64, _>("avg_rating"),
            })
            .collect())
    }

    async fn get_location(&self, id: Uuid) -> anyhow::Result<Option<AdminLocationView>> {
        let row = sqlx::query(
            r#"SELECT l.id, l.tenant_id, l.name, l.address, l.city, l.district,
                l.status, l.bay_count, l.queue_mode, l.created_at,
                COALESCE(AVG(r.rating), 0)::FLOAT8 as avg_rating,
                COUNT(r.id)::BIGINT as review_count
            FROM locations l
            LEFT JOIN reviews r ON r.location_id = l.id
            WHERE l.id = $1 AND l.deleted_at IS NULL
            GROUP BY l.id"#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| AdminLocationView {
            id: r.get("id"),
            tenant_id: r.get("tenant_id"),
            name: r.get("name"),
            address: r.get("address"),
            city: r.get("city"),
            district: r.get("district"),
            status: r.get("status"),
            bay_count: r.get("bay_count"),
            queue_mode: r.get("queue_mode"),
            created_at: r.get("created_at"),
            review_count: r.get::<i64, _>("review_count"),
            avg_rating: r.get::<f64, _>("avg_rating"),
        }))
    }

    async fn update_location_status(&self, id: Uuid, status: &str) -> anyhow::Result<()> {
        sqlx::query("UPDATE locations SET status = $1, updated_at = now() WHERE id = $2")
            .bind(status)
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn platform_metrics(&self) -> anyhow::Result<PlatformMetrics> {
        let row = sqlx::query(
            r#"SELECT
                COUNT(*)::BIGINT as total_locations,
                COUNT(*) FILTER (WHERE status = 'active')::BIGINT as active_locations,
                COUNT(*) FILTER (WHERE status = 'pending')::BIGINT as pending_locations,
                COUNT(*) FILTER (WHERE status = 'suspended')::BIGINT as suspended_locations
            FROM locations
            WHERE deleted_at IS NULL"#,
        )
        .fetch_one(&self.pool)
        .await?;

        let tenants_row = sqlx::query(
            "SELECT COUNT(DISTINCT id)::BIGINT as total_tenants FROM tenants WHERE deleted_at IS NULL",
        )
        .fetch_one(&self.pool)
        .await?;

        let revenue_row = sqlx::query(
            r#"SELECT COALESCE(SUM(amount), 0)::BIGINT as total_revenue_today
            FROM payments
            WHERE status = 'completed' AND created_at::date = CURRENT_DATE"#,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(PlatformMetrics {
            total_locations: row.get::<i64, _>("total_locations"),
            active_locations: row.get::<i64, _>("active_locations"),
            pending_locations: row.get::<i64, _>("pending_locations"),
            suspended_locations: row.get::<i64, _>("suspended_locations"),
            total_tenants: tenants_row.get::<i64, _>("total_tenants"),
            total_revenue_today: revenue_row.get::<i64, _>("total_revenue_today"),
        })
    }

    async fn log_action(&self, action: &AdminAction) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO admin_actions (id, admin_user_id, action_type, target_type, target_id, reason, metadata, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"#,
        )
        .bind(action.id)
        .bind(action.admin_user_id)
        .bind(&action.action_type)
        .bind(&action.target_type)
        .bind(action.target_id)
        .bind(&action.reason)
        .bind(&action.metadata)
        .bind(action.created_at)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn list_actions(&self, limit: i64, offset: i64) -> anyhow::Result<Vec<AdminAction>> {
        let rows = sqlx::query(
            r#"SELECT id, admin_user_id, action_type, target_type, target_id, reason, metadata, created_at
            FROM admin_actions
            ORDER BY created_at DESC
            LIMIT $1 OFFSET $2"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .iter()
            .map(|r| AdminAction {
                id: r.get("id"),
                admin_user_id: r.get("admin_user_id"),
                action_type: r.get("action_type"),
                target_type: r.get("target_type"),
                target_id: r.get("target_id"),
                reason: r.get("reason"),
                metadata: r.get("metadata"),
                created_at: r.get("created_at"),
            })
            .collect())
    }

    async fn list_tiers(&self) -> anyhow::Result<Vec<SubscriptionTier>> {
        let rows = sqlx::query(
            r#"SELECT id, name, display_name, max_locations, max_staff, features
            FROM subscription_tiers
            ORDER BY name"#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .iter()
            .map(|r| SubscriptionTier {
                id: r.get("id"),
                name: r.get("name"),
                display_name: r.get("display_name"),
                max_locations: r.get("max_locations"),
                max_staff: r.get("max_staff"),
                features: r.get("features"),
            })
            .collect())
    }
}
